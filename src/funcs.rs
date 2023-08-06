//utility functions file like running text, clock display etc
// 8 digit 7 segment display
//currently obscolete because removed write_segments_raw element due to extern crate removal


//to do:
// change the counter from 9999 max to 99999999 max
//redo this whole file cuz its obsolete

pub const DISPLAY_COUNT: usize = 8;

use crate::mappings::SegmentBits;
use crate::{Brightness, DisplayState, TM1638Adapter};
use alloc::string::String;

//running text banner
pub fn display_text_banner_in_loop(adapter: &mut TM1638Adapter, text: &str, sleep_fn: &dyn Fn()) {
    adapter.set_display_state(DisplayState::ON);
    adapter.set_brightness(Brightness::L7);

    // remove dots
    let data = text.replace('.', " ");
    let data = TM1638Adapter::encode_string(&data);

    // +1 because the upper border in a range is exclusive
    // otherwise last char is lost!
    let to = (data.len() - DISPLAY_COUNT) + 1;

    // display this text over and over again
    loop {
        for x in 0..to {
            let data_slice = &data[x..(x + DISPLAY_COUNT)];
            adapter.write_segments_raw(data_slice, 0);
            sleep_fn();
        }
    }
}

// clock display in form hh:mm, ":" blinks
pub fn display_current_time_in_loop(
    adapter: &mut TM1638Adapter,
    tick_fn: &dyn Fn(),
    time_fn: &dyn Fn() -> (String, String),
) {
    adapter.set_display_state(DisplayState::ON);
    adapter.set_brightness(Brightness::L7);

    let mut show_dots = false;
    loop {
        let (l, r): (String, String) = (time_fn)();
        #[allow(clippy::iter_nth_zero)]
        let mut data: [u8; DISPLAY_COUNT] = [
            TM1638Adapter::encode_char(l.chars().nth(0).unwrap()),
            TM1638Adapter::encode_char(l.chars().nth(1).unwrap()),
            TM1638Adapter::encode_char(r.chars().nth(0).unwrap()),
            TM1638Adapter::encode_char(r.chars().nth(1).unwrap()),
        ];

        if show_dots {
            data[1] |= SegmentBits::SegPoint as u8;
        }

        adapter.write_segments_raw(&data, 0);

        (tick_fn)();

        show_dots = !show_dots;
    }
}


// Maximum value for counter
pub const STOPWATCH_MAX: u16 = 10_000;
// Counter from 0 to 9999.
pub fn display_stopwatch(adapter: &mut TM1638Adapter, sleep_fn: &dyn Fn(), to: u16, blink: bool) {
    adapter.set_display_state(DisplayState::ON);
    adapter.set_brightness(Brightness::L7);

    let mut show_dot = false;
    // 0 to 9999
    for i in 0..to {
        let mut data = TM1638Adapter::encode_number(i);
        if blink && show_dot {
            data[1] |= SegmentBits::SegPoint as u8;
        }
        adapter.write_segments_raw(&data, 0);
        show_dot = !show_dot;
        sleep_fn();
    }
}

// Timer from x to 0
pub fn display_timer(adapter: &mut TM1638Adapter, sleep_fn: &dyn Fn(), from_val: u16) {
    adapter.set_display_state(DisplayState::ON);
    adapter.set_brightness(Brightness::L7);

    let mut show_dot = false;
    // 0 to 9999
    for i in 0..(from_val + 1) {
        let i = from_val - i;
        let data = TM1638Adapter::encode_number(i);
        adapter.write_segments_raw(&data, 0);
        show_dot = !show_dot;
        sleep_fn();
    }

    // blinking with just zeros to show that timer is done
    for i in 0..4 {
        let data = if i % 2 == 0 {
            [0; 4]
        } else {
            [TM1638Adapter::encode_digit(0); 4]
        };
        adapter.write_segments_raw(&data, 0);
        sleep_fn();
    }
    adapter.clear();
}