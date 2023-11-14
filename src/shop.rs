use crate::filesystem::file::{File, FileContent};

pub fn shop<S: Into<String>>(name: S) -> Result<(), String> {
    let name = name.into();

    // todo: implement shops
    println!("Welcome to the {} shop!", name);
    println!("Type ls to browse the stock. Run `help` to see how to navigate the shop.");
    println!();
    println!("Shops are currently not implemented.");

    Ok(())
}