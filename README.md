```
         _--"-.
      .-"      "-.
     |""--..      '-.
     |      ""--..   '-.
     |            ""--..".         ____                               _               _   
     |                   |        / ___|___  _ __ ___   ___ _ __ ___ | |__   ___ _ __| |_ 
     |                 .-'       | |   / _ \| '_ ` _ \ / _ \ '_ ` _ \| '_ \ / _ \ '__| __|
     '--..             -.        | |__| (_) | | | | | |  __/ | | | | | |_) |  __/ |  | |_ 
          ""--..         :        \____\___/|_| |_| |_|\___|_| |_| |_|_.__/ \___|_|   \__|
                ""--..   |
                      ""-'      Com(mand re)member t(ool)
```

---

![preview](media/preview.gif)

## Requirements
- fzf: 
https://github.com/junegunn/fzf
- Either bash, zsh or fish shell.

## Installation
### Bash
```bash
cargo install comembert
mkdir ~/.config/comembert
wget https://raw.githubusercontent.com/ficolas2/comembert/refs/heads/main/scripts/bash/cm -O ~/.config/comembert/cm.sh
echo 'source ~/.config/comembert/cm.sh' >> ~/.bashrc
```

### Zsh
```bash
cargo install comembert
mkdir ~/.config/comembert
wget https://raw.githubusercontent.com/ficolas2/comembert/refs/heads/main/scripts/zsh/cm -O ~/.config/comembert/cm.zsh
echo 'source ~/.config/comembert/cm.zsh' >> ~/.zshrc
```

### Fish shell
```bash
cargo install comembert
wget https://raw.githubusercontent.com/ficolas2/comembert/refs/heads/main/scripts/fish/cm.fish -O ~/.config/fish/functions/cm.fish
```

## Usage
- [Read a command to the terminal prompt](#read-a-command-to-the-terminal-prompt)
- [Save a command](#save-a-command)
- [Save the last command](#save-the-last-command)
- [Edit a command](#edit-a-command)
- [Delete a command](#delete-a-command)
- [Move a command](#move-a-command)
- [List all commands](#list-all-commands)

### Read a command to the terminal prompt
```bash
cm
```
**NOTE**: THIS ONLY WORKS ON FISH SHELL. If you are using bash or zsh, you need to press the keybinding Ctrl+F
fzf will show you a list of the saved commands, select one, and it will be printed to the terminal input.

### Save a command
```bash
cm --add 'command' 'command/route'
```

### Save the last command
```bash
cm --prev 'command/route'
```

### Edit a command
```bash
cm --edit 'command/route'
```
Your default editor will be oppened with the selected command.

If no command route is specified, fzf will be opened to select a command.

Note: the editor is defined by the $EDITOR environment variable.

### Move a command
```bash
cm --move 'command/route' 'new/command/route'
```
If no command route is specified, fzf will be opened once to select the command to move, and then you will be prompted to enter the new route.

### Delete a command
```bash
cm --delete 'comand/route'
```
If no command route is specified, fzf will be opened to select a command.

### List all commands
```bash
cm --list
```

## Planned features
- [ ] Read an openapi file, and save the commands as curl requests.

## License
This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

