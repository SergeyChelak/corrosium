OUTPUT_DIR := out
BOOT_EFI := $(OUTPUT_DIR)/bootx64.efi

.PHONY: prepare always build

prepare: always
	mkdir -p $(OUTPUT_DIR)
	# rustup target add x86_64-unknown-uefi

build: prepare
	cargo build --target x86_64-unknown-uefi

$(BOOT_EFI): build
	cp target/x86_64-unknown-uefi/debug/bootloader.efi $(BOOT_EFI)

run: $(BOOT_EFI)
	echo "$(BOOT_EFI) created"
