/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::{convert::TryFrom, hash::Hash};

const MAX_CHAR_COUNT: usize = 25;
const MAX_CHAR_VALUE: u8    = 26;

const LUT_ASCII_TO_TS: [i8; 256] = {
    let mut a: [i8; 256] = [-1; 256];
    a[b'A' as usize] =  1; a[b'B' as usize] =  2; a[b'C' as usize] =  3;
    a[b'D' as usize] =  4; a[b'E' as usize] =  5; a[b'F' as usize] =  6;
    a[b'G' as usize] =  7; a[b'H' as usize] =  8; a[b'I' as usize] =  9;
    a[b'J' as usize] = 10; a[b'K' as usize] = 11; a[b'L' as usize] = 12;
    a[b'M' as usize] = 13; a[b'N' as usize] = 14; a[b'O' as usize] = 15;
    a[b'P' as usize] = 16; a[b'Q' as usize] = 17; a[b'R' as usize] = 18;
    a[b'S' as usize] = 19; a[b'T' as usize] = 20; a[b'U' as usize] = 21;
    a[b'V' as usize] = 22; a[b'W' as usize] = 23; a[b'X' as usize] = 24;
    a[b'Y' as usize] = 25; a[b'Z' as usize] = 26;

    a[b'a' as usize] =  1; a[b'b' as usize] =  2; a[b'c' as usize] =  3;
    a[b'd' as usize] =  4; a[b'e' as usize] =  5; a[b'f' as usize] =  6;
    a[b'g' as usize] =  7; a[b'h' as usize] =  8; a[b'i' as usize] =  9;
    a[b'j' as usize] = 10; a[b'k' as usize] = 11; a[b'l' as usize] = 12;
    a[b'm' as usize] = 13; a[b'n' as usize] = 14; a[b'o' as usize] = 15;
    a[b'p' as usize] = 16; a[b'q' as usize] = 17; a[b'r' as usize] = 18;
    a[b's' as usize] = 19; a[b't' as usize] = 20; a[b'u' as usize] = 21;
    a[b'v' as usize] = 22; a[b'w' as usize] = 23; a[b'x' as usize] = 24;
    a[b'y' as usize] = 25; a[b'z' as usize] = 26;

    a[b'-' as usize] = 0; a[b'_' as usize] = 0; a[b' ' as usize] = 0;

    a
};

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TinyString(u128);

// //////////////////// //
// // Main Functions // //
// //////////////////// //

impl TinyString {

    pub fn raw(&self) -> u128 {
        self.0
    }

    pub const fn len(&self) -> usize {
        let mut i = MAX_CHAR_COUNT;
        loop {
            if self.get_codepoint_at(i - 1) != 0 { return i; }
            i = i - 1;
            if i <= 0 { return 0; }
        }
    }

    pub fn as_string(&self) -> String {
        self.as_string_opts(b'_', false)
    }

    pub const fn get_ascii_at(&self, idx: usize) -> u8 {
        self.get_ascii_at_opts(idx, b'_', false)
    }

    pub const fn get_codepoint_at(&self, idx: usize) -> u8 {
        if idx >= MAX_CHAR_COUNT { panic!("Attempt to index out of range."); }
        ((self.0 >> idx*5) & 0x1F) as u8
    }
}

// ////////////////////////// //
// // Formatting with Opts // //
// ////////////////////////// //

impl TinyString {
    pub fn as_string_opts(&self, space: u8, caps: bool) -> String {
        let len = self.len();
        let mut bytes = Vec::<u8>::with_capacity(len);
        for i in 0..len { 
            let ascii = self.get_ascii_at_opts(i, space, caps);
            if ascii == 0 { continue; } // Skip null characters
            bytes.push(ascii); 
        }
        return unsafe { String::from_utf8_unchecked(bytes) };
    }

