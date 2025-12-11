#!/bin/bash

function check_serial() {
    if [ ! -c "${SERIAL}" ]; then
        echo "Serial file does not exist '${SERIAL}'..."
        exit 1
    fi
}

function check_elf() {
    if [ ! -f "${ELF}" ]; then
        echo "No ELF found at '${ELF}'..."
        exit 1
    fi
}

function flash() {
    avrdude -p "${ARCH}" -c "${PROGRAMMER}" -P "${SERIAL}" -U "${MEMTYPE}:w:${ELF}:e"
}

###
### Execution
###
export ELF="target/${ARCH}/release/station-meteo.elf"
check_serial
check_elf
flash
