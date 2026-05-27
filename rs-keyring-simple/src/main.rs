use keyring_core::{Entry, Result};
use linux_keyutils_keyring_store::Store;

fn main() -> Result<()> {
    keyring_core::set_default_store(Store::new()?);

    let entry = Entry::new("temp-service", "temp-user")?;
    entry.set_password("OmgCoolPassword!")?;

    let password = entry.get_password()?;
    println!("My password is {password}");

    entry.delete_credential()?;
    Ok(())
}
