<div align="center">
    <h1>pls</h1>
</div>

#### 🌸 *pls this is a little helper in your work*

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
- more fixes and optimizations (3 fixes)
- implement give permissions for files
- ~~implement smart deletion of folders with files~~
- ~~implement smart move/rename~~
- ~~add completion for bash, fish~~
- redesign code structure