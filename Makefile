ARCH =? x86_64
TARGET = $(ARCH)-FOMOS.json
FINAL_CMD =? mv
BUILD_CMD =? cargo build --target $(TARGET)
IMAGE_CMD =? cargo build --target $(TARGET)
IMAGE_NAME =? FOMOSv3.img

ifeq ($(ARCH), x86_64)
	TARGET = x86_64-FOMOS.json
	BUILD_CMD = cargo build --target $(TARGET)
	IMAGE_CMD = cargo bootimage --target $(TARGET)
	IMAGE_NAME = FOMOSv3.img
	FINAL_CMD = mv target/x86_64-FOMOS/debug/bootimage-FOMOSv3-Blue.bin $(IMAGE_NAME) && echo Created $(IMAGE_NAME)
endif

ifeq ($(ARCH), aarch64)
	TARGET = aarch64-FOMOS.json
	BUILD_CMD = cargo build --target $(TARGET) --release
	IMAGE_CMD = rust-objcopy --strip-all -O binary target/aarch64-FOMOS/release/FOMOSv3-Blue $(IMAGE_NAME)
	IMAGE_NAME = kernel8.img
	FINAL_CMD = echo Created $(IMAGE_NAME)
endif

all: build image

build:
	@ $(BUILD_CMD)

image:
	@ $(IMAGE_CMD)
	@ $(FINAL_CMD)

clean:
	@ cargo clean
	@ rm -rf *.img
