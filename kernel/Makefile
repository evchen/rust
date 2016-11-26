arch ?=x86_64
kernel :=build/kernel-$(arch).bin
iso :=build/os-$(arch).iso

linker_script :=src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm,  build/arch/$(arch)/%.o, $(assembly_source_files))
target ?= $(arch)-unknown-linux-gnu
rust_os := target/$(target)/debug/libblog_os.a

.PHONY: all clean run iso

all: $(kernel)

clean:
	@rm -r build

run: $(iso)
	@qemu-system-x86_64 -cdrom $(iso)

iso: $(iso)

cargo:
	@cargo build --target $(target)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(grub_cfg) build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@grub-mkrescue -o $(iso) build/isofiles 2> /dev/null
	@rm -r build/isofiles

$(kernel): cargo $(rust_os) $(linker_script) $(assembly_object_files)
	@ld -n --gc-sections -T $(linker_script) -o $(kernel) $(assembly_object_files) $(rust_os)

build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -f elf64 $< -o $@
