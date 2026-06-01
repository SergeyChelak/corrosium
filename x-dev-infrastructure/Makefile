UNAME_S := $(shell uname -s)

OUTPUT_DIR := esp/efi/boot
BOOT_EFI := $(OUTPUT_DIR)/bootx64.efi

ifeq ($(UNAME_S), Linux)
    QEMU_ACCEL := -enable-kvm
endif
ifeq ($(UNAME_S), Darwin)
    QEMU_ACCEL := -accel tcg
endif

.PHONY: run clean

$(OUTPUT_DIR):
	mkdir -p $(OUTPUT_DIR)

$(BOOT_EFI): $(OUTPUT_DIR)
	$(MAKE) -C ../bootloader out/bootx64.efi
	cp ../bootloader/out/bootx64.efi $(BOOT_EFI)

run: $(BOOT_EFI)
	qemu-system-x86_64 $(QEMU_ACCEL) \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
    -drive format=raw,file=fat:rw:esp

clean:
	rm -fr ${OUTPUT_DIR}

clean-all: clean
	$(MAKE) -C ../bootloader clean
