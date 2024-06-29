BUILD_DIR = build
BUILD_MODE = release
QEMU = qemu-system-x86_64

.PHONY: run setup always clean

${BUILD_DIR}/bootloader.bin: ${BUILD_DIR}/stage1.bin ${BUILD_DIR}/stage2.bin
	dd if=/dev/zero of=$@ count=64
	dd if=${BUILD_DIR}/stage1.bin of=$@ conv=notrunc
	dd if=${BUILD_DIR}/stage2.bin of=$@ conv=notrunc seek=1

${BUILD_DIR}/stage1.bin: setup
	cargo build --${BUILD_MODE} --target=x86_16.json --package=stage1
	objcopy -I elf32-i386 -O binary target/x86_16/${BUILD_MODE}/stage1 $@

${BUILD_DIR}/stage2.bin: setup
	cargo build --${BUILD_MODE} --target=x86_32.json --package=stage2
	objcopy -I elf32-i386 -O binary target/x86_32/${BUILD_MODE}/stage2 $@

setup: always
	mkdir -p ${BUILD_DIR}

run: ${BUILD_DIR}/bootloader.bin
	${QEMU} -drive format=raw,file=$^

clean:
	cargo clean
	rm -fr ${BUILD_DIR}