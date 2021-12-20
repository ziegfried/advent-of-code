use crate::fs_utils;
use anyhow::anyhow;
use anyhow::{Context, Result};
use std::env;
use std::fs;

pub fn auto_detect_aoc_problem() -> Result<Option<(u16, u16)>> {
    let working_directory = env::current_dir()?;
    let day_dir = fs_utils::file_name(&working_directory)?;
    if let Some(day_number) = day_dir.strip_prefix("day") {
        if let Ok(day) = day_number.parse::<u16>() {
            if let Some(parent) = working_directory.parent() {
                let year_dir = fs_utils::file_name(parent)?;
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
    p.ok_or_else(||anyhow!("You are not in a problem directory. Change directory to problem folder or use flags --year and --day."))
}

pub fn src_folder_exists() -> Result<bool> {
    let working_directory = env::current_dir()?;
    Ok(working_directory.join("src").exists())
}

pub fn list_days(year: u16) -> Result<Vec<u16>> {
    let working_directory = env::current_dir()?.join(format!("{}", year));
    Ok(fs::read_dir(working_directory)
        .with_context(|| format!("Unable to list AOC projects for year {}", year))?
        .filter_map(|dir_res| dir_res.ok())
        .map(|dir| dir.file_name().to_str().unwrap().to_string())
        .filter_map(|dir_name| dir_name.strip_prefix("day").map(|s| s.to_string()))
        .filter_map(|number| number.parse::<u16>().ok())
        .collect::<Vec<_>>())
}
