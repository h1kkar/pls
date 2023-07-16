pub fn usage() {
    println!("
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

  ver               output pls version")
}