#![deny(unsafe_code)]
#![no_std]
#![no_main]
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1xx_hal::{gpio::PinState, pac, prelude::*, timer::Tim3NoRemap};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clock = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split();

    let mut led_dot = gpioa
        .pa8
        .into_push_pull_output_with_state(&mut gpioa.crh, PinState::Low);
    let mut led_dash = gpioa
        .pa9
        .into_push_pull_output_with_state(&mut gpioa.crh, PinState::Low);

    let switch = gpioa.pa10.into_pull_up_input(&mut gpioa.crh);
    let key_dot = gpioa.pa11.into_pull_up_input(&mut gpioa.crh);
    let key_dash = gpioa.pa12.into_pull_up_input(&mut gpioa.crh);

    let mut afio = dp.AFIO.constrain();
    let c1 = gpioa.pa6.into_alternate_push_pull(&mut gpioa.crl);

    let mut pwm = dp
        .TIM3
        .pwm_hz::<Tim3NoRemap, _, _>(c1, &mut afio.mapr, 100.Hz(), &clock)
        .split();

    let max = pwm.get_max_duty();

    pwm.enable();
    pwm.set_duty(max);
    let mut delay = cp.SYST.delay(&clock);
    let mut delayed = true;
    loop {
        if key_dot.is_low() && switch.is_low() {
            pwm.disable();
            pwm.set_duty(max);
            led_dot.set_high();
            delay.delay_ms(80u8);
            delay.delay_ms(40u8);
            delayed = false;
        } else if key_dash.is_low() && switch.is_low() {
            pwm.disable();
            pwm.set_duty(max / 2);
            led_dash.set_high();
            delay.delay_ms(240u8);
            delay.delay_ms(120u8);
            delayed = false;
        } else {
            pwm.set_duty(max);
            led_dot.set_low();
            led_dash.set_low();
            pwm.enable();
            delay.delay_ms(80u8);
            delay.delay_ms(20u8);
            delayed = true;
        }
        if !delayed {
            pwm.set_duty(max);
            led_dot.set_low();
            led_dash.set_low();
            pwm.enable();
            delay.delay_ms(80u8);
            delay.delay_ms(20u8);
            delayed = true;
        }
    }
}
