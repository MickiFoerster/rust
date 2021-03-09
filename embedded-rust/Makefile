TARGET_PPC=powerpc-unknown-linux-gnu
TARGET_ARM=armv7-unknown-linux-gnueabihf
BINARY_NAME=$(shell basename `pwd`)
RELEASE_BINARY_PATH_PPC=target/$(TARGET_PPC)/release/$(BINARY_NAME)
RELEASE_BINARY_PATH_ARM=target/$(TARGET_ARM)/release/$(BINARY_NAME)
SOURCES=$(shell find -name "*.rs")

all: \
	$(RELEASE_BINARY_PATH_PPC) \
	$(RELEASE_BINARY_PATH_ARM)

$(RELEASE_BINARY_PATH_PPC): $(SOURCES)
	cross rustc --release --target $(TARGET_PPC)
$(RELEASE_BINARY_PATH_ARM): $(SOURCES)
	cross rustc --release --target $(TARGET_ARM)

rustup:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

cross:
	cargo install cross

.PHONY: all cross rustup
