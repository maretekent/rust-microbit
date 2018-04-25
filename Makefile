.include "config.mk"

XARGO_RUST_SRC=${BUILD_DIR}/rustc-${RUST_VERSION}-src/src

.FORCE:

# fetch the rust source
${BUILD_DIR}/rustc-${RUST_VERSION}-src.tar.gz:
	mkdir -p ${BUILD_DIR}
	cd ${BUILD_DIR} && ${FETCH} https://static.rust-lang.org/dist/rustc-${RUST_VERSION}-src.tar.gz

# extract the rust source
${BUILD_DIR}/rustc-${RUST_VERSION}-src: ${BUILD_DIR}/rustc-${RUST_VERSION}-src.tar.gz
	cd ${BUILD_DIR} && tar xvzf rustc-${RUST_VERSION}-src.tar.gz

target/${TARGET}/release/microbit: ${BUILD_DIR}/rustc-${RUST_VERSION}-src .FORCE
.export XARGO_RUST_SRC
	xargo build --release

target/${TARGET}/release/microbit.hex: target/${TARGET}/release/microbit
	${OBJCOPY} -O ihex target/${TARGET}/release/microbit target/${TARGET}/release/microbit.hex

build: target/${TARGET}/release/microbit.hex

flash: build
	mount -t msdos /dev/serno/${SERNO} ${MNT}
	test -f ${MNT}/details.txt
	cp target/${TARGET}/release/microbit.hex ${MNT}
	umount ${MNT}
	echo "Successfully flashed"

serial:
	cat ${TTY}

diss: target/${TARGET}/release/microbit
	${OBJDUMP} -d target/${TARGET}/release/microbit

clean:
	rm -rf ${TARGET_DIR}

realclean:
	rm -rf ${BUILD_DIR}
