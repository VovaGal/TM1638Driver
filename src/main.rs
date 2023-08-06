#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]


//       A
//      ---
//  F  |   |  B
//      -G-
//  E  |   |  C
//      ---
//       D

//    0x3f,    // 0 0b00111111
//    0x06,    // 1 0b00000110
//    0x5b,    // 2 0b01011011
//    0x4f,    // 3 0b01001111
//    0x66,    // 4 0b01100110
//    0x6d,    // 5 0b01101101
//    0x7d,    // 6 0b01111101
//    0x07,    // 7 0b00000111
//    0x7f,    // 8 0b01111111
//    0x6f,    // 9 0b01101111
//    0x77,    // A 0b01110111
//    0x7c,    // b 0b01111100
//    0x39,    // C 0b00111001
//    0x5e,    // d 0b01011110
//    0x79,    // E 0b01111001
//    0x71,    // F 0b01110001
//    0x40,    // - 0b01000000
//    0x00,    // nothing 0b00000000
//    0x80     // dot 

//to do:
// there are a few bugs with different adresses, work out whats causing overflows 
// request the data sheet for the keyboard to tey to get that working in free time
//turn main into lib to have it as a proper driver crate




//#[macro_use]
//////////////extern crate alloc; = heap
// Import our enums/arrays for the symbol mappings to the 7 segment display
#[cfg(feature = "funcs")]
pub mod funcs;
pub mod mappings;
use core::str::Chars;

//use alloc::collections::btree_map::Values;
use embassy_stm32::{self, gpio::{Level, Output, Speed}, into_ref, Peripheral};
use embassy_stm32::gpio::{Flex, Pin, Pull, AnyPin};
use {defmt_rtt as _, panic_probe as _};
use crate::mappings::{NumCharBits, SpecialCharBits, CharBits};
//use alloc::boxed::Box;
//use alloc::rc::Rc;
//use core::cell::RefCell;
//use alloc::alloc::*;
////////use alloc::vec::Vec;  = heap
//use core::{fmt::{Debug}, num::bignum::Digit32};
//use defmt::export::u8;

//use cortex_m::itm::Aligned;
//use defmt::export::UnsignedInt;
use embassy_executor::Spawner;
//use crate::{Chip, Line, LineHandle, LineRequestFlags};

//+++++++++++++++++++++++++
// fix the no global memory allocator found but one is required; link to std or add `#[global_allocator]` to a static item that implements the GlobalAlloc trait
//error
//#[alloc_error_handler]
//fn my_allocator_error(_layout: Layout) -> ! {
//    panic!("out of memory");
//}
//#[global_allocator]
//static GLOBAL_ALLOCATOR: Allocator = Allocator;


//++++++++++++++++++++++++

//from the data sheet looks like the tm uses 16 display registers
//++++++ play with this from 6-16 this if cargo run doesnt work
pub const DISPLAY_REGISTERS_COUNT: usize = 16;

//GPIO pins
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum GpioPinValue {
    // Low.
    LOW,
    // High.
    HIGH,
}

impl From<u8> for GpioPinValue {
    fn from(x: u8) -> Self {
        if x == 0 {
            Self::LOW
        } else {
            Self::HIGH
        }
    }
}

pub struct TM1638Adapter <'d, const STB: usize, CLK: Pin, DIO: Pin>{
    stb: [Output<'d, AnyPin>; STB],
    clk: Output<'d, CLK>,
    dio: Flex<'d, DIO>,
    brightness: u8
}
fn init<'d>(pin: AnyPin) -> Output<'d, AnyPin>{
    into_ref!(pin);
    Output::new(pin, Level::High, Speed::Low)
}

//binary conversions
fn convert_to_bin(n: u8) -> [u8; 8]{
    return [(n>127) as u8, (n%128>63) as u8, (n%64>31) as u8, (n%32>15) as u8, (n%16>7) as u8, (n%8>=4)as u8, (n%4>=2) as u8, n%2]
}

// brightness and display state
#[repr(u8)]
#[derive(Debug)]
pub enum Brightness {
    // Brightness level 0 - Lowest
    L0 = 0b000,
    // Brightness level 1
    L1 = 0b001,
    // Brightness level 2
    L2 = 0b010,
    // Brightness level 3
    L3 = 0b011,
    // Brightness level 4
    L4 = 0b100,
    // Brightness level 5
    L5 = 0b101,
    // Brightness level 6
    L6 = 0b110,
    // Brightness level 7 - Highest 
    L7 = 0b111,
}

