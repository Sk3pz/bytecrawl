use crate::filesystem::directory::Directory;
use crate::filesystem::file::{File, FileContent};
use crate::player::Player;
use crate::shop::shop;

pub mod file;
pub mod directory;

pub struct FileSystem {
    root: Directory,
    pub current_directory: String,
}

impl FileSystem {
    pub fn new() -> Self {
        let root = Directory {
            name: String::from("/"),
            parent: None,
            files: Vec::new(),
            subdirectories: Vec::new(),
        };

        let current_directory = root.get_path().clone();

        FileSystem {
            root,
            current_directory
        }
    }

    // if successful, will parse a path from a string and return the remainder if it exists
    // this will handle cases where the path has spaces and is surrounded by ''
    pub fn parse_path_out_of_string(str: &String) -> Result<(String, Option<String>), String> {
        let mut path = String::new();
        let mut index = 0;

        // handle the case where the path is surrounded by ''
        if str.starts_with('\'') {
            index += 1;
            while str.chars().nth(index).unwrap() != '\'' {
                if index >= str.len() - 1 {
                    return Err("No closing ' found!".to_string());
                }
                path.push(str.chars().nth(index).unwrap());
                index += 1;
            }
            index += 1;
        } else {
            // handle the case where the path is not surrounded by ''
            while index < str.len() && str.chars().nth(index).unwrap() != ' ' {
                path.push(str.chars().nth(index).unwrap());
                index += 1;
            }
        }

        if index != str.len() {
            let remainder = str[index+1..str.len()].to_string();
            Ok((path, Some(remainder)))
        } else {
            Ok((path, None))
        }
    }

    pub fn separate_file_and_parent(path: String) -> Option<(String, String)> {
        // handle case where path is a file in root and there are no /'s
        if !path.contains('/') {
            return Some((String::from("/"), path));
        }
        path.rfind('/').map(|last_separator| (path[0..last_separator].to_string(), path[last_separator+1..path.len()].to_string()))
    }

    pub fn get_current_dir(&self) -> Option<&Directory> {
        if self.current_directory.as_str() == "/" {
            return Some(&self.root)
        }

        self.root.get_child(&self.current_directory)
    }

    pub fn get_current_dir_mut(&mut self) -> Option<&mut Directory> {
        if self.current_directory.as_str() == "/" {
            return Some(&mut self.root)
        }

        self.root.get_mut_child(&self.current_directory)
    }

    pub fn parse_path(&self, path: &str) -> Result<&Directory, String> {
        let mut current_dir = if path.starts_with('/') {
            &self.root
        } else {
            let Some(current) = self.get_current_dir() else {
                return Err("Failed to get current directory.".to_string());
            };
            current
        };

        let components: Vec<&str> = path.split('/').filter(|&c| !c.is_empty()).collect();

        for component in components {
            // handle .. and . paths
            if component == ".." {
                // go to parent
                current_dir = self.parse_path(current_dir.parent.clone().unwrap_or(String::from("/")).as_str())?;
                // this intentionally ignores a call for parent on root by skipping it.
                continue;
            }
            if component == "." {
                // no change to directory, but should be parsed
                continue;
            }

            let Some(dir) = current_dir.find_directory(component) else {
                return Err("Failed to find directory".to_string());
            };

            current_dir = dir;
        }

        Ok(current_dir)
    }

    pub fn parse_path_mut(&mut self, path: &str) -> Result<&mut Directory, String> {
        let mut current_dir = if path.starts_with('/') {
            &mut self.root
        } else {
            let Some(current) = self.get_current_dir_mut() else {
                return Err("Failed to get current directory.".to_string());
            };
            current
        };

        let components: Vec<&str> = path.split('/').filter(|&c| !c.is_empty()).collect();

        for component in components {
            // handle .. and . paths
            if component == ".." {
                // go to parent
                // todo: this creates multiple mutable references to self
                // current_dir = self.get_mut_parent(current_dir)?;
                println!("Due to a bug, '..' is not supported in certain capacities right now.");
                // this intentionally ignores a call for parent on root by skipping it.
                continue;
            }
            if component == "." {
                // no change to directory, but should be parsed
                continue;
            }

            let Some(dir) = current_dir.find_directory_mut(component) else {
                return Err("Failed to find directory".to_string());
            };

            current_dir = dir;
        }

        Ok(current_dir)
    }

    // todo: truncated version for long paths
    pub fn get_pwd(&self) -> String {
        self.current_directory.clone()
    }

    pub fn cd(&mut self, path: String) -> Result<(), String> {
        let new_path = self.parse_path(&path)?;

        self.current_directory = new_path.get_path();

        Ok(())
    }

    pub fn ls(&self) {
        println!("{}:", self.get_pwd());

        let Ok(current_dir) = self.parse_path(&self.current_directory.clone()) else {
            println!("Failed to get current directory from path!");
            return;
        };

        for x in 0..current_dir.subdirectories.len() {
            let subdir = &current_dir.subdirectories.get(x).unwrap();
            if x != &current_dir.subdirectories.len() - 1 || !current_dir.files.is_empty() {
                println!(" ├─{}", subdir);
            } else {
                println!(" └─{}", subdir);
            }
        }

        for x in 0..current_dir.files.len() {
            let file = &current_dir.files.get(x).unwrap();
            if x != &current_dir.files.len() - 1 {
                println!(" ├─{}", file);
            } else {
                println!(" └─{}", file);
            }
        }
    }

