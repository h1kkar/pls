BINDIR = /bin
PKGFILE = target/release/pls

all: pkg install

pkg:
	@echo build package
	@cargo b --release

install:
	@echo copy pkgfile to /bin
	@cp -i $(PKGFILE) $(BINDIR)/pls

uninstall:
	@echo remove pkgfile
	@rm /bin/pls
