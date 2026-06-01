OUTPUT_DIR := out
KERNEL_ELF := $(OUTPUT_DIR)/kernel.elf

.PHONY: prepare always build clean

prepare: always
	mkdir -p $(OUTPUT_DIR)

build: prepare
	cargo build

$(KERNEL_ELF): build
	cp target/x86_64-unknown-none/debug/kernel $(KERNEL_ELF)

run: $(KERNEL_ELF)
	echo "$(KERNEL_ELF) created"

clean:
	rm -fr ${OUTPUT_DIR}
