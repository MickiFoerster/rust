TARGET_ppc32=powerpc-unknown-linux-gnu
TARGET_armv7=armv7-unknown-linux-gnueabihf
BINARY_NAME=$(shell basename `pwd`)
RELEASE_BINARY_PATH_ppc32=target/$(TARGET_ppc32)/release/$(BINARY_NAME)
RELEASE_BINARY_PATH_armv7=target/$(TARGET_armv7)/release/$(BINARY_NAME)
SOURCES=$(shell find -name "*.rs")

all: armv7 ppc32
ppc32: $(RELEASE_BINARY_PATH_ppc32)
armv7: $(RELEASE_BINARY_PATH_armv7)

$(RELEASE_BINARY_PATH_armv7): $(SOURCES)
	cross rustc --release --target $(TARGET_armv7)
$(RELEASE_BINARY_PATH_ppc32): $(SOURCES)
	cross rustc --release --target $(TARGET_ppc32)

upload:
	scp $(RELEASE_BINARY_PATH_armv7) root@embedded-system-with-ppc32-or-armv7:/tmp/

rustup:
	curl --proto '=https' --tlsv1.3 -sSf https://sh.rustup.rs | sh

cross:
	cargo install cross

.PHONY: cross rustup upload
