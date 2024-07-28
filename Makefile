UNAME = ${shell uname}
ifeq (${UNAME}, Linux)
	MKFS_FAT = mkfs.fat
	MCOPY = mcopy
endif
ifeq (${UNAME}, Darwin)
	MKFS_FAT = ${shell brew --prefix dosfstools}/sbin/mkfs.fat
	MCOPY = ${shell brew --prefix mtools}/bin/mcopy
endif

BUILD_DIR = build
QEMU = qemu-system-x86_64

DISK_IMAGE = ${BUILD_DIR}/corrosium.iso
BOOTLOADER = ${BUILD_DIR}/bootloader.bin
KERNEL = ${BUILD_DIR}/kernel.bin
KERNEL_BUILD_MODE = release

.PHONY: prepare clean run always

${DISK_IMAGE}: prepare ${BOOTLOADER} ${KERNEL}
	dd if=/dev/zero of=${DISK_IMAGE} bs=1M count=10
# reserve 64 sectors x 512 bytes
	${MKFS_FAT} -R 64 ${DISK_IMAGE}
	dd if=${BOOTLOADER} of=$@ conv=notrunc
	${MCOPY} -i $@ ${KERNEL} "::"
	${MCOPY} -i $@ ./cfg/* "::"

${BOOTLOADER}: always
	make -C boot
	cp ./boot/build/bootloader.bin $@	

${KERNEL}: always
	make -C kernel
	cp ./kernel/build/kernel.bin $@	

prepare: 
	cp ./cfg/* ./boot/
	cp ./cfg/x86_32.json ./kernel/
	mkdir -p ${BUILD_DIR}

run: ${DISK_IMAGE}
	${QEMU} -drive format=raw,file=$^

clean:
	make -C boot clean
	make -C kernel clean
	rm -fr ${BUILD_DIR}
