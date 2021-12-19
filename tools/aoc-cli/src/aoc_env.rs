use anyhow::anyhow;
use anyhow::{Context, Result};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

fn os_to_str(os_str: &OsStr) -> String {
    String::from(os_str.to_str().unwrap())
}

fn dir_name(path: &Path) -> Result<String> {
    if let Some(file_name) = path.file_name() {
        if let Some(name) = file_name.to_str() {
            return Ok(String::from(name));
        }
    }
    return Err(anyhow!("Unable to extract file name from path"));
}

pub fn auto_detect_aoc_problem() -> Result<Option<(u16, u16)>> {
    let working_directory = env::current_dir()?;
    let day_dir = dir_name(&working_directory)?;
    if day_dir.starts_with("day") {
        if let Ok(day) = day_dir[3..].parse::<u16>() {
            if let Some(parent) = working_directory.parent() {
                let year_dir = dir_name(parent)?;
                if let Ok(year) = year_dir.parse::<u16>() {
                    return Ok(Some((year, day)));
                }
            }
        }
    }
    Ok(None)
}

pub fn aoc_problem_in_cwd() -> Result<(u16, u16)> {
    let p = auto_detect_aoc_problem()
        .map_err(|e| e.context("Error trying to determine AOC project environment"))?;
    Ok(p.ok_or_else(||anyhow!("You are not in a problem directory. Change directory to problem folder or use flags --year and --day."))?)
}

pub fn src_folder_exists() -> Result<bool> {
    let working_directory = env::current_dir()?;
    Ok(working_directory.join("src").exists())
}

pub fn list_days(year: u16) -> Result<Vec<u16>> {
    let working_directory = env::current_dir()?.join(format!("{}", year));
    Ok(fs::read_dir(working_directory)
        .with_context(|| format!("Unable to list AOC projects for year {}", year))?
        .filter_map(|d| d.ok())
        .map(|d| os_to_str(&d.file_name()))
        .filter(|d| d.starts_with("day"))
        .filter_map(|d| d[3..].parse::<u16>().ok())
        .collect::<Vec<_>>())
}
