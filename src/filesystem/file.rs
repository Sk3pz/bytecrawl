use std::fmt::{Display, Formatter};
use crate::filesystem::FileSystem;
use crate::player::Player;

pub enum FileContent {
    Text(String), // text files are currently read only
    Executable(&'static dyn Fn(&mut FileSystem, &mut Player, Vec<String>)), // executables are not editable by users
}

impl Display for FileContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = String::from(match self {
            FileContent::Text(_) => "TXT",
            FileContent::Executable(_) => "EXEC",
        });
        write!(f, "{}", str)
    }
}

pub struct File {
    pub name: String,
    pub content: FileContent,
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "🗎 {} ({})", self.name, self.content)
    }
}