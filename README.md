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
SCK → D13 (PB5) - Serial Clock
CSN → D9 (PB1) - Chip Select (active low)

Status Monitoring Pins (GDO - General Digital Output)
GDO0 → D2 (PD2) - RX data, TX data, or other signals
GDO2 → D3 (PD3) - Chip status / sync / carrier sense

Note: GDO1 is shared with MISO on most CC1101 modules
```

