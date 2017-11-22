# ebcdic-rs 
[![Build Status](https://travis-ci.org/rohitjoshi/ebcdic-rs.svg?branch=master)](https://travis-ci.org/rohitjoshi/ebcdic-rs)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Provides methods to convert ebcsic to ascii format and back

# EBCDIC to ASCII
 It support EBCDIC format (character set IBM-1047) which is used in IBM's TPF and OS/390 operating systems.
 It additionally provides flags to replace non-printable ascii characters with
 space characters and also replace NEL (0x15) with '\n' characters.
 
 # ASCII to EBCDIC  
 It converts US-ASCII to  EBCDIC format (character set IBM-1047) .
 It additionally provides flags to replace '\n' character with NEL (0x15).

 # Code example

 ```
 extern crate ebcdic;
 use std::str;
 use ebcdic::Ebcdic;

 fn main() {
     /// ascii string
     let ascii_str = "       IDENTIFICATION DIVISION.                                         00000010";
     let mut ebcdic_bytes: [u8; 80] = [0; 80];
     //convert ascii to ebcdic with replace '\n' with 0x15 flag set to true
     Ebcdic::ascii_to_ebcdic(ascii_str.as_bytes(), &mut ebcdic_bytes, 80, true);
     
     let mut ascii_str_2: [u8; 80] = [0; 80];
     // convert ebcdic to ascii with replace 0x15 with '\n' set to true
     Ebcdic::ebcdic_to_ascii(&ebcdic_bytes, &mut ascii_str_2, 80, false, true);
     assert_eq!(ascii_str, str::from_utf8(&ascii_str_2).unwrap());
     
     
     let ebcdic_bytes: [u8; 20] = [
            0xe2,0x4b,0x40,0xc4,0x4b,
            0x40,0xc2,0xd6,0xd9,0xd4,
            0xc1,0xd5,0x40,0x40,0x40,
            0x40,0x40,0x40,0x40,0x40,
        ];
        let ascii = String::from("S. D. BORMAN        ");
        let mut ascii_str: [u8; 20] = [0u8; 20];
        Ebcdic::ebcdic_to_ascii(&ebcdic_bytes, &mut ascii_str, 20, false, true);
        println!("Ascii Output:{}", str::from_utf8(&ascii_str).unwrap());
        assert_eq!(ascii.as_bytes(), ascii_str);
 }
 ```


 
