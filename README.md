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
                      ""-'      Co(mmand re)member t(ool)
```

## Overview
Comembert (Command remember tool) is a cli command manager, that allows you to save commands, 
and output them directly onto your command line, so that you can modify them before running them.

![preview](media/preview.gif)

## Requirements
- fzf: 
https://github.com/junegunn/fzf
- Either bash, zsh or fish shell.

## Installation
### Bash
**Note**: for bash, the command cm won't work to insert the command to the terminal prompt, you need to
use the keybinding Ctrl+F. See [Insertion options](#insertion-options).
```bash
cargo install comembert
mkdir ~/.config/comembert
wget https://raw.githubusercontent.com/ficolas2/comembert/refs/heads/main/scripts/bash/cm -O ~/.config/comembert/cm.sh
echo 'source ~/.config/comembert/cm.sh' >> ~/.bashrc
```

### Zsh
**Note**: for zsh, the command cm won't work to insert the command to the terminal prompt, you need to
use the keybinding Ctrl+F. See [Insertion options](#insertion-options).
```bash
cargo install comembert
mkdir ~/.config/comembert
wget https://raw.githubusercontent.com/ficolas2/comembert/refs/heads/main/scripts/zsh/cm -O ~/.config/comembert/cm.zsh
echo 'source ~/.config/comembert/cm.zsh' >> ~/.zshrc
```

### Fish
```bash
cargo install comembert
wget https://raw.githubusercontent.com/ficolas2/comembert/refs/heads/main/scripts/fish/cm.fish -O ~/.config/fish/functions/cm.fish
```

## Usage
- [Read a command to the terminal prompt](#insert-a-command-to-the-terminal-prompt)
- [Save a command](#save-a-command)
- [Save the last command](#save-the-last-command)
- [Edit a command](#edit-a-command)
- [Delete a command](#delete-a-command)
- [Move a command](#move-a-command)
- [List all commands](#list-all-commands)

### Insert a command to the terminal prompt
```bash
cm
```
**NOTE**: INSERTING WITH THE COMMAND ONLY WORKS ON FISH SHELL, or with clipboard output mode, 
see [Insertion options](#insertion-options). If you are using bash or zsh, you need to press the 
keybinding Ctrl+F for command insertions.


fzf will show you a list of the saved commands, select one, and it will be printed to the terminal input.

### Save a command
```bash
cm add 'command' 'command/route'
```

### Save the last command
```bash
cm prev 'command/route'
```

### Edit a command
```bash
cm edit 'command/route'
```
Your default editor will be oppened with the selected command.

If no command route is specified, fzf will be opened to select a command.

Note: the editor is defined by the $EDITOR environment variable.

### Move a command
```bash
cm move 'command/route' 'new/command/route'
```
If no command route is specified, fzf will be opened once to select the command to move, and then you will be prompted to enter the new route.

### Delete a command
```bash
cm delete 'comand/route'
```
If no command route is specified, fzf will be opened to select a command.

### List all commands
```bash
cm list
```

## Insertion options
### Using the system clipboard, and simulating input
If you want to insert the command to the terminal prompt using the system clipboard, you can set the
environment variable 'COMEMBERT_OUTPUT' to 'clipboard'. This will copy the command to the clipboard,
and simulate the input to the terminal.

Currently, you need to have xdotool to simulate the input, and xclip to copy the command to the 
clipboard. This means that this option is only available on linux, with X11, but this should change 
in the future.

### Bash or Zsh
Bash and zsh don't allow inserting the command directly to the terminal prompt when running another 
command.

To insert the command to the terminal prompt, it has to be called from a keybinding. The default 
keybinding is Ctrl+F, but you can change it modifying the file `~/.config/comembert/cm.sh` or 
`~/.config/comembert/cm.zsh`.

### Fish
Fish shell allows inserting the command directly to the terminal prompt when running another command. 
This means that you can simply call `cm` and, pick the command, and it will be inserted to the terminal.

If you want to add a keybinding in fish shell, you can add add the binding to the file 
`~/.config/fish/functions/fish_user_key_bindings.fish`:
More info at https://fishshell.com/docs/current/cmds/bind.html

## Planned features
- [ ] Read an openapi file, and save the commands as curl requests.

## License
This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

