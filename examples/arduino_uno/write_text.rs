#![no_std]
#![no_main]

use panic_halt as _;
use rustino_lcd::hd44780_lcd::LcdDriverTC1601A2;


#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    /* Sets pins to use on the arduino UNO */
    let rs = pins.d4.into_output().downgrade();
    let rw = pins.d5.into_output().downgrade();
    let e = pins.d6.into_output().downgrade();
    let db7 = pins.d13.into_output().downgrade();
    let db6 = pins.d12.into_output().downgrade();
    let db5 = pins.d11.into_output().downgrade();
    let db4 = pins.d10.into_output().downgrade();
    let dp = [db4, db5, db6, db7];

    /* Create the driver */
    let mut driver = LcdDriverTC1601A2::new(rs, rw, e, dp);

    ufmt::uwriteln!(&mut serial, "Initialize driver").unwrap();
    driver.initialize();

    ufmt::uwriteln!(&mut serial, "Write 'Hello world' to lcd").unwrap();
    driver.write_ascii("Hello World");
    loop { }
}

