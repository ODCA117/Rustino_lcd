#![no_std]
#![no_main]

use panic_halt as _;
use rustino_lcd::hd44780_lcd::LcdDriverTC1601A2;


#[arduino_hal::entry]
fn main() -> ! {
    /* Assign pins */
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let rs = pins.d4.into_output().downgrade();
    let rw = pins.d5.into_output().downgrade();
    let e = pins.d6.into_output().downgrade();

    let db7 = pins.d13.into_output().downgrade();
    let db6 = pins.d12.into_output().downgrade();
    let db5 = pins.d11.into_output().downgrade();
    let db4 = pins.d10.into_output().downgrade();

    /* Create driver */
    let dp = [db4, db5, db6, db7];
    let mut driver = LcdDriverTC1601A2::new(rs, rw, e, dp);

    ufmt::uwriteln!(&mut serial, "Initialize driver").unwrap();
    driver.initialize();

    /* Use Character generator */
    /* The CGRam have address 0-7, lets print all of them */
    ufmt::uwriteln!(&mut serial, "Print CGs").unwrap();
    for i in 0..8 {
        driver.write(i);
    }

    /* Modify each of the characters */
    /* The character is a 5*8 pixel character.
     * Each element in the vector represent the row,
     *   element 0 represents top 5 pixels,
     *   element 7 represents bottom 5 pixels,
     * */
    ufmt::uwriteln!(&mut serial, "Set CG pattern").unwrap();
    /* Checkerd pattern */
    let pattern = [
        0b10101,
        0b01010,
        0b10101,
        0b01010,
        0b10101,
        0b01010,
        0b10101,
        0b01010];

    driver.set_cg_character(0, &pattern);

    /* Thick Arrow */
    let pattern = [
        0b00100,
        0b01110,
        0b11111,
        0b01110,
        0b01110,
        0b01110,
        0b01110,
        0b01110];
    driver.set_cg_character(1, &pattern);

    /* Heart */
    let pattern = [
        0b00000,
        0b00000,
        0b01010,
        0b11111,
        0b01110,
        0b00100,
        0b00000,
        0b00000];
    driver.set_cg_character(2, &pattern);

    /* Left half smily */
    let pattern = [
        0b00000,
        0b00111,
        0b00111,
        0b00111,
        0b00000,
        0b11000,
        0b01111,
        0b00000];
    driver.set_cg_character(3, &pattern);

    /* right half smily */
    let pattern = [
        0b00000,
        0b11100,
        0b11100,
        0b11100,
        0b00000,
        0b00011,
        0b11110,
        0b00000];
    driver.set_cg_character(4, &pattern);

    /* House */
    let pattern = [
        0b00000,
        0b00000,
        0b00000,
        0b00100,
        0b01010,
        0b10001,
        0b10001,
        0b11111];
    driver.set_cg_character(5, &pattern);

    /* Hour glas */
    let pattern = [
        0b00000,
        0b11111,
        0b10001,
        0b01010,
        0b00100,
        0b01010,
        0b10001,
        0b11111];
    driver.set_cg_character(6, &pattern);

    /* Stickman */
    let pattern = [
        0b01100,
        0b01100,
        0b00100,
        0b01110,
        0b10101,
        0b00100,
        0b01010,
        0b11011];
    driver.set_cg_character(7, &pattern);

    /* Remove cursor and blink */
    driver.toggle_blink(false);
    driver.toggle_cursor(false);

    loop { }
}

