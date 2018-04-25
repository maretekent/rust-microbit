.include "config.mk"

RUST_VERSION=1.24.0

TARGET_DIR=${BUILD_DIR}/target
CARGO_TARGET_DIR=${TARGET_DIR}

SYSROOT=${TARGET_DIR}/sysroot
TARGET=thumbv6m-none-eabi

PATH := $(PATH):/usr/local/bin

RUSTFLAGS=--sysroot ${SYSROOT} -C link-arg=--gc-sections -C link-arg=-Tlink.x -C linker=arm-none-eabi-ld -Z linker-flavor=ld

build: ${TARGET_DIR}/combined.hex

.FORCE:

${TARGET_DIR}/sysroot/lib/rustlib/${TARGET}/lib/libcore.rlib:
.export CARGO_TARGET_DIR
	mkdir -p ${TARGET_DIR}/sysroot/lib/rustlib/${TARGET}/lib
	test -f ${BUILD_DIR}/rustc-${RUST_VERSION}-src.tar.gz || (cd ${BUILD_DIR} && ${FETCH} https://static.rust-lang.org/dist/rustc-${RUST_VERSION}-src.tar.gz)
	test -d ${BUILD_DIR}/rustc-${RUST_VERSION}-src || (cd ${BUILD_DIR} && tar xvzf rustc-${RUST_VERSION}-src.tar.gz)
	cd ${BUILD_DIR}/rustc-${RUST_VERSION}-src/src/libcore && RUSTFLAGS="-C opt-level=3" cargo build --target ${TARGET} --release
	cp ${TARGET_DIR}/${TARGET}/release/libcore.rlib ${SYSROOT}/lib/rustlib/${TARGET}/lib

${TARGET_DIR}/${TARGET}/release/microbit: ${SYSROOT}/lib/rustlib/${TARGET}/lib/libcore.rlib .FORCE
.export PATH
.export RUSTFLAGS
	cargo build --release --verbose
	
${TARGET_DIR}/hex: ${TARGET_DIR}/${TARGET}/release/microbit
	${OBJCOPY} -O ihex ${TARGET_DIR}/${TARGET}/release/microbit ${TARGET_DIR}/hex

${TARGET_DIR}/combined.hex: ${TARGET_DIR}/hex
	cp ${TARGET_DIR}/hex ${TARGET_DIR}/combined.hex
	ls -la ${TARGET_DIR}/combined.hex

flash:
	test -f ${TARGET_DIR}/combined.hex
	mount -t msdos /dev/serno/${SERNO} ${MNT}
	test -f ${MNT}/details.txt
	cp ${TARGET_DIR}/combined.hex ${MNT}
	umount ${MNT}
	echo "Successfully flashed"

serial:
	cat ${TTY}

diss: ${TARGET_DIR}/bin
	${OBJDUMP} -d ${TARGET_DIR}/bin

clean:
	rm -rf ${TARGET_DIR}

realclean:
	rm -rf ${BUILD_DIR}
