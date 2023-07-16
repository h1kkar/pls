<div align="center">
    <h1>pls</h1>
    <h3>🌸 <strong>pls this is a little helper in your work</strong></h3>
</div>

### screenshot

![pls scrot](/scrots/pls.png)

### install
1. clone this repo:
```
$ git clone https://github.com/h1kkar/pls.git
```
2. run in your shell:
```
$ make build
# make install
```
if you don't have a `make` then run these command:
```
$ cargo build --release
$ install target/release/pls /bin/pls
```

### uninstall
1. run in your shell:
```
$ make uninstall
# or
$ rm /bin/pls
```

#### reference info
To receive reference information, contact [usage.md](https://github.com/h1kkar/pls/blob/main/usage.md)

#### todo
- [ ] more fixes and optimizations (4 fixes)
- [ ] implement give permissions for files
- [x] implement smart deletion of folders with files
- [x] implement smart move/rename
- [x] add completion for bash, fish
- [x] redesign code structure
- [x] calculator takes expressions from arguments