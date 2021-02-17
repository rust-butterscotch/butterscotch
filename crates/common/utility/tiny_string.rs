/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::{convert::TryFrom, hash::Hash};

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
pub struct TinyString<const WIDTH: usize>([u8; WIDTH]);

pub type TinyString18  = TinyString< 2>; //  3ch + 1b
pub type TinyString32  = TinyString< 4>; //  6ch + 2b
pub type TinyString48  = TinyString< 6>; //  9ch + 3b
pub type TinyString64  = TinyString< 8>; // 12ch + 4b
pub type TinyString80  = TinyString<10>; // 16ch + 0b
pub type TinyString96  = TinyString<12>; // 19ch + 1b
pub type TinyString112 = TinyString<14>; // 22ch + 2b
pub type TinyString128 = TinyString<16>; // 25ch + 3b

// //////////////////// //
// // Main Functions // //
// //////////////////// //

impl<const WIDTH: usize> Default for TinyString<WIDTH> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const WIDTH: usize> TinyString<WIDTH> {
    pub const fn new() -> Self {
        Self([0; WIDTH])
    }

    pub const fn len(&self) -> usize {
        let mut  i = WIDTH*8 - 1;
        loop {
            if self.get_codepoint_at(i) != 0 { return i + 1; }
            if i == 0 { return 0; }
            i = i - 1;
        }
    }

    pub const fn capacity() -> usize {
        WIDTH/5
    }

    pub fn as_string(&self) -> String {
        self.as_string_opts(b'_', false)
    }

    pub const fn get_ascii_at(&self, idx: usize) -> u8 {
        self.get_ascii_at_opts(idx, b'_', false)
    }

    pub const fn get_codepoint_at(&self, idx: usize) -> u8 {
        let bit     = idx*5;
        let offset  = bit/8;
        let diff    = bit - offset*8;
        let rdiff   = 10 - diff;
        let section = u16::from_le_bytes([self.0[offset], if offset +1 == WIDTH { 0 } else { self.0[offset + 1] }]);
        return ((section >> rdiff) & 0x1F) as u8;
    }

    pub const fn set_ascii_at(&mut self, idx: usize, character: u8) -> bool {
        let codepoint = LUT_ASCII_TO_TS[character as usize];
        if codepoint < 0 { return false; }
        self.set_codepoint_at(idx, character as u8);
        return true;
    }

    pub const fn set_codepoint_at(&mut self, idx: usize, character: u8) {
        let character = character as u16;

        let bit     = idx*5;
        let offset  = bit/8;
        let diff    = bit - offset*8;
        let rdiff   = 10 - diff;

        let section      = character << rdiff;
        let section_high = (section >> 8  ) as u8;
        let section_low  = (section & 0xFF) as u8;

        let mask      = !0x1F << rdiff;
        let mask_high = (mask >> 8  ) as u8;
        let mask_low  = (mask & 0xFF) as u8;
        
        self.0[offset] = (self.0[offset] & mask_high) | section_high;
        if offset + 1 != WIDTH {
            self.0[offset] = (self.0[offset] & mask_low) | section_low;
        }
    }

}

// ////////////////////////// //
// // Formatting with Opts // //
// ////////////////////////// //

impl<const WIDTH: usize> TinyString<WIDTH> {
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

impl<const WIDTH: usize> TinyString<WIDTH> {
    /*const fn try_from_raw(value: u128) -> Result<Self, &'static str> {
        if (value & 0xF000_0000_0000_0000_0000_0000_0000_0000) != 0 { return Err("TinyString reserved bits shouldn't be set"); }

        let tmp = Self(value);
        let mut i = 0;
        loop {
            if tmp.get_codepoint_at(i) > 26 { return Err("Raw contains invalid TinyString value"); }
            i = i + 1;
            if i >= MAX_CHAR_COUNT { break; }
        }
        return Ok(tmp);
    }

    fn try_from_vec(value: &Vec<u8>) -> Result<Self, &'static str> {
        TinyString::try_from_slice(value.as_slice())
    }*/
    
    pub fn try_from_string(value: &String) -> Result<Self, &'static str> {
        TinyString::try_from_ascii_slice(value.as_bytes())
    }
    
    pub const fn try_from_str(value: &str) -> Result<Self, &'static str> {
        TinyString::try_from_ascii_slice(value.as_bytes())
    }
    
    const fn try_from_ascii_slice(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() > Self::capacity() { return Err("TinyString limited to 25 characters") }

        let mut result = Self([0; WIDTH]);

        let mut i = 0; 
        let len = if WIDTH < bytes.len() { WIDTH } else { bytes.len() };
        loop {
            if !result.set_ascii_at(i, bytes[i]) { return Err("Invalid character in source string"); }
            if i >= len { break; }
            i = i + 1;
        }

        return Ok(result);
    }
}

// ///////////// //
// // Traits! // //
// ///////////// //

impl<const WIDTH: usize> Into<String> for TinyString<WIDTH> {
    fn into(self) -> String {
        self.as_string()
    }
}

impl<const WIDTH: usize> std::fmt::Debug for TinyString<WIDTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TinyString")
            .field("i", &self.0)
            .field("s", &self.as_string())
            .finish()
    }
}

impl<const WIDTH: usize> std::fmt::Display for TinyString<WIDTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.as_string())
    }
}

impl<const WIDTH: usize> TryFrom<&str> for TinyString<WIDTH> {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from_str(value)
    }
}
