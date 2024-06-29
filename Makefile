BUILD_DIR = build
QEMU = qemu-system-x86_64

DISK_IMAGE = ${BUILD_DIR}/corrosuim.iso

.PHONY: prepare clean run

${DISK_IMAGE}: prepare ${BUILD_DIR}/bootloader.bin ${BUILD_DIR}/kernel.bin
	dd if=/dev/zero of=${DISK_IMAGE} bs=1M count=10
# reserve 64 sectors x 512 bytes
	mkfs.fat -R 64 ${DISK_IMAGE}
	dd if=${BUILD_DIR}/bootloader.bin of=$@ conv=notrunc
	mcopy -i $@ ${BUILD_DIR}/kernel.bin "::"

${BUILD_DIR}/bootloader.bin:
	cd boot && \
	make && \
	cd ..
	cp ./boot/build/bootloader.bin $@	

${BUILD_DIR}/kernel.bin:
	touch $@
	echo hello > $@

prepare:
	mkdir -p ${BUILD_DIR}

run: ${DISK_IMAGE}
	${QEMU} -drive format=raw,file=$^

clean:
	cd boot && \
	cargo clean && \
	cd ..
	cd kernel && \
	cargo clean && \
	cd .. 
	rm -fr ${BUILD_DIR}
