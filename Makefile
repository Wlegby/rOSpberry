# --- rOSpberry Makefile ---

BSP ?= rpi3

# Map the chosen board to the correct Cargo feature AND Linker Script
ifeq ($(BSP), rpi3)
    FEATURE = bsp_rpi3
    LINKER_SCRIPT = ./src/bsp/r3bp/linker.ld
else ifeq ($(BSP), rpi4)
    FEATURE = bsp_rpi4
    LINKER_SCRIPT = ./src/bsp/r4b/linker.ld
else
    $(error ❌ Unsupported board: $(BSP). Use BSP=rpi3 or BSP=rpi4)
endif

.PHONY: all build deploy clean

all: build

# Notice we replaced ./linker.ld with $(LINKER_SCRIPT) here:
build:
	@echo "🛠️ Building rOSpberry for $(BSP)..."
	rm ./kernel8.img
	cargo rustc --no-default-features --features "$(FEATURE)" -- -C link-arg=--script=$(LINKER_SCRIPT)
	cargo objcopy -- -O binary ./kernel8.img
	@echo "✅ Success! kernel8.img is ready for your $(BSP)."

clean:
	cargo clean
	rm -f kernel8.img
	@echo "🧹 Workspace cleaned."
