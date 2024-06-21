use anyhow::Result;
use inquire::Text;

#[derive(Debug)]
pub struct Password {
    name: String,
    site: String,
    password: Vec<u8>,
}

pub fn prompt_for_password() -> Result<Password> {
    let name = Text::new("Enter name for Password: ").prompt()?;
    let site = Text::new("Enter site: ").prompt()?;
    let password = Text::new("Enter Password: ").prompt()?.as_bytes().to_vec();

    Ok(Password {
        name,
        site,
        password,
    })
}
