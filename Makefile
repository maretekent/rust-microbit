.include "config.mk"

RUST_VERSION=1.19.0

# This is where the cortex-m0.json file is located
RUST_TARGET_PATH=${PWD}

TARGET_DIR=${BUILD_DIR}/target
CARGO_TARGET_DIR=${TARGET_DIR}

build: ${TARGET_DIR}/combined.hex

.FORCE:

${TARGET_DIR}/sysroot/lib/rustlib/cortex-m0/lib/libcore.rlib:
.export RUST_TARGET_PATH
.export CARGO_TARGET_DIR
	mkdir -p ${TARGET_DIR}/sysroot/lib/rustlib/cortex-m0/lib
	test -f ${BUILD_DIR}/rustc-${RUST_VERSION}-src.tar.gz || (cd ${BUILD_DIR} && ${FETCH} https://static.rust-lang.org/dist/rustc-${RUST_VERSION}-src.tar.gz)
	test -d ${BUILD_DIR}/rustc-${RUST_VERSION}-src || (cd ${BUILD_DIR} && tar xvzf rustc-${RUST_VERSION}-src.tar.gz)
	cd ${BUILD_DIR}/rustc-${RUST_VERSION}-src/src/libcore && RUSTFLAGS='-C opt-level=3' cargo build --target cortex-m0 --release
	cp ${TARGET_DIR}/cortex-m0/release/libcore.rlib ${TARGET_DIR}/sysroot/lib/rustlib/cortex-m0/lib

${TARGET_DIR}/cortex-m0/release/libmicrobit.a: ${TARGET_DIR}/sysroot/lib/rustlib/cortex-m0/lib/libcore.rlib .FORCE
.export CARGO_TARGET_DIR
	@RUSTFLAGS="--sysroot=${TARGET_DIR}/sysroot -C opt-level=3" cargo build --target cortex-m0 --release --verbose
	
${TARGET_DIR}/bin: ${TARGET_DIR}/cortex-m0/release/libmicrobit.a linker.ld Makefile
	${LD} \
		--gc-sections \
		-T linker.ld \
		-o ${TARGET_DIR}/bin \
		--verbose \
		${TARGET_DIR}/cortex-m0/release/libmicrobit.a

${TARGET_DIR}/hex: ${TARGET_DIR}/bin
	${OBJCOPY} -O ihex ${TARGET_DIR}/bin ${TARGET_DIR}/hex

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
