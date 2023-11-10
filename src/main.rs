use std::fmt::Display;
use rand::Rng;
use crate::command::Command;
use crate::filesystem::file::{File, FileContent};
use crate::filesystem::FileSystem;

mod command;
mod filesystem;
mod shop;

/***
  * TODO LIST
  *   - [ ] implement tutorial
  *   - [ ] implement save/load
  *   - [ ] implement procedural generation of directories for select directories
  *   - [ ] implement "shops" in the root directory where users can spend bytes
  *   - [ ] implement an inventory system
***/

// this allows debug commands to be disabled on release builds but be available in debug builds.
#[cfg(debug_assertions)]
const DEBUG: bool = true;
#[cfg(not(debug_assertions))]
const DEBUG: bool = false;

pub struct Player {
    health: u32,
    pub score: u32,
    pub bytes: u32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            health: 100,
            score: 0,
            bytes: 0,
        }
    }

    pub fn write_file(&mut self, file: &mut File) {
        file.content = FileContent::Text(format!("{}", self));
    }

    /// returns true if the player dies from the damage
    pub fn damage(&mut self, amount: u32) -> bool {
        if self.health > amount {
            self.health -= amount;
            false
        } else {
            self.health = 0;
            true
        }
    }

    pub fn heal(&mut self, amount: u32) {
        self.health += amount;
    }
}

impl Default for Player {
    fn default() -> Self { Self::new() }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Health: {}", self.health)?;
        writeln!(f, "Score: {}", self.score)?;
        write!(f, "Bytes: {}", self.bytes)?;
        Ok(())
    }
}

// creates and populates the FileSystem with the initial file structure
fn create_fs(ps: &mut Player, with_tutorial: bool) -> Result<FileSystem, String> {
    let mut fs = FileSystem::new();

    // create the default shops
    fs.mkdir("/shops/")?;
    fs.touch("/shops", File {
        name: "test_shop".to_string(),
        content: FileContent::Shop { name: "Scripts".to_string() }
    });

    // create the default scripts?
    fs.mkdir("/scripts/")?;
    fs.touch("/scripts", File {
        name: "README".to_string(),
        content: FileContent::Text("Scripts are currently not implemented.".to_string())
    });

    // procedural pain
    fs.mkdir("/dungeon/door1")?;
    fs.touch("/dungeon/door1", File {
        name: "loot_example".to_string(),
        content: FileContent::Executable(&|_, ps, _| {
            println!("You found a loot box! You got 10 bytes!");
            ps.bytes += 10;
        })
    });
    fs.mkdir("/dungeon/door2")?;
    fs.touch("/dungeon/door2", File {
        name: "gamble_example".to_string(),
        content: FileContent::Executable(&|_, ps, _| {
            println!("You open the crate...");
            let rng = rand::thread_rng().gen_range(0..=1);
            if rng == 0 {
                println!("You found 10 bytes!");
                ps.bytes += 10;
            } else {
                println!("You found a file gremlin that takes some bytes! :(");
                if ps.bytes >= 10 {
                    ps.bytes -= 10;
                } else if ps.bytes != 0 {
                    println!("The gremlin took all your remaining bytes. :(");
                    ps.bytes = 0;
                } else {
                    println!("You didnt have any bytes for the gremlin to take, so it just left.");
                }
            }
        })
    });
    fs.mkdir("/dungeon/door3")?;

    // create the stats file
    fs.touch("/", File {
        name: "stats".to_string(),
        content: FileContent::Text(format!("{}", ps)),
    });

    // create a way to access the tutorial
    if with_tutorial {
        fs.touch("/", File {
            name: "tutorial".to_string(),
            content: FileContent::Executable(&|_, _, _| {

                if let Err(e) = run_tutorial() {
                    println!("Failed to run tutorial. {}", e);
                }

            })
        });
    }

    // populate the main area, and have designated directories that lead to generated paths

    Ok(fs)
}

fn run_tutorial() -> Result<(), String> {
    println!("Welcome to the tutorial! This will run you through the basics of the game!\n\
    All changes made here will not be reflected in the actual game, so feel free to experiment!");
    let mut ps = Player::default();
    let Ok(_fs) = create_fs(&mut ps, false) else {
        return Err("Failed to create and populate virtual filesystem, exiting program.".to_string());
    };

    // todo: run user through the commands and how to play the game
    println!("the tutorial is currently not implemented, sorry :(");

    Ok(())
}

fn main() {
    let mut ps = Player::default();
    let Ok(mut fs) = create_fs(&mut ps, true) else {
        println!("Failed to create and populate virtual filesystem, exiting program.");
        return;
    };

    println!("Welcome to ByteCrawl! Type ls to get your bearings. Run the tutorial program with `./tutorial`.");

    loop {
        let input = better_term::read_input!("{}> ", fs.get_pwd());

        let cmd = Command::parse(input, DEBUG);

        let result = cmd.execute(&mut fs, &mut ps);

        if let Ok(exit) = result {
            if exit {
                break;
            }
        } else if let Err(e) = result {
            println!("{}", e);
        }

        // update player stats file
        if let Err(e) = fs.edit_file("/stats", FileContent::Text(format!("{}", ps))) {
            println!("Couldn't write stats to file. {}", e);
        }
    }

    println!("Exited safely. Thanks for playing!");
}