// display state
#[repr(u8)]
#[derive(Debug)]
pub enum DisplayState {
    // off
    OFF = 0b0000,
    // on
    ON = 0b1000,
}

// clear display
// to do


impl <'d, const STB: usize, CLK: Pin, DIO: Pin> TM1638Adapter <'d, STB, CLK, DIO> {
    pub fn new(s: [AnyPin; STB], c: CLK, d: DIO) -> Self {
        let clk = Output::new(c, Level::Low, Speed::Low);
        let mut dio = Flex::new(d);
        dio.set_as_input_output(Speed::Low, Pull::Up);
        Self { stb: s.map(init), clk, dio, brightness: DisplayState::ON as u8 | Brightness::L7 as u8}
    }

//+++++++++++
    pub fn set_display_state(&mut self, ds: DisplayState) {
        let old_brightness = self.brightness & 0b0000_0111;
        self.brightness = ds as u8 | old_brightness;
    }

//+++++++++++
    pub fn set_brightness(&mut self, brightness: Brightness) {
        // check if display is configured as on
        let display_on = self.brightness as u8 & DisplayState::ON as u8;
        self.brightness = display_on | brightness as u8;
    }

    pub fn command(&mut self, byte: [u8; 8]) {
        for i in 0..8 {
            match byte[7-i] {
                0 => { self.dio.set_low(); }
                1 => { self.dio.set_high(); }
                _ => {}
            }
            self.clk.set_high();
            self.clk.set_low();
        }
    }    
    pub fn listen(&mut self, s: [u8; STB]) {
        for i in 0..STB {
            self.stb[i].set_high();
            if s[i] == 1 {
                self.stb[i].set_low();
            }
        }
    }

    //10 is the first segment, 20 is centre
    pub fn select_address(&mut self, mut address: u8) {
        let mut displays: [u8; STB] = [0; STB];
        displays[address as usize / 16] = 1;
        self.listen(displays);
        address %= 16;
        self.command(convert_to_bin(192 + address));
    }

//+++++++ removed cuz heap
//    pub fn encode_number(num: u16) -> [u8; 4] {
//        let mut num = num % 10000;
//        let mut bits: [u8; 4] = [0; 4];
//        for i in 0..4 {
//            let digit = (num % 10) as u8;
//           bits[3 - i] = Self::encode_digit(digit);
//            num /= 10;
//        }
//        bits
//    }

//pub fn encode_digit(&mut self, address: u8, digit: u8, u32: bool) -> u8 {
//   self.select_address(address);
//        let digit = digit % 10;
//        if digit == 0 {
//            NumCharBits::Zero as u8
//        } else if digit == 1 {
//           NumCharBits::One as u8
//       } else if digit == 2 {
//            NumCharBits::Two as u8
//        } else if digit == 3 {
//            NumCharBits::Three as u8
//        } else if digit == 4 {
//            NumCharBits::Four as u8
//        } else if digit == 5 {
//            NumCharBits::Five as u8
//        } else if digit == 6 {
//            NumCharBits::Six as u8
//        } else if digit == 7 {
//            NumCharBits::Seven as u8
//       } else if digit == 8 {
//            NumCharBits::Eight as u8
//        }
//        // else if digit == 9 { NumCharBits::Nine as u8 }
//        else {
//            NumCharBits::Nine as u8
//        }
//    }
//    }
//    pub fn write_digit(&mut self, mut address: u8, text: &str){
//        self.select_address(address);
//        for c in text.chars() {
//            self.encode_digit(address, u8, false);
//            address+=2;
//            address%=16 * STB as u8;
//        }    


//    pub const fn encode_char(c: char) -> u8 {
    pub fn encode_char(&mut self, address: u8, chars: char) -> () {
        self.select_address(address);
        let nums: NumCharBits = match chars {
            // nums
            '0' => { NumCharBits::Zero }
            '1' => { NumCharBits::One }
            '2' => { NumCharBits::Two }
            '3' => { NumCharBits::Three }
            '4' => { NumCharBits::Four }
            '5' => { NumCharBits::Five }
            '6' => { NumCharBits::Six }
            '7' => { NumCharBits::Seven }
            '8' => { NumCharBits::Eight }
            '9' => { NumCharBits::Nine },
            _ => { NumCharBits::Zero }
        };
        let letters: CharBits = match chars {        
            // chars
            'A' => { CharBits::UpA }
            'C' => { CharBits::UpC }
            'E' => { CharBits::UpE }
            'F' => { CharBits::UpF }
            'G' => { CharBits::UpG }
            'H' => { CharBits::UpH }
            'I' => { CharBits::UpI }
            'J' => { CharBits::UpJ }
            'L' => { CharBits::UpL }
            'O' => { CharBits::UpO }
            'P' => { CharBits::UpP }
            'S' => { CharBits::UpS }
            'U' => { CharBits::UpU },
            _ => { CharBits::UpA }
        };
        let specials: SpecialCharBits = match chars { 
            // special chars
            ' ' => { SpecialCharBits::Space }
            '?' => { SpecialCharBits::QuestionMark }
            '-' => { SpecialCharBits::Minus }
            '_' => { SpecialCharBits::Underscore }
            '=' => { SpecialCharBits::Equals }
            '.' => { SpecialCharBits::Dot },
            _ => { SpecialCharBits::Space }
        };
        let n = nums as u8;
        let l = letters as u8;
        let spec = specials as u8;
        self.command(convert_to_bin(n));
        self.command(convert_to_bin(l));
        self.command(convert_to_bin(spec));
    }
    
