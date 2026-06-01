UNAME_S := $(shell uname -s)

OUTPUT_DIR := esp/efi/boot
BOOTLOADER := $(OUTPUT_DIR)/bootx64.efi
KERNEL := esp/kernel.elf

ifeq ($(UNAME_S), Linux)
    QEMU_ACCEL := -enable-kvm
endif
ifeq ($(UNAME_S), Darwin)
    QEMU_ACCEL := -accel tcg
endif

.PHONY: run clean clean-all

$(OUTPUT_DIR):
	mkdir -p $(OUTPUT_DIR)

$(BOOTLOADER): $(OUTPUT_DIR)
	$(MAKE) -C ../bootloader out/bootx64.efi
	cp ../bootloader/out/bootx64.efi $(BOOTLOADER)

$(KERNEL):
	$(MAKE) -C ../kernel out/kernel.elf
	cp ../kernel/out/kernel.elf $(KERNEL)

run: $(BOOTLOADER) $(KERNEL)
	qemu-system-x86_64 $(QEMU_ACCEL) \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
    -drive format=raw,file=fat:rw:esp

clean:
	rm -fr ${OUTPUT_DIR}
	rm -f $(KERNEL)

clean-all: clean
	$(MAKE) -C ../bootloader clean
	$(MAKE) -C ../kernel clean

clean-bootloader:
	rm -f $(BOOTLOADER)
	$(MAKE) -C ../bootloader clean

clean-kernel:
	rm -f $(KERNEL)
	$(MAKE) -C ../kernel clean
