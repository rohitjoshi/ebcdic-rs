# ebcdic - Provides methods to convert ebcsic to ascii format and back.


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
 }
 ```


 ## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Licensing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
