use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct Character {
    name: String,
    level: u8,
    weapons: Vec<String>
}

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

    let c: Character = serde_json::from_str(data)?;

    println!(
        "Character \"{}\" is level {} and his first weapon is \"{}\"",
        c.name, c.level, c.weapons[0]
    );

    Ok(())
}
