cargo build --release -- %*
copy /Y target\release\charfind.exe .
rcedit charfind.exe --set-icon images\charfind.ico
copy /Y charfind.exe C:\bin
