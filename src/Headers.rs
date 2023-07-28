pub struct Header {
    length: Box<[u8]>,
    signature: Box<[u8]>,
    data: Box<[u8]>,
    checksum: Box<[u8]>,
}

impl Header {
    pub fn new() -> Header {
        Header {
            length: todo!(),
            signature: todo!(),
            data: todo!(),
            checksum: todo!(),
        }
    }
}

pub struct IHDR {
    image_height: u32,
    image_width: u32,
    bit_depth: u8,
    color_type: u8,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8,
}

impl IHDR {
    pub fn new() -> IHDR {
        IHDR {
            image_height: todo!(),
            image_width: todo!(),
            bit_depth: todo!(),
            color_type: todo!(),
            compression_method: todo!(),
            filter_method: todo!(),
            interlace_method: todo!(),
        }
    }
}
