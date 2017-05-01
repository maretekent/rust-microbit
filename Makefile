.include "config.mk"

RUST_VERSION=1.16.0

# This is where the cortex-m0.json file is located
RUST_TARGET_PATH=${PWD}

build: target/combined.hex

.FORCE:

target/sysroot/lib/rustlib/cortex-m0/lib/libcore.rlib:
.export RUST_TARGET_PATH
	mkdir -p target/sysroot/lib/rustlib/cortex-m0/lib
	test -f target/rustc-${RUST_VERSION}-src.tar.gz || (cd target && ${FETCH} https://static.rust-lang.org/dist/rustc-${RUST_VERSION}-src.tar.gz)
	test -d target/rustc-${RUST_VERSION}-src || (cd target && tar xvzf rustc-${RUST_VERSION}-src.tar.gz)
	cd target/rustc-${RUST_VERSION}-src/src/libcore && RUSTFLAGS='-C panic=abort -C opt-level=3' cargo build --target cortex-m0 --release
	cp target/rustc-${RUST_VERSION}-src/src/target/cortex-m0/release/libcore.rlib target/sysroot/lib/rustlib/cortex-m0/lib

target/cortex-m0/release/libmicrobit.a: target/sysroot/lib/rustlib/cortex-m0/lib/libcore.rlib .FORCE
	@RUSTFLAGS='--sysroot=target/sysroot -C opt-level=3 -C panic=abort' cargo build --target cortex-m0 --release --verbose
	
target/bin: target/cortex-m0/release/libmicrobit.a linker.ld Makefile
	${LD} \
		--gc-sections \
		-T linker.ld \
		-o target/bin \
		--verbose \
		target/cortex-m0/release/libmicrobit.a

target/hex: target/bin
	${OBJCOPY} -O ihex target/bin target/hex

target/combined.hex: target/hex
	cp target/hex target/combined.hex
	ls -la target/combined.hex

flash:
	test -f target/combined.hex
	mount -t msdos /dev/serno/${SERNO} ${MNT}
	test -f ${MNT}/details.txt
	cp target/combined.hex ${MNT}
	umount ${MNT}
	echo "Successfully flashed"

serial:
	cat ${TTY}

diss: target/bin
	${OBJDUMP} -d target/bin

clean:
	rm -rf target/rustc-${RUST_VERSION}-src target/sysroot

realclean:
	rm -rf target
