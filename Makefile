PREFIX = /usr/local

NAME = as-root

GLOBAL = $(PREFIX)/bin/$(NAME)
LOCAL = target/release/$(NAME)


$(LOCAL): src/*.rs Cargo.toml
	cargo build --release

install: $(LOCAL)
	install -o root -g root -m 6755 -s $(LOCAL) $(GLOBAL) && \
	install -o root -g root -m 644 man $(PREFIX)/share/man/man1/as-root.1

uninstall:
	$(RM) $(GLOBAL)
