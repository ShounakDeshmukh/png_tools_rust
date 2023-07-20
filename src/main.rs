use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;

use crc::{Crc, CRC_32_ISO_HDLC};

const CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
// 89 50 4E 47 0D 0A 1A 0A
const IHDR_HEX: [u8; 4] = [73, 72, 68, 82];
const LENGTH_IHDR: u8 = 13;
fn main() -> io::Result<()> {
    let f = File::open("pngsuite/xhdn0g08.png")?;
    let mut reader: BufReader<File> = BufReader::new(f);
    let mut buffer: Vec<u8> = Vec::new();
    reader.read_to_end(&mut buffer)?;
    println!("{}", buffer.capacity());

    if buffer[0..=7] != PNG_SIGNATURE {
        print!("Invalid file please choose a PNG")
    } else {
        // rmove png signature
        buffer = buffer[8..].to_vec();
        // remove bytes signifying length of ihdr
        buffer = buffer[4..].to_vec();

        let checksum_input: Vec<u8> = buffer[0..(LENGTH_IHDR + 4) as usize].to_vec();
        print!("{:?}", checksum_input);

        let checksum: [u8; 4] = CRC.checksum(&checksum_input).to_be_bytes();
        println!("{:?}", checksum);

        if checksum != buffer[17..21] {
            print!("INVALID IHDR CHECKSUM")
        }
    }
    println!("{:?}", buffer);

    Ok(())
}

// let length_ihdr: u32 = u32::from_be_bytes(buffer[0..4].try_into().unwrap());
// buffer = buffer[4..].to_vec();
// println!("{}", buffer.capacity());
// if buffer[0..4] != IHDR_HEX {
//     print!("INVALID HEADER ORDER OR HEX")
// } else {
//     buffer.drain(0..4);
//     let image_height: u32 = u32::from_be_bytes(buffer[0..4].try_into().unwrap());
//     let image_width: u32 = u32::from_be_bytes(buffer[4..8].try_into().unwrap());
//     buffer.drain(0..8);
//     println!("{}", image_height);
//     println!("{}", image_width);
// }

//     IHDR must be the first chunk; it contains (in this order) the image's
//         width (4 bytes)
//         height (4 bytes)
//         bit depth (1 byte, values 1, 2, 4, 8, or 16)
//         color type (1 byte, values 0, 2, 3, 4, or 6)
//         compression method (1 byte, value 0)
//         filter method (1 byte, value 0)
//         interlace method (1 byte, values 0 "no interlace" or 1 "Adam7 interlace") (13 data bytes total)
