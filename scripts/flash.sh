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
    # Extract AVR part name from ARCH (e.g., avr-unknown-gnu-atmega328 -> m328p)
    AVR_PART="${AVR_PART:-m328p}"
    avrdude -C ./avrdude.conf -p "${AVR_PART}" -c "${PROGRAMMER}" -P "${SERIAL}" -U "${MEMTYPE}:w:${ELF}:e"
}

###
### Execution
###
export ELF="target/${ARCH}/release/station-meteo.elf"
check_serial
check_elf
flash
