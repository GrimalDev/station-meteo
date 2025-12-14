# station-meteo

---

## CC1101 Module Connections to Arduino Uno R3

```
Power (⚠️ CRITICAL: 3.3V ONLY! CC1101 is NOT 5V tolerant!)
VCC → 3.3V (use level shifter or 3.3V Arduino)
GND → GND

SPI Bus (Hardware SPI)
MOSI → D11 (PB3) - Master Out Slave In
MISO → D12 (PB4) - Master In Slave Out (shared with GDO1)
SCK  → D13 (PB5) - Serial Clock
CSN  → D9  (PB1) - Chip Select (active low)

Status Monitoring Pins (GDO - General Digital Output)
GDO0 → D2 (PD2) - Sync Word Indicator (pulses HIGH during packet TX/RX)
GDO2 → D3 (PD3) - Chip Ready (LOW when ready, HIGH when busy)

Note: GDO1 is shared with MISO on most CC1101 modules
```

## GDO Pin Usage

The code configures the GDO pins for useful monitoring without requiring code changes:

- **GDO0 (D2)**: Pulses HIGH when a sync word is transmitted or received. This indicates active packet transmission/reception. You can connect an LED (with resistor) to visualize TX/RX activity.

- **GDO2 (D3)**: Chip Ready signal (active LOW). Goes LOW when the CC1101 is ready for operation. Can be used to monitor chip state or trigger external circuits.

These signals can be monitored with:
- LEDs for visual indication
- Logic analyzer for debugging
- Oscilloscope for timing analysis
- External control circuits (PA/LNA switching, etc.)

