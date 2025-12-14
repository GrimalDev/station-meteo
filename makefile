-include .env

export PROJECT := station-meteo
export ARCH := avr-unknown-gnu-atmega328
export PROGRAMMER := arduino
export SERIAL ?= /dev/tty.usbmodem1201
# flash OR eeprom
export MEMTYPE := flash
export AVR_CPU_FREQUENCY_HZ=16000000


.PHONY: dev
dev:
	@cargo +nightly-2024-12-10 build -Z build-std=core --target avr-unknown-gnu-atmega328

.PHONY: build
build:
	@cargo +nightly-2024-12-10 build -Z build-std=core --target avr-unknown-gnu-atmega328 --release

.PHONY: flash
flash:
	@./scripts/flash.sh

.PHONY: monitor
monitor:
	@serialconsole r -b 57600 /dev/tty.usbmodem11201

.PHONY: flash-monitor
flash-monitor:
	@$(MAKE) flash
	@sleep 2
	@$(MAKE) monitor

.PHONY: clean
clean:
	@cargo clean

.PHONY: doc
doc:
	@cargo doc --open

