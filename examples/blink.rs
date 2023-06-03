#![no_std]
#![no_main]
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1xx_hal::{
    gpio::{Input, Pin},
    pac,
    prelude::*,
};

const MAX_DEL: u32 = 10_0000_u32;
const MIN_DEL: u32 = 2_5000_u32;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut gpiob = dp.GPIOB.split();
    let mut gpioe = dp.GPIOE.split();
    let mut led = gpiob.pb5.into_push_pull_output(&mut gpiob.crl);
    let button = gpioe.pe4.into_pull_up_input(&mut gpioe.crl);

    let mut delay_val = MAX_DEL;
    led.set_high();
    loop {
        delay_val = loop_delay(delay_val, &button);
        led.toggle();
    }
}
fn loop_delay<const N: char, const P: u8, MODE>(mut del: u32, but: &Pin<N, P, Input<MODE>>) -> u32 {
    for _ in 1..del {
        if but.is_low() {
            del -= MIN_DEL;
            if del < MIN_DEL {
                del = MAX_DEL;
            }
            return del;
        }
    }
    del
}
