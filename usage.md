## usage
##### to display help info type in shell:
```
$ pls hlp

usage: pls [command]

available commands:
  sh                open config shell
  wm                open config window manager
  tty               open config terminal
  open              open file in dir

  rm                delete files
  mv                rename files
  cp                copy file
  calc              calculator

  off               poweroff
  rb                reboot
  sleep             suspend
  out               logout

  ver               output pls version

```

##### to open files in an editor ( which must be specified in the shell's `$EDITOR` variable, otherwise the file will open in `vi` ) write:
```
# to open shell config files
$ pls sh
nvim open config.fish

# to open window manager config files or desktop environment setting
$ pls wm
open gnome settings

# to open terminal config files ( must be specified in the $TERM shell variable )
$ pls tty
nvim open wezterm.lua

# to open files any in dir
$ pls open Cargo.toml
nvim open Cargo.toml
```

##### to delete any number of files and *empty* dirs write:
```
$ pls rm
dir test is delete
file test1 is delete
file test2 is delete
dir test3 is delete
```

##### to move ( or rename ) a file or dir specify the starting file, location and name of the target file or dir write:
```
$ pls mv test test1
test rename to test1
```

##### to copy a file or dir specify the source file, location and name of the target file or dir write:
```
$ pls cp test test1
test copy to test1
```

##### to run the calculator write:
```
$ pls calc
❯ 25+44
= 69

❯ exit
bye-bye
```

##### to shutdown, restart, put to sleep, exit the current session, write:
```
$ pls off
$ pls rb
$ pls sleep
$ pls out
```

##### to display the version of pls or any other program (if it supports the `--version` flag) write:
```
$ pls ver
pls v1.0.0

$ pls ver alacritty
alacritty 0.12.1 (5fdfd47f)
```