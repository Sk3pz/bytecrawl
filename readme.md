# ByteCrawl
A terminal dungeon crawler where you must go through the PC to find treasure and fight malware.  
Written completely in rust as a passion project.  

This currently only implements a virtual filesystem with basic commands to go through it. There are text files, executable files and directories.
    
If built in debug mode, the `debug` command becomes available. `debug help` lists its available subcommands.
  
Current directory structure:
```
(root) /:
  ├─ 📁 shops/
  |  └─ 🗎 test_shop (EXEC)
  ├─ 📁 scripts/
  |  └─ 🗎 README (TXT)
  ├─ 📁 dungeon/
  |  ├─ 📁 door1
  |  |  └─ 🗎 loot_example (EXEC)
  |  ├─ 📁 door2
  |  |  └─ 🗎 gamble_example (EXEC)
  |  └─ 📁 door3
  ├─ 🗎 stats (TXT)
  └─ 🗎 tutorial (EXE)
```
  
TODO LIST  
  - [ ] implement tutorial
  - [ ] implement enemies and a way to fight them
  - [ ] implement procedural generation of directories for select directories  
  - [ ] implement "shops" in the root directory where users can spend bytes  
  - [ ] implement an inventory system
  - [ ] implement save/load
