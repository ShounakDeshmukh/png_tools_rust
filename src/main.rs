use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;

use crc::{Crc, CRC_32_ISO_HDLC};

mod Headers;
mod constants;
use Headers::IHDR;

const CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
fn main() {
    let Ok(mut buffer) = read_image() else {panic!("Problem opening the file")};
    read_headers(&buffer);

    if buffer[0..=7] != PNG_SIGNATURE {
        panic!("Invalid file please choose a PNG")
    } else {
        {
            // rmove png signature and ihdr size (8+4)

            buffer = buffer[12..].to_vec();

            if ihdr_checksum(&buffer) {
                eprintln!("INVALID IHDR CHECKSUM")
            } else {
                // remove IHDR ascii bytes
                buffer = buffer[4..].to_vec();
                read_ihdr(&buffer);

                buffer = buffer[..].to_vec();
            }
            println!("{buffer:?}");
        }
    }
}

fn read_headers(buffer: &Vec<u8>) {
    for (idx, i) in buffer.windows(4).enumerate() {
        println!("{}  {:?}", idx, i)
    }
}

fn read_image() -> Result<Vec<u8>, io::Error> {
    let f = File::open("pngsuite/basn0g16.png")?;
    let mut reader: BufReader<File> = BufReader::new(f);
    let mut buffer: Vec<u8> = Vec::new();
    reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn ihdr_checksum(buffer: &Vec<u8>) -> bool {
    let checksum_input: Vec<u8> = buffer[0..(13 + 4) as usize].to_vec();
    print!("{:?}", checksum_input);
    let checksum: [u8; 4] = CRC.checksum(&checksum_input).to_be_bytes();
    println!("{:?}", checksum);
    checksum != buffer[17..21]
}

fn read_ihdr(buffer: &Vec<u8>) -> IHDR {
    let img_h: u32 = u32::from_be_bytes(buffer[0..4].try_into().unwrap());
    let img_w: u32 = u32::from_be_bytes(buffer[4..8].try_into().unwrap());
    let bit_dep: u8 = u8::from_be(buffer[8]);
    let colo_type: u8 = u8::from_be(buffer[9]);
    let compress_meth: u8 = u8::from_be(buffer[10]);
    let filter_meth: u8 = u8::from_be(buffer[11]);
    let interlace_meth: u8 = u8::from_be(buffer[12]);
    IHDR::new()
}

fn read_data() {}
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
