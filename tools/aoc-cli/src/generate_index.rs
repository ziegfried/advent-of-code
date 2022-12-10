use anyhow::Context;
use std::fs;
use std::io;
use std::path::Path;

fn list_dir<S: AsRef<Path>>(dir: S) -> Result<Vec<String>, io::Error> {
    let mut entries = vec![];
    for child in fs::read_dir(dir)? {
        if let Some(name) = child?.path().file_name() {
            entries.push(String::from(name.to_str().unwrap()));
        }
    }
    Ok(entries)
}

pub fn generate_index(base_dir: String, filename: String) -> anyhow::Result<()> {
    let base_dir = fs::canonicalize(base_dir)?
        .into_os_string()
        .into_string()
        .unwrap();
    let mut contents = String::new();
    let mut years = vec![];
    for dir in list_dir(&base_dir)
        .with_context(|| format!("failed to list directories in {}", base_dir))?
    {
        if let Ok(year) = dir.parse::<u32>() {
            years.push(year);
        }
    }

    if years.is_empty() {
        return Err(anyhow::anyhow!(
            "did not find any year directories in {}",
            base_dir
        ));
    }

    years.sort_unstable();

    eprintln!("Found year directories {:?}", years);

    let latest_year = years.iter().max().unwrap();

    for year in years.iter().rev() {
        let mut days = vec![];
        for folder in list_dir(format!("{}/{}", &base_dir, year))? {
            if let Some(day_number) = folder.strip_prefix("day") {
                if let Ok(day) = day_number.parse::<u32>() {
                    days.push(day);
                }
            }
        }
        days.sort_unstable();

        if year == latest_year {
            contents.push_str(format!("Advent of Code {year}\n\n", year = year).as_str())
        } else {
            contents.push_str(
                format!(
                    "<details><summary>Advent of Code {year}</summary>\n<p>\n\n",
                    year = year
                )
                .as_str(),
            );
        }
        for day in days.iter().rev() {
            contents.push_str(
                format!(
                    " - [{year} Day {day}](./{year}/day{day:02}/src/main.rs)\n",
                    day = day,
                    year = year
                )
                .as_str(),
            );
        }
        if year == latest_year {
            contents.push('\n');
        } else {
            contents.push_str("\n</p>\n</details>\n");
        }
    }

    let readme_path = format!("{}/{}", &base_dir, &filename);
    eprintln!("Updating contents of {}", readme_path);
    let readme_contents =
        fs::read_to_string(&readme_path).context("failed to read markdown file")?;
    let start = readme_contents
        .find("<!-- INDEX-START -->")
        .context("placeholder <!-- INDEX-START --> not found in markdown file")?;
    let end = readme_contents
        .find("<!-- INDEX-END -->")
        .context("placeholder <!-- INDEX-END --> not found in markdown file")?;

    let mut result = String::new();
    result.push_str(&readme_contents[0..start]);
    result.push_str("<!-- INDEX-START -->\n");
    result.push_str(contents.as_str());
    result.push_str(&readme_contents[end..]);

    fs::write(&readme_path, result).context("failed to write markdown file")?;

    Ok(())
}
