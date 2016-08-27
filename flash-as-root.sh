#!/bin/sh

SERNO=9900000037024e45006620080000004e0000000097969901
MNT=/mnt
HEX=target/combined.hex

mount -t msdos /dev/serno/${SERNO} ${MNT} || exit 1

if [ -f ${MNT}/details.txt ]; then
    cp ${HEX} ${MNT}
    umount ${MNT}
    echo "flashed"
    exit 0
else
    echo "Not correctly mounted"
    exit 1
fi
