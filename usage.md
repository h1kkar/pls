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
$ pls opn Cargo.toml
nvim open Cargo.toml
```

##### to delete any number of files and dirs write:
```
$ pls rm test test2 test3
you really want to remove the non empty test dir?
yes or Enter for removal: yes
file test1 in dir test deleted
dir test deleted
dir test2 deleted
file test3 deleted
```

##### to move or rename a file or dir specify the starting file, location and name of the target file or dir write:
```
# rename
$ pls mv test test1
test rename to test1

# move
$ pls mv test test1/test
test move to test1/test
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
pls v1.1.0
```