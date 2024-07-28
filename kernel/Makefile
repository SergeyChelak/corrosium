OBJCOPY ?= objcopy

BUILD_DIR = build
KERNEL = ${BUILD_DIR}/kernel.bin
BUILD_MODE = release

.PHONY: setup always clean

${KERNEL}: setup
	cargo build --${BUILD_MODE} --target=x86_32.json
	${OBJCOPY} -I elf32-i386 -O binary target/x86_32/${BUILD_MODE}/kernel $@

setup: always
	mkdir -p ${BUILD_DIR}

clean:
	cargo clean
	rm -fr ${BUILD_DIR}