    pub const fn get_ascii_at_opts(&self, idx: usize, space: u8, caps: bool) -> u8 {
        match self.get_codepoint_at(idx) {
            0 => space,
            v => v + (if caps { 64 } else { 96 }) // tiny "a-z" maps with offset to ascii 
        }
    }
}

// //////////////////// //
// // Const try_fron // //
// //////////////////// //

impl TinyString {
    const fn try_from_raw(value: u128) -> Result<Self, &'static str> {
        if (value & 0xF000_0000_0000_0000_0000_0000_0000_0000) != 0 { return Err("TinyString reserved bits shouldn't be set"); }

        let tmp = TinyString(value);
        let mut i = 0;
        loop {
            if tmp.get_codepoint_at(i) >= 26 { 
                return Err("Raw contains invalid TinyString value"); 
            }
            i = i + 1;
            if i >= MAX_CHAR_COUNT { break; }
        }
        return Ok(tmp);
    }
    
    fn try_from_string(value: &String) -> Result<Self, &'static str> {
        TinyString::try_from_slice(value.as_bytes())
    }
    
    const fn try_from_str(value: &str) -> Result<Self, &'static str> {
        TinyString::try_from_slice(value.as_bytes())
    }
    
    fn try_from_vec(value: &Vec<u8>) -> Result<Self, &'static str> {
        TinyString::try_from_slice(value.as_slice())
    }
    
    const fn try_from_slice(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() > MAX_CHAR_COUNT { return Err("TinyString limited to 25 characters") }
        let result: u128 = 0;

        let mut i = 0;
        loop { // TODO convert to for loop when const_for is stable/nightly
            let val = LUT_ASCII_TO_TS[bytes[i] as usize];
            if val < 0 { return Err("Invalid character in TinyString"); }
            i = i + 1;
            if i >= bytes.len() { break; }
        }

        return Ok(TinyString(result));
    }
}

// //////////////// //
// // Const from // //
// //////////////// //

impl TinyString {
    pub const fn from_raw(value: u128) -> Self {
        expect(&Self::try_from_raw(value))
    }
    
    pub fn from_string(value: &String) -> Self {
        expect(&Self::try_from_string(value))
    }
    
    pub const fn from_str(value: &str) -> Self {
        expect(&Self::try_from_str(value))
    }
    
    pub fn from_vec(value: &Vec<u8>) -> Self {
        expect(&Self::try_from_vec(value))
    }
    
    pub const fn from_slice(value: &[u8]) -> Self {
        expect(&Self::try_from_slice(value))
    }
}

// ///////////// //
// // Traits! // //
// ///////////// //

impl Into<String> for TinyString {
    fn into(self) -> String {
        self.as_string()
    }
}

impl std::fmt::Debug for TinyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TinyString")
            .field("i", &self.0)
            .field("s", &self.as_string())
            .finish()
    }
}

impl std::fmt::Display for TinyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.as_string())
    }
}

impl TryFrom<u128> for TinyString {
    type Error = &'static str;
    fn try_from(value: u128) -> Result<Self, Self::Error> {
        let tmp = TinyString(value);
        for i in 0..MAX_CHAR_COUNT {
            if tmp.get_codepoint_at(i) <= MAX_CHAR_VALUE { continue; }
            return Err("TinyString contains invalid value"); 
        }
        return Ok(tmp);
    }
}

impl TryFrom<&String> for TinyString {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        TinyString::try_from_string(value)
    }
}

impl TryFrom<&str> for TinyString {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        TinyString::try_from_str(value)
    }
}

impl TryFrom<&Vec<u8>> for TinyString {
    type Error = &'static str;
    fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
        TinyString::try_from_vec(value)
    }
}

impl TryFrom<&[u8]> for TinyString {
    type Error = &'static str;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        TinyString::try_from_slice(value)
    }
}

// ///////////// //
// // Utility // //
// ///////////// //

const fn expect<T: Copy>(v: &Result<T, &'static str>) -> T {
    match v {
        Ok(v) => *v,
        Err(v) => panic!(*v)
    }
}