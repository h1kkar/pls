BINDIR = /bin/
PKGFILE = target/release/pls

build:
	@echo build package
	@cargo build --release

install:
	@echo copy pkgfile to /bin
	@install ${PKGFILE} ${BINDIR}pls

completion:
	@echo install autocompletion for bash
	@install completions/complete.bash /usr/share/bash-completion/completions/pls.bash
	@echo install autocompletion for fish
	@install completions/complete.fish /usr/share/fish/completions/pls.fish

uninstall:
	@echo remove pkgfile
	@rm /bin/pls

test: