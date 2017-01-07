OBJCOPY=objcopy
OBJDUMP=llvm-objdump39
LD=/usr/local/gcc-arm-embedded-5_4-2016q3-20160926/bin/arm-none-eabi-ld
SREC_CAT=srec_cat
FETCH=fetch
SERNO?=9900000037024e45006620080000004e0000000097969901
TTY?=/dev/ttyU0
MNT=/mnt

RUST_VERSION=1.14.0
RUST_TARGET_PATH=${PWD}

build: target/combined.hex

.FORCE:

target/sysroot/lib/rustlib/cortex-m0/lib/libcore.rlib:
.export RUST_TARGET_PATH
	mkdir -p target/sysroot/lib/rustlib/cortex-m0/lib
	test -f target/rustc-${RUST_VERSION}-src.tar.gz || (cd target && ${FETCH} https://static.rust-lang.org/dist/rustc-${RUST_VERSION}-src.tar.gz)
	test -d target/rustc-${RUST_VERSION} || (cd target && tar xvzf rustc-${RUST_VERSION}-src.tar.gz)
	cd target/rustc-${RUST_VERSION}/src/libcore && RUSTFLAGS='-C panic=abort' cargo build --target cortex-m0 --release
	cp target/rustc-${RUST_VERSION}/src/libcore/target/cortex-m0/release/libcore.rlib target/sysroot/lib/rustlib/cortex-m0/lib

target/cortex-m0/release/libmicrobit.a: target/sysroot/lib/rustlib/cortex-m0/lib/libcore.rlib .FORCE
	@RUSTFLAGS='--sysroot=target/sysroot' cargo build --target cortex-m0 --release --verbose
	
target/bin: target/cortex-m0/release/libmicrobit.a linker.ld Makefile
	${LD} \
		--gc-sections \
		-T linker.ld \
		-o target/bin \
		--verbose \
		target/cortex-m0/release/libmicrobit.a

target/hex: target/bin
	${OBJCOPY} -O ihex target/bin target/hex

target/combined.hex: target/hex contrib/BLE_BOOTLOADER_RESERVED.hex contrib/s110_nrf51822_8.0.0_softdevice.hex
	${SREC_CAT} \
		contrib/BLE_BOOTLOADER_RESERVED.hex -intel \
		contrib/s110_nrf51822_8.0.0_softdevice.hex -intel \
		target/hex -intel \
		-o target/combined.hex -intel
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
	rm -rf target
