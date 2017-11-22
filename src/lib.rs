// Copyright 2017 Rohit Joshi <rohit.c.joshi@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// ebcdic-rs
///
/// It provides methods to convert ebcsic to ascii and back.
/// # Examples
/// ```
/// extern crate ebcdic;
/// use std::str;
/// use ebcdic::ebcdic::Ebcdic;
/// fn main() {
///     let ascii_str = "       IDENTIFICATION DIVISION.                                         00000010";
///     let mut ebcdic_bytes: [u8; 80] = [0; 80];
///     Ebcdic::ascii_to_ebcdic(ascii_str.as_bytes(), &mut ebcdic_bytes, 80, true);
///     let mut ascii_str_2: [u8; 80] = [0; 80];
///     Ebcdic::ebcdic_to_ascii(&ebcdic_bytes, &mut ascii_str_2, 80, false, true);
///     assert_eq!(ascii_str, str::from_utf8(&ascii_str_2).unwrap());
/// }
/// ```
pub mod ebcdic;
#[cfg(test)]
mod tests {
    use ebcdic::Ebcdic;
    use std::str;
    #[test]
    fn ascii_to_ebcdic() {
        let array_size = 80;
        let ascii_str = "       IDENTIFICATION DIVISION.                                         00000010";
        let mut ebcdic_bytes: [u8; 80] = [0u8; 80];
        Ebcdic::ascii_to_ebcdic(ascii_str.as_bytes(), &mut ebcdic_bytes, array_size, true);
        assert!(ebcdic_bytes.len() == 80);
        let mut ascii_str_2: [u8; 80] = [0u8; 80];
        Ebcdic::ebcdic_to_ascii(&ebcdic_bytes, &mut ascii_str_2, array_size, false, true);
        assert!(ascii_str_2.len() == array_size);
        assert_eq!(ascii_str, str::from_utf8(&ascii_str_2).unwrap());
        println!("Ascii Output:{}", str::from_utf8(&ascii_str_2).unwrap());
        println!("Ebcdic Output:{:?}", &ebcdic_bytes[..]);

    }
}
