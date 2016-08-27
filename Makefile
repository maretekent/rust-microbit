GCC_ARM_PATH=/usr/local/gcc-arm-embedded-5_4-2016q2-20160622
AS=${GCC_ARM_PATH}/bin/arm-none-eabi-as
GCC=${GCC_ARM_PATH}/bin/arm-none-eabi-gcc
OBJCOPY=${GCC_ARM_PATH}/bin/arm-none-eabi-objcopy
SREC_CAT=srec_cat

build: target/combined.hex

.FORCE:

target/cortex-m0/release/libmicrobit.a: .FORCE
	@RUSTFLAGS='--sysroot ./sysroot -C opt-level=3' cargo build --target cortex-m0 --release --verbose
	
target/startup_NRF51822.o: contrib/startup_NRF51822.S
	${AS} \
		-mcpu=cortex-m0 \
		-mthumb \
		contrib/startup_NRF51822.S \
		-o target/startup_NRF51822.o

target/bin: target/cortex-m0/release/libmicrobit.a contrib/NRF51822.ld target/startup_NRF51822.o
	${GCC} \
		-mcpu=cortex-m0 \
		-mthumb \
		-Wl,--gc-sections \
		-Tcontrib/NRF51822.ld \
		target/startup_NRF51822.o \
		target/cortex-m0/release/libmicrobit.a \
		-Wl,--start-group \
		-lnosys \
		-lgcc \
		-lc \
		-Wl,--end-group \
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

clean:
	rm -rf target
