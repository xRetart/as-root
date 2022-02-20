PREFIX = /usr/local

# binary
BIN = as-root
BIN_DEST = $(PREFIX)/bin

# man page
MAN = man
MAN_DEST = $(PREFIX)/share/man/man1/as-root.1

# cargo
CARGO_MODE = release
CARGO_OUTPUT=target/$(CARGO_MODE)/$(BIN)

$(CARGO_OUTPUT): src/*.rs Cargo.toml
	cargo build --$(CARGO_MODE)

install: $(CARGO_OUTPUT)
	install --owner=root --group=root --mode=6755 --strip $(CARGO_OUTPUT) $(BIN_DEST) && \
	install --owner=root --group=root --mode=644 $(MAN) $(MAN_DEST) && \
	bzip2 $(MAN_DEST)

uninstall:
	rm --recursive --force $(BIN_DEST)
