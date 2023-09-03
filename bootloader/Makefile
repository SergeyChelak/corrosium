ASM = nasm
SRC = src
BUILD = build

.PHONY: run, clean

boot.bin:
	${ASM} -f bin ${SRC}/boot.asm -o ${BUILD}/$@

clean:
	rm ${BUILD}/*

run: boot.bin
	qemu-system-x86_64 -drive format=raw,file=$(BUILD)/$^

	# dd if=/dev/zero bs=512 count=1  >> $(BUILD)/boot.bin
	# qemu-system-x86_64 -drive format=raw,file=$(BUILD)/boot.bin