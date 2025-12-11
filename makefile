-include .env

export PROJECT := station-meteo
export ARCH := avr-unknown-gnu-atmega328
export PROGRAMMER := arduino
export SERIAL ?= /dev/fill-serial-input
# flash OR eeprom
export MEMTYPE := flash
export AVR_CPU_FREQUENCY_HZ=16000000


.PHONY: build
build:
	@cargo build -Z build-std=core --target avr-unknown-gnu-atmega328 --release

.PHONY: flash
flash:
	@./scripts/flash.sh

.PHONY: clean
clean:
	@cargo clean

.PHONY: doc
doc:
	@cargo doc --open
