// required-features: chrono

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#[path = "../common.rs"]
mod common;

use chrono::{NaiveDate, NaiveDateTime};
use common::*;
use defmt::assert;
use embassy_executor::Spawner;
use embassy_stm32::rcc::RtcClockSource;
use embassy_stm32::rtc::{Rtc, RtcConfig};
use embassy_time::{Duration, Timer};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = config();

    #[cfg(feature = "stm32h755zi")]
    {
        use embassy_stm32::rcc::Lse;
        config.rcc.lse = Some(Lse::Oscillator);
        config.rcc.rtc_mux = Some(RtcClockSource::LSE);
    }
    #[cfg(not(feature = "stm32h755zi"))]
    {
        use embassy_stm32::time::Hertz;
        config.rcc.lse = Some(Hertz(32_768));
        config.rcc.rtc = Some(RtcClockSource::LSE);
    }

    let p = embassy_stm32::init(config);
    info!("Hello World!");

    let now = NaiveDate::from_ymd_opt(2020, 5, 15)
        .unwrap()
        .and_hms_opt(10, 30, 15)
        .unwrap();

    let mut rtc = Rtc::new(p.RTC, RtcConfig::default());

    rtc.set_datetime(now.into()).expect("datetime not set");

    info!("Waiting 5 seconds");
    Timer::after(Duration::from_millis(5000)).await;

    let then: NaiveDateTime = rtc.now().unwrap().into();
    let seconds = (then - now).num_seconds();

    info!("measured = {}", seconds);

    assert!(seconds > 3 && seconds < 7);

    info!("Test OK");
    cortex_m::asm::bkpt();
}
