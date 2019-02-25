//! # nom-varint
//!
//! Parse varints (used by Minecraft, Celeste, and Protocol Buffers) using nom.

#[macro_use]
extern crate nom;

use nom::Err::*;
use nom::Needed::Unknown;

/// Parses a varint.
///
/// # Examples
///
/// ```
/// assert_eq!(nom_varint::take_varint(&[0x0b]), Ok((&[] as &[u8], 0x0b)));
/// ```
pub fn take_varint(i: &[u8]) -> nom::IResult<&[u8], usize> {
    let mut res: usize = 0;
    let mut count: usize = 0;
    let mut remainder = i;
    loop {
        let byte = match take!(remainder, 1) {
            Ok((rest, bytes)) => {remainder = rest; bytes[0]},
            Err(_) => return Err(Incomplete(Unknown)),
        };
        res += ((byte as usize) & 127) << (count * 7);
        count += 1;
        if (byte >> 7) == 0 {
            return Ok((remainder, res));
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn parse_varint_simple() {
        assert_eq!(super::take_varint(&[0x0b, 0x01, 0x02, 0x03]), Ok((b"\x01\x02\x03" as &[u8], 11)));
    }

    #[test]
    fn parse_varint_twobyte() {
        assert_eq!(super::take_varint(&[0x84, 0x02, 0x04, 0x05, 0x06]), Ok((b"\x04\x05\x06" as &[u8], 260)));
    }
}
