BUILD_DIR = build
KERNEL = ${BUILD_DIR}/kernel.bin
BUILD_MODE = release
QEMU = qemu-system-x86_64

.PHONY: setup always clean

${KERNEL}: setup
	cargo build --${BUILD_MODE} --target=x86_32.json
	objcopy -I elf32-i386 -O binary target/x86_32/${BUILD_MODE}/kernel $@

setup: always
	mkdir -p ${BUILD_DIR}

clean:
	cargo clean
	rm -fr ${BUILD_DIR}