    pub fn mkdir(&mut self, path: &str) -> Result<(), String> {
        let components: Vec<&str> = path.split('/').filter(|&c| !c.is_empty()).collect();

        let mut current_dir = if path.starts_with('/') {
            &mut self.root
        } else {
            self.parse_path_mut(&self.current_directory.clone())?
        };

        // Handle the root directory
        if components.is_empty() {
            return Err("Specified path can not be empty!".to_string());
        }

        for component in components {
            let mut parent_path = current_dir.parent.clone().unwrap_or(String::from("/"));
            if current_dir.name != "/" {
                parent_path.push_str(current_dir.name.as_str());
                parent_path.push('/');
            }

            if component == ".." {
                // go to parent
                // current_dir = self.parse_path_mut(current_dir.parent.clone().unwrap_or(String::from("/")).as_str())?;
                println!("Due to a bug, '..' is not supported in certain capacities right now.");
                // update parent path
                // parent_path = current_dir.parent.clone().unwrap_or(String::from("/"));
                // this intentionally ignores a call for parent on root by skipping it.
                continue;
            }

            current_dir = if !current_dir.has_subdir(component) {
                current_dir.make_subdir(component, parent_path)?
            } else if let Some(dir) = current_dir.get_mut_child(component) {
                dir
            } else {
                return Err("Failed to get child directory!".to_string());
            };
        }

        Ok(())
    }

    pub fn touch<S: Into<String>>(&mut self, path: S, file: File) -> bool {
        let path_result = self.parse_path_mut(&path.into());
        let Ok(current_dir) = path_result else {
            return false;
        };
        current_dir.touch(file);
        true
    }

    fn get_file<S: Into<String>>(&mut self, path: S) -> Result<&File, String> {
        let mut file_name = path.into();
        // get parent directory
        let parent_dir = if file_name.contains('/') {
            let Some((parent, f_name)) = Self::separate_file_and_parent(file_name) else {
                return Err("Failed to get parent directory of specified file.".to_string());
            };

            file_name = f_name;
            if parent.is_empty() {
                &self.root
            } else {
                self.parse_path(&parent)?
            }
        } else {
            self.parse_path(&self.current_directory.clone())?
        };

        // get the file if it exists in the parent directory
        let Some(file) = parent_dir.find_file(file_name) else {
            return Err("File by that name does not exist!".to_string());
        };

        Ok(file)
    }

    fn get_file_mut<S: Into<String>>(&mut self, path: S) -> Result<&mut File, String> {
        let mut file_name = path.into();
        // get parent directory
        let parent_dir = if file_name.contains('/') {
            let Some((parent, f_name)) = Self::separate_file_and_parent(file_name) else {
                return Err("Failed to get parent directory of specified file.".to_string());
            };

            file_name = f_name;
            if parent.is_empty() {
                &mut self.root
            } else {
                self.parse_path_mut(&parent)?
            }
        } else {
            self.parse_path_mut(&self.current_directory.clone())?
        };

        // get the file if it exists in the parent directory
        let Some(file) = parent_dir.find_file_mut(file_name.clone()) else {
            return Err(format!("Failed to find file: {}", file_name));
        };

        Ok(file)
    }

    pub fn run<S: Into<String>>(&mut self, path: S, ps: &mut Player, args: Vec<String>) -> Result<(), String> {
        let file = self.get_file(path)?;

        let FileContent::Executable(run) = file.content else {
            return Err("This file is not executable!".to_string());
        };

        run(self, ps, args);

        Ok(())
    }

    pub fn cat<S: Into<String>>(&mut self, path: S) -> Result<(String, String), String> {
        let file = self.get_file(path)?;

        let FileContent::Text(txt) = &file.content else {
            return Err("This file is not a text file and can not be read!".to_string());
        };

        Ok((file.name.clone(), txt.clone()))
    }

    pub fn edit_file<S: Into<String>>(&mut self, path: S, new_content: FileContent) -> Result<(), String> {
        let file = self.get_file_mut(path)?;

        file.content = new_content;

        Ok(())
    }

    pub fn rm_file<S: Into<String>>(&mut self, path: S) -> Result<(), String> {
        // split the path and the file
        let file_name = path.into();
        let Some((path, file)) = Self::separate_file_and_parent(file_name) else {
            return Err("Failed to get parent directory of specified file.".to_string());
        };
        // get the parent directory
        let parent_dir = if path.is_empty() {
            &mut self.root
        } else {
            self.parse_path_mut(&path)?
        };
        // remove the file if it exists in the directory.
        parent_dir.rm_file(file.as_str())?;
        Ok(())
    }

    pub fn rm_dir<S: Into<String>>(&mut self, path: S) -> Result<(), String> {
        // separate the last directory from the path
        let dir_name = path.into();
        let Some((path, dir)) = Self::separate_file_and_parent(dir_name) else {
            return Err("Failed to get parent directory of specified directory.".to_string());
        };

        // get the parent directory
        let parent_dir = if path.is_empty() {
            &mut self.root
        } else {
            self.parse_path_mut(&path)?
        };

        // remove the directory if it exists in the parent directory
        parent_dir.rm_dir(dir.as_str())?;

        Ok(())
    }
}

impl Default for FileSystem {
    fn default() -> Self { Self::new() }
}