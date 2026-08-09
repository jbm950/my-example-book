Example that connects 2 of the on board SPI peripherals (SPI1 as controller and
SPI2 as peripheral). The controller and peripheral exchange four bytes in
full-duplex mode, with RTT output verifying that each transmitted byte was
received correctly. Note this example relies exclusively on the PAC rather than
the HAL to support learning the low level implementation.

Connection Table
| Signal | Function | Arduino Header Pin | MCU Pin (Master / SPI1) | Morpho Connector Pin | MCU Pin (Slave / SPI2) |
|---|---|---|---|---|---|
| Chip Select | SPI_NSS | A2 | PA4 (GPIO out) | CN10-16 | PB12 (SPI2_NSS) |
| Clock | SPI_SCK | D13 | PA5 (SPI1_SCK) | CN10-30 | PB13 (SPI2_SCK) |
| Master In, Slave Out | SPI_MISO | D12 | PA6 (SPI1_MISO) | CN10-28 | PB14 (SPI2_MISO) |
| Master Out, Slave In | SPI_MOSI | D11 | PA7 (SPI1_MOSI) | CN10-26 | PB15 (SPI2_MOSI) |


                 STM32F446               
        ┌───────────────────────────────┐
        │                               │
        │   SPI1              SPI2      │
        │ Controller          Peripheral│
        │                               │
        │ PA5 ───── SCK ───── PB13      │
        │ PA6 ──── MISO ───── PB14      │
        │ PA7 ──── MOSI ───── PB15      │
        │                               │
        │ PA4 ───── CS ────── PB12      │
        │                               │
        └───────────────────────────────┘
