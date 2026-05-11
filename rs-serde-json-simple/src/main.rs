use serde_json::Value;

fn main() -> std::io::Result<()> {
    let data = r#"
        {
            "name": "Link",
            "level": 46,
            "weapons": [
                "Master Sword",
                "Bow"
            ]
        }
    "#;

    let v: Value = serde_json::from_str(data)?;

    println!(
        "Character {} is level {} and his first weapon is {}",
        v["name"], v["level"], v["weapons"][0]
    );

    Ok(())
}
