#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::prelude::*;
use arduino_hal::spi;
use cc1101::lowlevel;
use embedded_hal_bus::spi::ExclusiveDevice;

// ============================================================================
// CONFIGURATION - Edit these values to match your setup
// ============================================================================

struct Config {
    radio: RadioConfig,
    serial_baud: u32,
    tx_interval_ms: u32,
}

struct RadioConfig {
    frequency_mhz: f32,
}

const CONFIG: Config = Config {
    radio: RadioConfig {
        // Supported bands: 300-348 MHz, 387-464 MHz, 779-928 MHz
        frequency_mhz: 432.500,
    },
    serial_baud: 57600,
    tx_interval_ms: 5000,
};

// ============================================================================
// END OF CONFIGURATION
// ============================================================================

impl RadioConfig {
    const fn frequency_registers(&self) -> (u8, u8, u8) {
        let freq_hz = (self.frequency_mhz * 1_000_000.0) as u64;
        let freq_value = freq_hz * 65536 / 26_000_000;
        let freq0 = (freq_value & 0xFF) as u8;
        let freq1 = ((freq_value >> 8) & 0xFF) as u8;
        let freq2 = ((freq_value >> 16) & 0x3F) as u8;
        (freq0, freq1, freq2)
    }
}

fn init_radio<E, S>(radio: &mut lowlevel::Cc1101<S>,
                    config: &RadioConfig,
                    serial: &mut impl ufmt::uWrite)
    where S: embedded_hal::spi::SpiDevice<u8, Error = E>,
          E: core::fmt::Debug
{
    use lowlevel::registers::{Config, MDMCFG2, PKTCTRL0};
    use cc1101::{ModulationFormat, SyncCheck, LengthConfig};

    radio.write_cmd_strobe(lowlevel::registers::Command::SIDLE).unwrap();
    arduino_hal::delay_ms(10);

    let (freq0, freq1, freq2) = config.frequency_registers();
    radio.write_register(Config::FREQ0, freq0).unwrap();
    radio.write_register(Config::FREQ1, freq1).unwrap();
    radio.write_register(Config::FREQ2, freq2).unwrap();

    ufmt::uwriteln!(serial, "Frequency: {} MHz (0x{:02X}{:02X}{:02X})\r",
                    config.frequency_mhz as u32, freq2, freq1, freq0).ok();

    radio.write_register(Config::MDMCFG2,
        MDMCFG2::default()
            .mod_format(ModulationFormat::BinaryFrequencyShiftKeying.into())
            .sync_mode(SyncCheck::DISABLED.into())
            .bits()
    ).unwrap();

    radio.write_register(Config::PKTCTRL0,
        PKTCTRL0::default()
            .length_config(LengthConfig::VARIABLE.into())
            .bits()
    ).unwrap();

    radio.write_register(Config::PKTLEN, 255).unwrap();

    // Configure GDO pins for useful monitoring:
    // GDO0 (D2): Sync word sent/received - pulses high during packet transmission
    // GDO2 (D3): Chip ready signal - low when chip is ready
    radio.write_register(Config::IOCFG0, 0x06).unwrap();  // Sync word sent/received
    radio.write_register(Config::IOCFG2, 0x29).unwrap();  // Chip ready (active low)

    ufmt::uwriteln!(serial, "GDO0: Sync word indicator (D2)\r").ok();
    ufmt::uwriteln!(serial, "GDO2: Chip ready signal (D3)\r").ok();
}

fn verify_radio<E, S>(radio: &mut lowlevel::Cc1101<S>,
                      serial: &mut impl ufmt::uWrite) -> bool
    where S: embedded_hal::spi::SpiDevice<u8, Error = E>,
          E: core::fmt::Debug
{
    match (radio.read_register(lowlevel::registers::Status::PARTNUM),
           radio.read_register(lowlevel::registers::Status::VERSION)) {
        (Ok(partnum), Ok(version)) => {
            ufmt::uwriteln!(serial, "CC1101 Part: {}, Ver: {}\r", partnum, version).ok();
            true
        }
        _ => {
            ufmt::uwriteln!(serial, "ERROR: Cannot read CC1101\r").ok();
            false
        }
    }
}

fn transmit_pulse<E, S>(radio: &mut lowlevel::Cc1101<S>)
    where S: embedded_hal::spi::SpiDevice<u8, Error = E>,
          E: core::fmt::Debug
{
    radio.write_cmd_strobe(lowlevel::registers::Command::SIDLE).unwrap();
    arduino_hal::delay_ms(10);

    radio.write_cmd_strobe(lowlevel::registers::Command::STX).unwrap();
    arduino_hal::delay_ms(100);

    radio.write_cmd_strobe(lowlevel::registers::Command::SIDLE).unwrap();
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, CONFIG.serial_baud);

    ufmt::uwriteln!(&mut serial, "Weather Station Starting...\r").unwrap_infallible();
    ufmt::uwriteln!(&mut serial, "Frequency: {} MHz\r", CONFIG.radio.frequency_mhz as u32).unwrap_infallible();
    ufmt::uwriteln!(&mut serial, "Baud Rate: {}\r", CONFIG.serial_baud).unwrap_infallible();

    let (spi, _) = arduino_hal::Spi::new(
        dp.SPI,
        pins.d13.into_output(),  // SCK
        pins.d11.into_output(),  // MOSI
        pins.d12.into_pull_up_input(),  // MISO
        pins.d10.into_output(),  // SS (not used, but required by HAL)
        spi::Settings::default(),
    );

    let cs = pins.d9.into_output();  // CS pin for CC1101
    let spi_device = ExclusiveDevice::new_no_delay(spi, cs).unwrap();

    ufmt::uwriteln!(&mut serial, "SPI initialized\r").unwrap_infallible();

    let mut radio = lowlevel::Cc1101::new(spi_device).unwrap();

    if !verify_radio(&mut radio, &mut serial) {
        loop {}
    }

    ufmt::uwriteln!(&mut serial, "Configuring radio...\r").unwrap_infallible();

    init_radio(&mut radio, &CONFIG.radio, &mut serial);

    ufmt::uwriteln!(&mut serial, "Config done!\r").unwrap_infallible();
    ufmt::uwriteln!(&mut serial, "Transmitting every {} ms\r", CONFIG.tx_interval_ms).unwrap_infallible();

    loop {
        ufmt::uwriteln!(&mut serial, "TX\r").unwrap_infallible();
        transmit_pulse(&mut radio);
        arduino_hal::delay_ms(CONFIG.tx_interval_ms);
    }
}
