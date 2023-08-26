#![no_std]

use embedded_hal_async::spi::{ErrorType, SpiBus};
use smart_leds::RGB8;

const PATTERNS: [u8; 4] = [0b1000_1000, 0b1000_1110, 0b1110_1000, 0b1110_1110];

/// N = 12 * NUM_LEDS
pub struct Ws2812<SPI: SpiBus<u8>, const N: usize> {
    spi: SPI,
    data: [u8; N],
}

impl<SPI: SpiBus<u8>, const N: usize> Ws2812<SPI, N> {
    pub fn new(spi: SPI) -> Self {
        Self { spi, data: [0; N] }
    }

    pub async fn write(
        &mut self,
        iter: impl Iterator<Item = RGB8>,
    ) -> Result<(), <SPI as ErrorType>::Error> {
        for (led_bytes, RGB8 { r, g, b }) in self.data.chunks_mut(12).zip(iter) {
            for (i, mut color) in [r, g, b].into_iter().enumerate() {
                for ii in 0..4 {
                    led_bytes[i * 4 + ii] = PATTERNS[((color & 0b1100_0000) >> 6) as usize];
                    color <<= 2;
                }
            }
        }
        self.spi.write(&self.data).await?;
        let blank = [0_u8; 140];
        self.spi.write(&blank).await
    }
}
