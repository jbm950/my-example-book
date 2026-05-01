fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(
        "https://secure.runescape.com/m=hiscore_oldschool/index_lite.json?player=Salvsis2"
        )?;

    println!("Status: {}", response.status());

    let body = response.text()?;
    println!("Body:\n {}", body);

    Ok(())
}
