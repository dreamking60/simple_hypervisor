TARGET = aarch64-unknown-none.json

.PHONY: all clean

all:
	cargo xbuild --target=$(TARGET)

release:
	cargo xbuild --target=$(TARGET) --release

clean:
	cargo clean
