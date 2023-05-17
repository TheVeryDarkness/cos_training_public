# Allows the usage of unstable features in stable compilers.
export RUSTC_BOOTSTRAP := 1

STEP ?= b0
STEP_FEATURES ?=

OBJCOPY ?= rust-objcopy --binary-architecture=riscv64

OUT_DIR := apps/$(STEP)/target/riscv64gc-unknown-none-elf/release
OUT_ELF := $(OUT_DIR)/$(STEP)
OUT_BIN := $(OUT_DIR)/$(STEP).bin

all: build

build: $(OUT_BIN)

$(OUT_BIN): $(OUT_ELF)
	$(OBJCOPY) $(OUT_ELF) --strip-all -O binary $@

$(OUT_ELF): FORCE
	cargo rustc -Zbuild-std=core,alloc \
		-Zbuild-std-features=compiler-builtins-mem \
		--target riscv64gc-unknown-none-elf --release \
		--features "$(STEP_FEATURES)" \
		--manifest-path apps/$(STEP)/Cargo.toml -- \
		-Clink-arg=-T../../libos/linker_riscv64.lds

run: build justrun
	@:

justrun:
	qemu-system-riscv64 -m 128M -smp 1 -machine virt \
		-bios default -kernel $(OUT_BIN) -nographic

clean:
	@rm -rf apps/*/target
	@rm -rf apps/*/linker_riscv64_generated.lds

.PHONY: all build clean run justrun FORCE
