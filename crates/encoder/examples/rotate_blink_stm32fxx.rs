#![no_std]
#![no_main]

pub use stm32f1xx_hal as hal;

use crate::hal::{pac, prelude::*}; // STM32F1 specific functions // When a panic occurs, stop the microcontroller

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use encoder::{Action};
#[allow(unused_imports)]
#[allow(clippy::single_component_path_imports)]
use panic_halt;

// This marks the entrypoint of our application. The cortex_m_rt creates some
// startup code before this, but we don't need to worry about this
#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut _rcc = dp.RCC.constrain();
    let mut gpiob = dp.GPIOB.split();

    let mut led_cw = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
    let mut led_btn = gpiob.pb13.into_push_pull_output(&mut gpiob.crh);
    let mut led_ccw = gpiob.pb14.into_push_pull_output(&mut gpiob.crh);

    let s1_pin = gpiob.pb6.into_pull_down_input(&mut gpiob.crl);
    let s2_pin = gpiob.pb7.into_pull_down_input(&mut gpiob.crl);
    let key_pin = gpiob.pb8.into_pull_down_input(&mut gpiob.crh);

    let rot_blink_time = 1000;
    let rot_pressed_blink_time = 10000;
    let click_time = 10000;

    let mut cw_t = 0;
    let mut ccw_t = 0;
    let mut click_t = 0;

    let mut encoder = encoder::EncoderInfallible::new(s1_pin, s2_pin, key_pin);
    loop {
        let action = encoder.update();

        match action {
            Action::None => {}
            Action::Cw => cw_t = rot_blink_time,
            Action::Ccw => ccw_t = rot_blink_time,
            Action::CwPressed => cw_t = rot_pressed_blink_time,
            Action::CcwPressed => ccw_t = rot_pressed_blink_time,
            Action::Click => click_t = click_time,
        }

        #[allow(unused_must_use)]
        fn pin_tick(pin: &mut impl OutputPin, t: &mut i32) {
            if *t > 0 {
                pin.set_high();
                *t -= 1;
            } else {
                pin.set_low();
            }
        }

        pin_tick(&mut led_cw, &mut cw_t);
        pin_tick(&mut led_btn, &mut click_t);
        pin_tick(&mut led_ccw, &mut ccw_t);
    }
}
