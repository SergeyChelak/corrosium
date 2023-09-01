ASM = nasm
SRC = src
BUILD = build

.PHONY: run, clean

preloader.bin:
	${ASM} -f bin ${SRC}/preloader.asm -o ${BUILD}/$@

clean:
	rm ${BUILD}/*

run: preloader.bin
	qemu-system-x86_64 -drive format=raw,file=$(BUILD)/$^