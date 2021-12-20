mod aoc_env;
mod config;
mod generate_index;
mod input;
use anyhow::{anyhow, Context, Result};
use aoc_env::list_days;
use chrono::{self, Datelike};
use dialoguer::Confirm;
use std::{env, fs, process::Command};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "aoc")]
enum Opt {
    /// Store AOC session key in config file to download
    /// problem inputs later.
    Auth {
        /// session key for adventofcode.com
        session: Option<String>,
    },
    /// Download puzzle input for given day
    Input {
        /// year of the AOC problem
        #[structopt(long)]
        year: Option<u16>,

        /// day of the AOC problem
        #[structopt(long)]
        day: Option<u16>,

        /// write input data to file at this path,
        /// or STDOUT if output is "-"
        #[structopt(short, long, default_value = "<auto>")]
        output: String,

        /// override the configured session key
        #[structopt(long)]
        session: Option<String>,
    },
    /// Create new cargo project for AOC problem
    New {
        /// year of the AOC problem
        #[structopt(long)]
        year: Option<u16>,

        /// day of the AOC problem
        #[structopt(long)]
        day: Option<u16>,

        /// Open project in VS Code
        #[structopt(short, long)]
        open: bool,
    },
    /// Generate markdown index linking to problem subdirectories
    GenerateIndex {
        /// directory that contains AOC solutions with a directory structure of <year>/day<day>
        #[structopt(long, default_value = ".")]
        dir: String,

        /// Markdown file to update with a placeholder for the index
        #[structopt(long, default_value = "README.md")]
        file: String,
    },
}

fn next_day(year: u16) -> Result<u16> {
    let days = list_days(year).unwrap_or_default();
    Ok(match days.iter().max() {
        Some(day) => day + 1,
        None => 1,
    })
}

fn current_year() -> u16 {
    chrono::offset::Local::now().date().year() as u16
}

fn prompt_year() -> anyhow::Result<u16> {
    dialoguer::Input::new()
        .with_prompt("Enter year")
        .validate_with(|val: &String| match val.parse::<u16>() {
            Ok(year) => {
                if year >= 2014 && year <= 3000 {
                    Ok(())
                } else {
                    Err("Day must be 1-25")
                }
            }
            Err(_) => Err("Invalid number"),
        })
        .with_initial_text(format!("{}", current_year()))
        .interact_text()
        .map(|v| v.parse::<u16>().unwrap())
        .context("Unable to perform prompt for year")
}

fn prompt_day(next_day: u16) -> anyhow::Result<u16> {
    dialoguer::Input::new()
        .with_prompt("Enter day")
        .validate_with(|val: &String| match val.parse::<u16>() {
            Ok(day) => {
                if day >= 1 && day <= 25 {
                    Ok(())
                } else {
                    Err("Day must be 1-25")
                }
            }
            Err(_) => Err("Invalid number"),
        })
        .with_initial_text(format!("{}", next_day))
        .interact_text()
        .map(|v| v.parse::<u16>().unwrap())
        .context("Unable to perform prompt for year")
}

fn execute() -> anyhow::Result<()> {
    use Opt::*;
    match Opt::from_args() {
        Auth { session } => {
            let session = match session {
                Some(session) => session,
                None => {
                    eprintln!("{}", include_str!("session_key_instructions.txt"));
                    dialoguer::Input::new()
                        .with_prompt("Session key")
                        .interact_text()?
                }
            };
            if let Err(e) = input::download_input(2021, 1, session.clone()) {
                return Err(anyhow!("Enter session key: {}", e));
            }
            let config = config::Config {
                session_key: session,
            };
            config::write_config(&config)?;
            eprintln!("Config updated.");
        }
        Input {
            year,
            day,
            output,
            session,
        } => {
            let config = config::load_config();
            let (year, day) = match (year, day) {
                (Some(year), Some(day)) => Ok((year, day)),
                (None, None) => aoc_env::aoc_problem_in_cwd(),
                _ => Err(anyhow!("Please provide both --year and --day")),
            }?;

            eprintln!("Downloading input for year {} day {}", year, day);

            let session = match session {
                Some(value) => Ok(value),
                None => match config {
                    Some(config) => Ok(config.session_key),
                    None => Err(anyhow!("No session key configured")),
                },
            }?;

            let input_contents = input::download_input(year, day, session)
                .map_err(|e| anyhow!("Input download failed: {:?}", e))?;

            let output = if output == "<auto>" {
                if aoc_env::src_folder_exists()? {
                    "src/in.txt".to_string()
                } else {
                    "-".to_string()
                }
            } else {
                output
            };

            if output != "-" {
                eprintln!("Writing input file to {}", output);
                if fs::metadata(&output).is_ok() {
                    if !Confirm::new()
                        .with_prompt("File already exists, overwrite?")
                        .default(true)
                        .interact()?
                    {
                        return Ok(());
                    }
                }
                fs::write(output.clone(), input_contents.trim())?;
            } else {
                print!("{}", input_contents.trim());
            }
        }
        New { year, day, open } => {
            let (year, day) = match (year, day) {
                (Some(year), Some(day)) => (year, day),
                (None, Some(day)) => (current_year(), day),
                (Some(year), None) => (year, prompt_day(next_day(year)?)?),
                _ => {
                    let year = prompt_year()?;
                    (year, prompt_day(next_day(year)?)?)
                }
            };

            if day > 25 {
                return Err(anyhow!("No more days for year {}", year));
            }

            let project_dir = env::current_dir()?
                .join(format!("{}", year))
                .join(format!("day{:02}", day));
            eprintln!("Creating new project {}/day{:02}", year, day);

            if project_dir.exists() {
                return Err(anyhow!("Project already exists at {}/day{:02}", year, day));
            }

            fs::create_dir_all(project_dir.join("src"))
                .with_context(|| anyhow!("Unable to create project dir"))?;
            fs::create_dir_all(project_dir.join("temp"))
                .with_context(|| anyhow!("Unable to create project temp dir"))?;

            fs::write(
                project_dir.join("src").join("main.rs"),
                include_str!("tmpl/main.rs"),
            )?;
            fs::write(
                project_dir.join(".gitignore"),
                include_str!("tmpl/.gitignore-tmpl"),
            )?;
            fs::write(
                project_dir.join("Cargo.toml"),
                include_str!("tmpl/Cargo.toml"),
            )?;
            fs::write(project_dir.join("src").join("test.txt"), "")?;
            fs::write(project_dir.join("src").join("in.txt"), "")?;
            fs::write(
                project_dir.join("temp").join("test.js"),
                include_str!("tmpl/test.js"),
            )?;

            if open
                || Confirm::new()
                    .with_prompt("Open in VSCode?")
                    .default(true)
                    .interact()?
            {
                eprintln!("Project created. Attempting to open...");
                Command::new("/usr/local/bin/code")
                    .args([project_dir])
                    .status()?;
            } else {
                eprintln!(
                    "\nProject created. To open run:\n\ncode {}/day{}",
                    year, day
                );
            }
        }
        GenerateIndex { dir, file } => {
            generate_index::generate_index(dir, file)?;
        }
    };
    Ok(())
}

fn main() {
    if let Err(e) = execute() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