    pub fn write_char(&mut self, mut address: u8, text: &str){
        self.select_address(address);
        for c in text.chars() {
            self.encode_char(address, c);
            address+=2;
            address%=16 * STB as u8;
        }
    }
}

struct Connector<
    'd,
    const STB: usize,
    CLK: Pin,
    DIO: Pin,
> {
    displays: TM1638Adapter<'d, STB, CLK, DIO>,
}

impl <
    'd,
    const STB: usize,
    CLK: Pin,
    DIO: Pin,
> Connector <'d, STB, CLK, DIO> {
    fn new(s: [AnyPin; STB], c: CLK, d: DIO) -> Self{
        let displays = TM1638Adapter::new(s, c, d);
        Self {displays}
    }


    //set adress to 20 for 4 numbers in the middle
    //interesting, if i set it to 15 and max 8 it overflows into _ letter state
    fn reset(&mut self){
        self.displays.set_brightness(Brightness::L4);     
        self.displays.set_display_state(DisplayState::ON);
        self.displays.write_char(22, "ACEF");
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let stbs = [p.PB9.degrade(), p.PB8.degrade()];
    let mut pin_setter = Connector::new(stbs, p.PB7, p.PB6);
    let mut led = Output::new(p.PC13, Level::Low, Speed::Low);  

        loop {
            pin_setter.reset();
           led.set_high();
            Timer::after(Duration::from_millis(100)).await;
           led.set_low();
            Timer::after(Duration::from_millis(50)).await;
        }
    }



















//use embassy_stm32::{
//    self,
//    gpio::{Level, Output, Speed},
//};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

//#[embassy_executor::main]
//async fn main(_spawner: Spawner) -> ! {
//    let p = embassy_stm32::init(Default::default());
//    let (display_clk_pin, display_dio_pin, display_stb_pin) = (B7, B6, B8);




//    let mut led = Output::new(p.PC13, Level::Low, Speed::Low);
//    let mut display_dio = Output::new(p.PA11, Level::Low, Speed::Low);
//    let mut display_stb = Output::new(p.PA15, Level::Low, Speed::Low);
//    let mut display_clk = Output::new(p.PA4, Level::Low, Speed::Low);        
//    loop {
//        if display_stb = Output::new(p.PA15, Level::High, Speed::Low) {
//            let mut display_stb = Output::new(p.PA15, Level::Low, Speed::Low);
//        }
//        display_dio.set_high();
//        Timer::after(Duration::from_millis(100)).await;
//        display_dio.set_low();
//        Timer::after(Duration::from_millis(50)).await;
//        led.set_high();
//        Timer::after(Duration::from_millis(200)).await;
//        led.set_low();
//        Timer::after(Duration::from_millis(50)).await;
//        if display_stb = Output::new(p.PA15, Level::Low, Speed::Low) {
//            let mut display_stb = Output::new(p.PA15, Level::High, Speed::Low);
//        }
//    }
//}
//+++++++++++++++++++++++++++++++
