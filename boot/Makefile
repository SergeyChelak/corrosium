BUILD_DIR = build
BUILD_MODE = release
QEMU = qemu-system-x86_64

STAGE1_ELF = target/x86_16/${BUILD_MODE}/stage1
STAGE2_ELF = target/x86_32/${BUILD_MODE}/stage2

STAGE1 = ${BUILD_DIR}/stage1.bin 
STAGE2 = ${BUILD_DIR}/stage2.bin

BOOTLOADER = ${BUILD_DIR}/bootloader.bin

.PHONY: run setup always clean

${BOOTLOADER}: ${STAGE1} ${STAGE2}
	dd if=/dev/zero of=$@ count=64
	dd if=${STAGE1} of=$@ conv=notrunc
	dd if=${STAGE2} of=$@ conv=notrunc seek=1

${STAGE1_ELF}: always
	cargo build --${BUILD_MODE} --target=x86_16.json --package=stage1

${STAGE2_ELF}: always
	cargo build --${BUILD_MODE} --target=x86_32.json --package=stage2

${STAGE1}: ${STAGE1_ELF} setup
	objcopy -I elf32-i386 -O binary ${STAGE1_ELF} $@

${STAGE2}: ${STAGE2_ELF} setup	
	objcopy -I elf32-i386 -O binary ${STAGE2_ELF} $@

setup: always
	mkdir -p ${BUILD_DIR}

run: ${BOOTLOADER}
	${QEMU} -drive format=raw,file=$^

clean:
	cargo clean
	rm -fr ${BUILD_DIR}