BUILD_DIR = build
QEMU = qemu-system-x86_64

DISK_IMAGE = ${BUILD_DIR}/corrosuim.iso
BOOTLOADER = ${BUILD_DIR}/bootloader.bin
KERNEL = ${BUILD_DIR}/kernel.bin
KERNEL_BUILD_MODE = release

.PHONY: prepare clean run always

${DISK_IMAGE}: prepare ${BOOTLOADER} ${KERNEL}
	dd if=/dev/zero of=${DISK_IMAGE} bs=1M count=10
# reserve 64 sectors x 512 bytes
	mkfs.fat -R 64 ${DISK_IMAGE}
	dd if=${BOOTLOADER} of=$@ conv=notrunc
	mcopy -i $@ ${KERNEL} "::"

${BOOTLOADER}: always
	cd boot && \
	make && \
	cd ..
	cp ./boot/build/bootloader.bin $@	

${KERNEL}: always
	cd kernel && \
	make && \
	cd ..
	cp ./kernel/build/kernel.bin $@	

prepare: 
	cp ./cfg/* ./boot/
	cp ./cfg/x86_32.json ./kernel/
	mkdir -p ${BUILD_DIR}

run: ${DISK_IMAGE}
	${QEMU} -drive format=raw,file=$^

clean:
	cd boot && \
	make clean && \
	cd ..
	cd kernel && \
	make clean && \
	cd .. 
	rm -fr ${BUILD_DIR}
