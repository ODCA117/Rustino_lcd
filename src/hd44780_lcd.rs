
use panic_halt as _;
use embedded_hal::digital::v2::PinState;

use crate::HD44780Error::DriverError;

const CLEAR_DISPLAY: u8 = 0x01;
const RETURN_HOME: u8 = 0x02;
const ENTRY_MODE: u8 = 0x04;
const DISPLAY_CONTROL: u8 = 0x08;
const CURSOR_OR_DISPLAY_SHIFT: u8 = 0x10;
const FUNCTION_SET: u8 = 0x20;
const SET_CGRAM: u8 = 0x40;
const SET_DDRAM: u8 = 0x80;

/* Entry mode options */
const AUTO_DECREMENT_CURSOR: u8 = 0x00;
const AUTO_INCREMENT_CURSOR: u8 = 0x10;
const AUTO_DISPLAY_SHIFT_OFF: u8 =  0x0;
const AUTO_DISPLAY_SHIFT_ON: u8 = 0x1;

/* Display control options */
const DISPLAY_ON: u8 = 0x4;
const DISPLAY_OFF: u8 = 0x0;
const CURSOR_ON: u8 = 0x2;
const CURSOR_OFF: u8 = 0x0;
const BLINK_ON: u8 = 0x1;
const BLINK_OFF: u8 = 0x0;

/* Cursor or display shift options */
const SHIFT_DISPLAY: u8 = 0x8;
const SHIFT_CURSOR: u8 = 0x0;
const SHIFT_RIGHT: u8 = 0x4;
const SHIFT_LEFT: u8 = 0x0;

/* Function set options */
const EIGHT_BIT_INTERFACE: u8 = 0x10;
const FOUR_BIT_INTERFACE: u8= 0x00;
const ONE_ROW: u8 = 0x00;
const TWO_ROW: u8 = 0x08;
const FIVEBYEIGTH: u8 = 0x00;
const FIVEBYTEN: u8 = 0x04;


pub struct LcdDriverTC1601A2<P> {
    rs: P,
    rw: P,
    e: P,
    dp: [P;4],

    /* State of lcd */
    display_control_mode: u8,
    function_set: u8,
    entry_mode: u8,
}

