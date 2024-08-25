#![no_std]

use embedded_hal_async::spi::{ErrorType, SpiBus};
use smart_leds::RGB8;

const PATTERNS: [u8; 4] = [0b1000_1000, 0b1000_1110, 0b1110_1000, 0b1110_1110];

/// The order of the colors
pub enum ColorOrder {
    RGB,
    GRB,
}

/// N = 12 * NUM_LEDS
pub struct Ws2812<SPI: SpiBus<u8>, const N: usize> {
    spi: SPI,
    data: [u8; N],
    color_order: ColorOrder,
}

impl<SPI: SpiBus<u8>, const N: usize> Ws2812<SPI, N> {
    pub fn new(spi: SPI) -> Self {
        Self { spi, data: [0; N], color_order: ColorOrder::RGB }
    }

    pub fn set_color_order(&mut self, color_order: ColorOrder) {
        self.color_order = color_order;
    }

    pub async fn write(
        &mut self,
        iter: impl Iterator<Item = RGB8>,
    ) -> Result<(), <SPI as ErrorType>::Error> {
        for (led_bytes, RGB8 { r, g, b }) in self.data.chunks_mut(12).zip(iter) {
            let colors = match self.color_order {
                ColorOrder::RGB => [r, g, b],
                ColorOrder::GRB => [g, r, b],
            };
            for (i, mut color) in colors.into_iter().enumerate() {
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
