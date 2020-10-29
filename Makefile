arch ?= x86_64
kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso
target ?= $(arch)-crabos
crabos := target/$(target)/debug/libcrabos.a

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, \
	build/arch/$(arch)/%.o, $(assembly_source_files))

.PHONY: all clean run iso moo kernel

all: $(kernel)

clean:
	@rm -r build

run: $(iso)
	@qemu-system-x86_64 -cdrom $(iso) --serial "stdio"

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@grub-mkrescue -o $(iso) build/isofiles
	@rm -r build/isofiles
	@if grub-file --is-x86-multiboot $(kernel); then \
	  echo multiboot confirmed; \
	else \
	  echo the file is not multiboot; \
	fi
	@if grub-file --is-x86-multiboot2 $(kernel); then \
	  echo multiboot2 confirmed; \
	else \
	  echo the file is not multiboot2; \
	fi

$(kernel): kernel $(crabos) $(assembly_object_files) $(linker_script)
	@ld -n -T $(linker_script) -o $(kernel) \
		$(assembly_object_files) $(crabos)

# compile assembly files
build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@

kernel:
	@RUST_TARGET_PATH=$(shell pwd) cargo +nightly build