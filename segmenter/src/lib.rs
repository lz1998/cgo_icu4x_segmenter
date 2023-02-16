extern crate alloc;

use alloc::boxed::Box;
use alloc::ffi::CString;
use alloc::vec;
use alloc::vec::{IntoIter, Vec};
use core::ffi::c_char;
use core::mem::forget;

mod segment;

pub struct CharBreakPoints(IntoIter<usize>);

fn breakpoints_utf8_to_char(text: &str, breakpoints: Vec<usize>) -> Vec<usize> {
    let mut char_breakpoints = vec![0];

    let mut utf8_len = 0;
    let mut chars = text.chars().enumerate();
    for breakpoint in breakpoints.into_iter().skip(1) {
        for (char_index, c) in chars.by_ref() {
            utf8_len += c.len_utf8();
            if breakpoint == utf8_len {
                char_breakpoints.push(char_index + 1);
                break;
            }
        }
    }
    char_breakpoints
}

#[cfg(feature = "sentence")]
#[no_mangle]
pub extern "C" fn break_sentence(a: *mut c_char) -> *mut CharBreakPoints {
    let s = unsafe { CString::from_raw(a) };
    let breakpoints = segment::get_sentence_breakpoints(s.to_str().unwrap());
    let s = CharBreakPoints(breakpoints_utf8_to_char(s.to_str().unwrap(), breakpoints).into_iter());
    Box::into_raw(Box::new(s))
}

#[cfg(feature = "word")]
#[no_mangle]
pub extern "C" fn break_word(a: *mut c_char) -> *mut CharBreakPoints {
    let s = unsafe { CString::from_raw(a) };
    let breakpoints = segment::get_word_breakpoints(s.to_str().unwrap());
    let s = CharBreakPoints(breakpoints_utf8_to_char(s.to_str().unwrap(), breakpoints).into_iter());
    Box::into_raw(Box::new(s))
}

#[no_mangle]
pub extern "C" fn next_break(a: *mut CharBreakPoints) -> i32 {
    let mut char_break_points = unsafe { Box::from_raw(a) };
    let breakpoint = char_break_points.0.next().map(|a| a as i32).unwrap_or(-1);
    forget(char_break_points);
    breakpoint
}

#[no_mangle]
pub extern "C" fn free_char_break_points(char_break_points: *mut CharBreakPoints) {
    let _ = unsafe { Box::from_raw(char_break_points) };
}
