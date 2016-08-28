GCC_ARM_PATH=/usr/local/gcc-arm-embedded-5_4-2016q2-20160622
GCC=${GCC_ARM_PATH}/bin/arm-none-eabi-gcc
OBJCOPY=${GCC_ARM_PATH}/bin/arm-none-eabi-objcopy
OBJDUMP=${GCC_ARM_PATH}/bin/arm-none-eabi-objdump
LD=${GCC_ARM_PATH}/bin/arm-none-eabi-ld
SREC_CAT=srec_cat
FETCH=fetch

build: target/combined.hex

.FORCE:

target/sysroot/lib/rustlib/cortex-m0/lib/libcore.rlib:
	mkdir -p target/sysroot/lib/rustlib/cortex-m0/lib
	test -f target/rustc-1.11.0-src.tar.gz || (cd target && ${FETCH} https://static.rust-lang.org/dist/rustc-1.11.0-src.tar.gz)
	test -d target/rustc-1.11.0 || (cd target && tar xvzf rustc-1.11.0-src.tar.gz)
	cp cortex-m0.json target/rustc-1.11.0/src/libcore
	cd target/rustc-1.11.0/src/libcore && RUSTFLAGS='-C panic=abort' cargo build --target cortex-m0 --release
	cp target/rustc-1.11.0/src/libcore/target/cortex-m0/release/libcore.rlib target/sysroot/lib/rustlib/cortex-m0/lib

target/cortex-m0/release/libmicrobit.a: target/sysroot/lib/rustlib/cortex-m0/lib/libcore.rlib .FORCE
	@RUSTFLAGS='--sysroot=target/sysroot' cargo build --target cortex-m0 --release --verbose
	
target/bin: target/cortex-m0/release/libmicrobit.a contrib/NRF51822.ld
	${LD} \
		--gc-sections \
		-Tcontrib/NRF51822.ld \
		target/cortex-m0/release/libmicrobit.a \
		-o target/bin

target/hex: target/bin
	${OBJCOPY} -O ihex target/bin target/hex

target/combined.hex: target/hex contrib/BLE_BOOTLOADER_RESERVED.hex contrib/s110_nrf51822_8.0.0_softdevice.hex
	${SREC_CAT} \
		contrib/BLE_BOOTLOADER_RESERVED.hex -intel \
		contrib/s110_nrf51822_8.0.0_softdevice.hex -intel \
		target/hex -intel \
		-o target/combined.hex -intel
	ls -lah target/combined.hex

dis: target/bin
	${OBJDUMP} -d target/bin

clean:
	rm -rf target
