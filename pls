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
  term                open config alacritty
  marks               open config qutemarks
  xres                open xresources
  open                open file in dir

  ping                ping output
  xup                 xresources config reload
  push                push git repo
  calc                calculator
  xmod                chmod +x
  echo                output text to a file

  pulse               restart pulseaudio
  wal                 change wallpaper
  edit                edit pls in editor
  fetch               output fetch
  scr                 take a screenshot

  pkg                 install package
  del                 delete package
  upd                 update system
  untar               extract tar-archive
  rm                  delete file
  neko                cat analogue

  off                 poweroff
  reboot              reboot
  suspend             suspend
  out                 logout of the user

  ver                 output pls version
]]
end

-- LOCAL VARIABLE --

-- editor
local ed = "vscodium "

-- conf dir
local conf = "$HOME/.config/"

-- home dir
local home = "$HOME/"

-- pls file
local pls = "/bin/pls"

-- table
local tbl = {}

-- count arg
local c = #arg

-- ver
local v = "0.5"

-- NO ARG RECEIVED FUNCTION --
local function noarg()
  if arg[2] == nil then
    print ("no arg received")
  end
end

-- check function
local function check(cmd)
  if arg[2] ~= nil then
    for i = 2, #arg do
-- table imsert
      if arg[i] ~= nil then table.insert(tbl, arg[i])
      else break
      end
    end
    os.execute(cmd .. table.concat(tbl, " "))
  else noarg()
  end
end

-- PUSH FUNCTION --
local function push()
-- add
  noarg()
  if arg[2] ~= nil then
    print ("add files...")
    os.execute ("git add .")

-- commit
    print ("commiting...")
    for i=2, c do
      if arg[i] ~= nil then table.insert(tbl, arg[i])
      else break
      end
    end
    os.execute ("git commit -m \"" .. table.concat(tbl, " " ) .. "\"")

-- push
    os.execute ("git push origin master")
  end
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
  os.execute (ed .. conf .. "alacritty/alacritty.yml")

-- open qutemarks conf
elseif arg[1] == "marks" then
  os.execute (ed .. conf .. "qutebrowser/quickmarks")

-- open Xresources
elseif arg[1] == "xres" then
  os.execute (ed .. home .. ".Xresources")

-- open
elseif arg[1] == "open" then
  if arg[2] ~= nil then
    os.execute (ed .. arg[2])
  end

-- ping out
elseif arg[1] == "ping" then
  os.execute ("ping -c 1 ya.ru")

-- reload Xresources
elseif arg[1] == "xup" then
  os.execute ("xrdb -merge $HOME/.Xresources")

-- git push
elseif arg[1] == "push" then
  push()

-- calculator
elseif arg[1] == "calc" then
  os.execute ("echo $((" .. arg[2] .. "))")

-- root files
elseif arg[1] == "xmod" then
  for i=2, c do
    if arg[i] ~= nil then
      os.execute ("chmod +x " .. arg[i])
    end
  end
  noarg()

-- echo
elseif arg[1] == "echo" then
  d = c - 1
  for i=2, d do
    if arg[i] ~= nil then table.insert(tbl, arg[i])
    else break
    end
  end
  os.execute ("echo \'" .. table.concat(tbl, " ") .. "\' >>" .. arg[c])

-- pulseaudio reload
elseif arg[1] == "pulse" then
  pulse()

-- change wallpaper
elseif arg[1] == "wal" then
  wal()

-- edit pls
elseif arg[1] == "edit" then
  os.execute (ed .. pls)

-- fetch
elseif arg[1] == "fetch" then
  os.execute ("fetch -c $HOME/.config/fetch/conf/cat")

-- scr
elseif arg[1] == "scr" then
  if arg[2] == nil then
    os.execute ("scrot $HOME/img/scr.png && notify-send \'Screenshot taken!\'")
  elseif arg[2] == "d" then
    os.execute ("scrot -d 5 $HOME/img/scr.png && notify-send \'Screenshot taken!\'")
  elseif arg[2] == "s" then
    os.execute ("scrot -s $HOME/img/scr.png && notify-send \'Screenshot taken!\'")
  else print (arg[2] .. "is not a valid arg")
  end

-- install pkg
elseif arg[1] == "pkg" then
  check("yay -S ")
  

-- delete pkg
elseif arg[1] == "del" then
  check("yay -Rcc ")

-- update sys or/and pkg
elseif arg[1] == "upd" then
  if arg[2] ~= nil then
    for i=2, #arg do
-- table insert
      if arg[i] ~= nil then table.insert(tbl, arg[i])
      else break
      end
    end
    os.execute ("yay -Syu " .. table.concat(tbl, " "))
  else os.execute ("yay -Syu")
  end

-- untar archive
elseif arg[1] == "untar" then
  check("tar -zxvf ")

-- delete files in dir
elseif arg[1] == "rm" then
  check("rm -rf ")

-- neko
elseif arg[1] == "neko" then
  if arg[3] ~= nil then
    os.execute ('grep ' .. arg[3] .. " " .. arg[2] .. " | moar")
  else noarg()
  end

-- poweroff
elseif arg[1] == "off" then
  os.execute ("systemctl poweroff")

-- reboot
elseif arg[1] == "reboot" then
  os.execute ("systemctl reboot")

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
  print ("no arg received")
  print ("read \"pls help\" for help")

-- false val
else
  print (arg[1] .. " is not pls command")
  print ("read \"pls help\" for help")
end