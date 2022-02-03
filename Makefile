PREFIX = /usr/local
DEST = $(PREFIX)/bin
BIN = as-root
CARGO_MODE = release
CARGO_OUTPUT=target/$(CARGO_MODE)/$(BIN)

$(CARGO_OUTPUT): src/*.rs Cargo.toml
	cargo build --$(CARGO_MODE)

install: $(CARGO_OUTPUT)
	install --owner=root --group=root --mode=6755 --strip $(CARGO_OUTPUT) $(DEST)

uninstall:
	rm --recursive --force $(DEST)
