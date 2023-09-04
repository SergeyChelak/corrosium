ASM = nasm
SRC_DIR = src
BUILD_DIR = build

.PHONY: run, clean, always

$(BUILD_DIR)/boot.bin: always
	${ASM} -f bin ${SRC_DIR}/boot.asm -o $@

clean:
	rm -rf ${BUILD_DIR}

always:
	mkdir -p $(BUILD_DIR)

run: $(BUILD_DIR)/boot.bin
	qemu-system-x86_64 -drive format=raw,file=$^

	# dd if=/dev/zero bs=512 count=1  >> $(BUILD)/boot.bin
	# qemu-system-x86_64 -drive format=raw,file=$(BUILD)/boot.bin