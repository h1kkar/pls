## pls

#### 🌸 *pls this is a little helper in your work*

### screenshot

![pls scrot](/scrots/scr_pls.png)

### install
1. clone this repo:
```
$ git clone https://github.com/h1kkar/pls.git
```
2. run in your shell:
```
$ make all
# or
$ make pkg && make install
```
if you don't have a `make` then run these command:
```
$ cargo build --release
$ cp -i target/release/pls /bin/pls
```

### uninstall
1. run in your shell:
```
$ make uninstall
# or
$ rm /bin/pls
```

#### todo
- more fixes and optimizations
- implement give permissions for files
- implement smart deletion of folders with files
- build.rs
- redesign code structure