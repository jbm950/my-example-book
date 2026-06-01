use std::process::Command;

#[derive(Debug)]
struct GitUser {
    name: String,
    email: String,
}

fn get_git_user() -> Result<GitUser, Box<dyn std::error::Error>> {
    let response = Command::new("git")
        .args(["config", "--get-regexp", "^user\\.(name|email)$"])
        .output()?;

    if !response.status.success() {
        return Err(format!("git exited with status \"{}\"", response.status).into());
    }

    let output = String::from_utf8(response.stdout)?;

    let mut name = String::new();
    let mut email = String::new();

    for line in output.lines() {
        if let Some(value) = line.strip_prefix("user.name ") {
            name = value.to_string();
        } else if let Some(value) = line.strip_prefix("user.email ") {
            email = value.to_string();
        }
    }

    if name.is_empty() {
        return Err("user.name not configured".into());
    }

    if email.is_empty() {
        return Err("user.email not configured".into());
    }

    Ok(GitUser { name, email })
}

fn main() {
    match get_git_user() {
        Ok(user) => println!("{user:?}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
