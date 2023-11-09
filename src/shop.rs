use crate::filesystem::file::{File, FileContent};

pub fn shop(file: &File) -> Result<(), String> {
    let FileContent::Shop { name } = &file.content else {
        return Err("Illegal call to shop on non-executable file!".to_string());
    };

    // todo: implement shops
    println!("Welcome to the {} shop!", name);
    println!("Type ls to browse the stock. Run `help` to see how to navigate the shop.");
    println!();
    println!("Shops are currently not implemented.");

    Ok(())
}