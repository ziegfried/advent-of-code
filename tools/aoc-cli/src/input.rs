pub fn download_input(year: u16, day: u16, session: String) -> reqwest::Result<String> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(format!(
            "https://adventofcode.com/{}/day/{}/input",
            year, day
        ))
        .header("Cookie", format!("session={}", session))
        .send()?;
    if !response.status().is_success() {
        return Err(response.error_for_status().err().unwrap());
    }
    response.text()
}
