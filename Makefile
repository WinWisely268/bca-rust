BINNAME := bca-rust
PREFIX := $(HOME)/.local

.PHONY: install

install: build
	install -m755 $(PWD)/target/release/$(BINNAME)  $(PREFIX)/bin/$(BINNAME)

build:
	cargo build --release
	strip $(PWD)/target/release/$(BINNAME)

clean:
	rm -rf $(PREFIX)/bin/$(BINNAME)

