use std::fmt::{Display, Formatter};
use crate::filesystem::file::File;

pub struct Directory {
    pub name: String,
    pub parent: Option<String>,
    pub subdirectories: Vec<Directory>,
    pub files: Vec<File>,
}

impl Directory {
    pub fn get_path(&self) -> String {
        format!("{}{}", self.parent.clone().unwrap_or_default(), self.name)
    }

    pub fn find_directory(&self, name: &str) -> Option<&Directory> {
        self.subdirectories.iter().find(|subdir| {
            subdir.name == name
        })
    }

    pub fn get_child<S: Into<String>>(&self, path: S) -> Option<&Directory> {
        let child_path = path.into();

        let components: Vec<&str> = child_path.split('/').filter(|&c| !c.is_empty()).collect();

        let mut current = self;

        for component in components {
            let search = current.find_directory(component);
            search?;
            current = search.unwrap();
        }

        Some(current)
    }

    pub fn get_mut_child<S: Into<String>>(&mut self, path: S) -> Option<&mut Directory> {
        let child_path = path.into();
        
        if child_path.is_empty() {
            return None;
        }

        let components: Vec<&str> = child_path.split('/').filter(|&c| !c.is_empty()).collect();

        let mut current = self;

        for component in components {
            let search = current.find_directory_mut(component);
            search.as_ref()?;
            current = search.unwrap();
        }

        Some(current)
    }

    pub fn find_directory_mut(&mut self, name: &str) -> Option<&mut Directory> {
        self.subdirectories.iter_mut().find(|dir| dir.name == name)
    }

    pub fn find_file(&self, name: String) -> Option<&File> {
        self.files.iter().find(|file| file.name == name)
    }

    pub fn find_file_mut(&mut self, name: String) -> Option<&mut File> {
        self.files.iter_mut().find(|f| f.name == name)
    }

    pub fn touch(&mut self, file: File) {
        self.files.push(file);
    }

    pub fn has_subdir(&self, name: &str) -> bool {
        self.subdirectories.iter().any(|subdir| subdir.name == name)
    }

    pub fn make_subdir(&mut self, name: &str, parent_path: String) -> Result<&mut Directory, String> {
        let new_dir = Directory {
            name: name.to_string(),
            parent: Some(parent_path.to_string()),
            files: Vec::new(),
            subdirectories: Vec::new(),
        };

        if self.subdirectories.iter().any(|subdir| subdir.name == name) {
            // Directory with the same name already exists
            return Err(format!("Directory already exists: {}", name));
        }

        self.subdirectories.push(new_dir);

        let index = self.subdirectories.len() -1;

        Ok(&mut self.subdirectories[index])
    }

    pub fn rm_file(&mut self, name: &str) -> Result<(), String> {
        let index = self.files.iter().position(|file| file.name == name);

        if let Some(index) = index {
            self.files.remove(index);
            Ok(())
        } else {
            Err(String::from("File not found"))
        }
    }

    pub fn rm_dir(&mut self, name: &str) -> Result<(), String> {
        let index = self.subdirectories.iter().position(|dir| dir.name == name);

        if let Some(index) = index {
            self.subdirectories.remove(index);
            Ok(())
        } else {
            Err(String::from("Directory not found"))
        }
    }
}

impl Display for Directory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "📁 {}", self.name)
    }
}