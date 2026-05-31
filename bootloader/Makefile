OUTPUT_DIR := out
BOOT_EFI := $(OUTPUT_DIR)/bootx64.efi

.PHONY: prepare always build clean

prepare: always
	mkdir -p $(OUTPUT_DIR)

build: prepare
	cargo build

$(BOOT_EFI): build
	cp target/x86_64-unknown-uefi/debug/bootloader.efi $(BOOT_EFI)

run: $(BOOT_EFI)
	echo "$(BOOT_EFI) created"

clean:
	rm -fr ${OUTPUT_DIR}
