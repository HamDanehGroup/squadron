use dialoguer::{console::Term, theme::ColorfulTheme, Select};

pub fn select_type() -> std::io::Result<(String)> {
    let items = vec!["Postman"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What is Input File Type?")
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => return Ok(String::from(items[index])),
        None => panic!("User did not select anything"),
    }
}