impl<P> LcdDriverTC1601A2<P>
    where P: embedded_hal::digital::v2::OutputPin,
{
    pub fn new (rs: P, rw: P, e: P, dp: [P;4]) -> Self {
        let display_control_mode = DISPLAY_ON | CURSOR_ON | BLINK_ON;
        let function_set = FOUR_BIT_INTERFACE | TWO_ROW | FIVEBYEIGTH;
        let entry_mode = AUTO_INCREMENT_CURSOR | AUTO_DISPLAY_SHIFT_OFF;
        Self { rs, rw, e, dp, display_control_mode, function_set, entry_mode}
    }

    pub fn initialize(&mut self) {
        /* Need time to ensure LCD have started */
        arduino_hal::delay_ms(500);
        let _ = self.rs.set_low();
        let _ = self.e.set_low();
        let _ = self.rw.set_low();

        /* Initialization sequence from documentation */
        let command = 0x3;
        self.command(command);
        arduino_hal::delay_ms(10);
        self.command(command);
        arduino_hal::delay_ms(10);
        self.command(command);
        arduino_hal::delay_ms(10);
        let command = 0x2;
        self.command(command);

        /* Set function (fourbit interface, one row, five by eight font */
        let command = FUNCTION_SET | FOUR_BIT_INTERFACE | TWO_ROW | FIVEBYEIGTH;
        self.command(command);
        arduino_hal::delay_ms(10);

        /* Turn display on with no cursor or blinking */
        let command = DISPLAY_CONTROL | DISPLAY_OFF | CURSOR_OFF | BLINK_OFF;
        self.command(command);
        arduino_hal::delay_ms(10);

        let command = CLEAR_DISPLAY;
        self.command(command);
        arduino_hal::delay_ms(10);

        /* Set entry mode to increment cursor */
        let command = ENTRY_MODE | AUTO_INCREMENT_CURSOR | AUTO_DISPLAY_SHIFT_OFF;
        self.command(command);
        arduino_hal::delay_ms(10);

        let command = DISPLAY_CONTROL | DISPLAY_ON | CURSOR_ON | BLINK_ON;
        self.command(command);
        arduino_hal::delay_ms(10);

        self.return_home()
    }

    pub fn reset_display(&mut self) {
        /* Clear display and return home */
        let command = CLEAR_DISPLAY;
        self.command(command);
    }

    pub fn return_home(&mut self) {
        let command = RETURN_HOME;
        self.command(command);
        /* Add extra delay for return home */
        arduino_hal::delay_ms(2);
    }

    pub fn toggle_display(&mut self, on: bool) {

        let options = if on {
           DISPLAY_ON | (self.display_control_mode & 0x3 )
        } else {
           DISPLAY_OFF | (self.display_control_mode & 0x3 )
        };
        let command = DISPLAY_CONTROL | options;
        self.display_control_mode = options;
        self.command(command);
    }

    pub fn toggle_cursor(&mut self, on: bool) {
        let options = if on {
            CURSOR_ON | (self.display_control_mode & 0x5)
        } else {
            CURSOR_OFF | (self.display_control_mode & 0x5)
        };
        let command = DISPLAY_CONTROL | options;
        self.command(command);
        self.display_control_mode = options;
    }

    pub fn toggle_blink(&mut self, on: bool) {
        let options = if on {
            BLINK_ON | (self.display_control_mode & 0x6)
        } else {
            BLINK_OFF | (self.display_control_mode & 0x6)
        };
        let command = DISPLAY_CONTROL | options;
        self.command(command);
        self.display_control_mode = options;
    }

    /* Seem to be able to hold 80 chars before it loops back around */
    pub fn write_ascii(&mut self, string: &str) -> Result<(), DriverError>{
        if !string.is_ascii() {
            return Err(DriverError::GenericError);
        }

        if string.len() > 80 {
            return Err(DriverError::GenericError);
        }

        for c in string.as_bytes() {
            self.write(*c);
        }

        Ok(())
    }

    pub fn shift_display_left(&mut self) {
        let command = CURSOR_OR_DISPLAY_SHIFT | SHIFT_DISPLAY | SHIFT_LEFT;
        self.command(command);
    }

    pub fn shift_display_right(&mut self) {
        let command = CURSOR_OR_DISPLAY_SHIFT | SHIFT_DISPLAY | SHIFT_RIGHT;
        self.command(command);
    }

    pub fn set_cursor_pos(&mut self, address: u8) -> Result<(), DriverError> {
        if address > 0x80 {
            return Err(DriverError::GenericError);
        }
        let command = SET_DDRAM | address;
        self.command(command);
        Ok(())
    }

    pub fn set_cg_character(&mut self, cg_index: u8, pattern: &[u8;8]) -> Result<(), DriverError> {
        if cg_index >= 8 {
            return Err(DriverError::GenericError);
        }

        let address = cg_index * 8;
        for (i, &row) in pattern.iter().enumerate() {
            let cmd = SET_CGRAM | (address + i as u8);
            self.command(cmd);
            self.write(row);
        }
        Ok(())
    }

    pub fn write(&mut self, data: u8) {
        self.send(data, PinState::High, PinState::Low);
    }

    pub fn command(&mut self, command: u8) {
        self.send(command, PinState::Low, PinState::Low);
    }


    fn send(&mut self, value: u8, rs: PinState, rw: PinState) {
        let _ = self.rs.set_state(rs);
        let _ = self.rw.set_state(rw);

        self.send4bits(value >> 4);
        self.send4bits(value & 0xF);
    }

    fn send4bits(&mut self, value: u8) {
        let mut pin_index = 0x01;

        for pin in self.dp.iter_mut() {
            if (value & pin_index) > 0 {
                pin.set_state(PinState::High);
            } else {
                pin.set_state(PinState::Low);
            }
            pin_index = pin_index << 1;
        }
        self.enable_pulse();
    }

    fn enable_pulse(&mut self) {
        let _ = self.e.set_low();
        arduino_hal::delay_us(1);
        let _ = self.e.set_high();
        arduino_hal::delay_us(40);
        let _ = self.e.set_low();
        arduino_hal::delay_ms(40);
    }
}

