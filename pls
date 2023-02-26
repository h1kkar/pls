#!/bin/lua
-- pls from h1kkar

-- HELP FUNCTION --
local function help()
print [[

usage: pls [command]

available commands:
  dunst               open config dunst
  sh                  open config hilbish
  wm                  open config herbstluftwm
  nvim                open config nvim
  picom               open config picom
  qute                open config qutebrowser
  term                open config kitty
  marks               open config qutemarks
  xres                open xresources

  ping                ping output
  xup                 xresources config reload
  flashmnt            mount my flash drive
  push                push git repo
  calc                calculator
  xmod                chmod +x

  pulse               restart pulseaudio
  wal                 change wallpaper
  edit                edit pls in editor
  fetch               output fetch

  pkg                 install package
  del                 delete package
  upd                 update system
  untar               extract tar-archive
  rem                 delete file

  off                 poweroff
  reboot              reboot
  suspend             suspend
  out                 logout of the user

  help                output this information
  ver                 output pls version
]]
end

-- LOCAL VARIABLE --

-- editor
local ed = "nvim "

-- conf dir
local conf = "$HOME/.config/"

-- home dir
local home = "$HOME/"

-- pls file
local pls = "/bin/pls"

-- ver
v = "0.1"

-- PUSH FUNCTION --
local function push()
  -- add
  print ("add files...")
  os.execute ("sleep 1")
  os.execute ("git add -A")
  
  -- commit
  os.execute ("sleep 1")
  if arg[2] ~= nil then
    print ("commiting...")
    os.execute ("git commit -m" .. arg[2])
  else
    print ("enter a commit message")
  end
    -- push
  os.execute ("git push origin master")
end

-- PULSEAUDIO RELOAD FUNCTION --
local function pulse()
  -- kill
  print ("killing pulseaudio")
  os.execute ("sleep 1")
  os.execute ("pulseaudio --kill")
  
  -- start
  print ("starting pulseaudio")
  os.execute ("sleep 2")
  os.execute ("pulseaudio -D")

  os.execute ("sleep 1")
end

-- CHANGE WALLPAPER FUNCTION --
local function wal()
  -- output img in dir
  os.execute ("ls $HOME/.wp")
  
  -- input name img
  print ("\nname: ")
  wal = io.read()
  
  -- change wal
  print ("change wal")
  os.execute ("feh --bg-fill " .. home .. ".wp/" .. wal)
end

-- BRANCHING ARGUMENTS --

-- help function
if arg[1] == "help" then
  help()

-- open dunst conf
elseif arg[1] == "dunst" then
  os.execute (ed .. conf .. "dunst/dunstrc")

-- open hilbish conf
elseif arg[1] == "sh" then
  os.execute (ed .. conf .."hilbish/init.lua")

-- open herbstluftwm conf
elseif arg[1] == "wm" then
  os.execute (ed .. conf .. "herbstluftwm/autostart")

-- open neovim conf
elseif arg[1] == "nvim" then
  os.execute (ed .. conf .. "nvim/init.lua")

-- open picom compositor conf
elseif arg[1] == "picom" then
  os.execute (ed .. conf .. "picom/picom.conf")

-- open qutebrowser conf
elseif arg[1] == "qute" then
  os.execute (ed .. conf .. "qutebrowser/config.py")

-- open kitty conf
elseif arg[1] == "term" then
  os.execute (ed .. conf .. "kitty/kitty.conf")

-- open qutemarks conf
elseif arg[1] == "marks" then
  os.execute (ed .. conf .. "qutebrowser/quickmarks")

-- open Xresources
elseif arg[1] == "xres" then
  os.execute (ed .. home .. ".Xresources")

-- ping out
elseif arg[1] == "ping" then
  os.execute ("ping -c 1 ya.ru")

-- reload Xresources
elseif arg[1] == "xup" then
  os.execute ("xrdb -merge $HOME/.Xresources")

-- mount my flash drive
elseif arg[1] == "flashmnt" then
  os.execute ("sudo mount dev/sdc1 /home/hikkar/.flash")

-- git push
elseif arg[1] == "push" then
  push()

-- calculator
elseif arg[1] == "calc" then
  os.execute ("echo $((" .. arg[2] .. "))")

  -- root files
elseif arg[1] == "xmod" then
  os.execute ("chmod +x " .. arg[2])

-- pulseaudio reload
elseif arg[1] == "pulse" then
  pulse()

-- change wallpaper
elseif arg[1] == "wal" then
  wal()

-- edit pls
elseif arg[1] == "edit" then
  os.execute (ed .. pls)

elseif arg[1] == "fetch" then
  os.execute ("fetch -c $HOME/.config/fetch/conf/cat")

-- install pkg
elseif arg[1] == "pkg" then
  os.execute ("yay -S " .. arg[2])

-- delete pkg
elseif arg[1] == "del" then
  os.execute ("yay -Rcc " .. arg[2])

-- update sys or/and pkg
elseif arg[1] == "upd" then
  if arg[2] ~= nil then
    os.execute ("yay -Syu " .. arg[2])
  else
    os.execute ("yay -Syu")
  end

-- untar archive
elseif arg[1] == "untar" then
  os.execute ("tar -zxvf " .. arg[2])

-- delete files in dir
elseif arg[1] == "rem" then
  os.execute ("rm -rf " .. arg[2])

-- poweroff
elseif arg[1] == "off" then
  os.execute ("poweroff")

-- reboot
elseif arg[1] == "reboot" then
  os.execute ("reboot")

-- suspend
elseif arg[1] == "suspend" then
  os.execute ("systemctl suspend")
  
-- logout
elseif arg[1] == "out" then
  os.execute ("pkill -9 -u $USER")

-- ver
elseif arg[1] == "ver" then
  print ("pls v" .. v)

-- nil val
elseif arg[1] == nil then
  print ("\n" .. "no arg received")
  print ("\n" .. "read \"pls help\" for help\n")

-- false val
else
  print ("\n" .. arg[1] .. " is not pls command")
  print ("\n" .. "read \"pls help\" for help\n")
end