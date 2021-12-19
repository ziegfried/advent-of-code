#!/usr/bin/env run-cargo-script
use std::fs;
use std::io;
use std::path::Path;

fn list_dir<S: AsRef<Path>>(dir: S) -> Result<Vec<String>, io::Error> {
    let mut entries = vec![];
    for child in fs::read_dir(dir)? {
        if let Some(name) = child?.path().file_name() {
            entries.push(String::from(name.to_str().unwrap().clone()));
        }
    }
    Ok(entries)
}

fn main() -> Result<(), io::Error> {
    let mut contents = String::new();
    let mut years = vec![];
    for dir in list_dir(".")? {
        if let Ok(year) = dir.parse::<u32>() {
            years.push(year);
        }
    }
    years.sort();
    for year in years.iter().rev() {
        let mut days = vec![];
        for folder in list_dir(format!("./{}", year))? {
            if folder.starts_with("day") {
                if let Ok(day) = folder[3..].parse::<u32>() {
                    days.push(day);
                }
            }
        }
        days.sort();

        contents
            .push_str(format!("<details><summary>{year}</summary>\n<p>\n\n", year = year).as_str());
        for day in days.iter().rev() {
            contents.push_str(
                format!(
                    " - [Day {day}](./{year}/day{day:02}/src/main.rs)\n",
                    day = day,
                    year = year
                )
                .as_str(),
            );
        }
        contents.push_str("\n</p>\n</details>\n");
    }

    let readme_contents = fs::read_to_string("./README.md")?;
    let start = readme_contents.find("<!-- INDEX-START -->").unwrap();
    let end = readme_contents.find("<!-- INDEX-END -->").unwrap();
    
    let mut result = String::new();
    result.push_str(&readme_contents[0..start]);
    result.push_str(&"<!-- INDEX-START -->\n");
    result.push_str(contents.as_str());
    result.push_str(&readme_contents[end..]);

    fs::write("README.md", result)?;

    Ok(())
}
