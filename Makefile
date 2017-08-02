.include "config.mk"

build: ${TARGET_DIR}/combined.hex

${TARGET_DIR}/cortex-m0/release/libmicrobit.a: .FORCE
.export XARGO_RUST_SRC
	xargo build -j1 --target cortex-m0 --release --verbose
	
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

.FORCE:
