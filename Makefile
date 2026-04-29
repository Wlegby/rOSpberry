# --- rOSpberry Makefile ---

BSP ?= rpi3

ifeq ($(BSP), rpi3)
    FEATURE = bsp_rpi3
else ifeq ($(BSP), rpi4)
    FEATURE = bsp_rpi4
else
    $(error ❌ Unsupported board: $(BSP). Use BSP=rpi3 or BSP=rpi4)
endif

.PHONY: all build clean

all: build

build:
	@echo "🛠️ Building rOSpberry for $(BSP)..."
	cargo build --target aarch64-unknown-none --no-default-features --features "$(FEATURE)"
	cargo objcopy --target aarch64-unknown-none --no-default-features --features "$(FEATURE)" -- -O binary ./kernel8.img
	@echo "✅ Success! kernel8.img is ready for your $(BSP)."

clean:
	cargo clean
	rm -f kernel8.img
	@echo "🧹 Workspace cleaned."
