use std::fmt;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;
use std::io::Result as IoResult;

extern crate byteorder;
use self::byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

//------------------------------------------------------------------- BmpHeader

pub struct BmpHeader {
    _b: char, // should be the 'B' character
    _m: char, // should be the 'M' character
    _file_size: u32,
    _reserved_1: u16,
    _reserved_2: u16,
    image_data_offset: u32,
}

impl BmpHeader {
    // pub fn new() -> BmpHeader {
    //     BmpHeader {
    //         _b: 'B',
    //         _m: 'M',
    //         _file_size: 0,
    //         _reserved_1: 0,
    //         _reserved_2: 0,
    //         image_data_offset: 54,
    //     }
    // }
    pub fn load<R>(file: &mut R) -> IoResult<BmpHeader> where R: ::std::io::Read {
        Ok(BmpHeader {
            _b: try!(file.read_u8()) as char,
            _m: try!(file.read_u8()) as char,
            _file_size: try!(file.read_u32::<LittleEndian>()),
            _reserved_1: try!(file.read_u16::<LittleEndian>()),
            _reserved_2: try!(file.read_u16::<LittleEndian>()),
            image_data_offset: try!(file.read_u32::<LittleEndian>()),
        })
    }
    pub fn save<W>(&self, file: &mut W) -> IoResult<()> where W: ::std::io::Write {
        try!(file.write_u8(self._b as u8));
        try!(file.write_u8(self._m as u8));
        try!(file.write_u32::<LittleEndian>(self._file_size));
        try!(file.write_u16::<LittleEndian>(self._reserved_1));
        try!(file.write_u16::<LittleEndian>(self._reserved_2));
        try!(file.write_u32::<LittleEndian>(self.image_data_offset));
        Ok(())
    }
}

//------------------------------------------------------------------- DibHeader

pub struct DibHeader {
    width: u32,
    height: u32,
    bpp: u16,
}

impl DibHeader {
    // pub fn new(width: u32, height: u32) -> DibHeader {
    //     DibHeader {
    //         width: width,
    //         height: height,
    //         bpp: 24,
    //     }
    // }
    pub fn load<R>(file: &mut R) -> IoResult<DibHeader> where R: ::std::io::Read {
        let _header_size = try!(file.read_u32::<LittleEndian>());
        let width = try!(file.read_u32::<LittleEndian>());
        let height = try!(file.read_u32::<LittleEndian>());
        let _color_planes = try!(file.read_u16::<LittleEndian>());
        let bpp = try!(file.read_u16::<LittleEndian>());
        let _compression = try!(file.read_u32::<LittleEndian>());
        let _image_size = try!(file.read_u32::<LittleEndian>());
        let _h_ppm = try!(file.read_i32::<LittleEndian>());
        let _v_ppm = try!(file.read_i32::<LittleEndian>());
        let _color_palette_size = try!(file.read_u32::<LittleEndian>());
        let _important_colors = try!(file.read_u32::<LittleEndian>());

        if bpp != 24 {
            panic!("bits per pixel was {} instead of 24", bpp);
        }

        Ok(DibHeader {
            width: width,
            height: height,
            bpp: bpp,
        })
    }
    pub fn save<W>(&self, file: &mut W) -> IoResult<()> where W: ::std::io::Write {
        try!(file.write_u32::<LittleEndian>(40)); // always write the 40 byte version
        try!(file.write_u32::<LittleEndian>(self.width));
        try!(file.write_u32::<LittleEndian>(self.height));
        try!(file.write_u16::<LittleEndian>(1)); // color planes
        try!(file.write_u16::<LittleEndian>(24)); // 24 bpp
        try!(file.write_u32::<LittleEndian>(0)); // compression method
        try!(file.write_u32::<LittleEndian>(self.width * self.height * 3)); // image size (3 bytes per pixel)
        try!(file.write_i32::<LittleEndian>(0)); // horizontal ppm
        try!(file.write_i32::<LittleEndian>(0)); // vertical ppm
        try!(file.write_u32::<LittleEndian>(0)); // color palette size
        try!(file.write_u32::<LittleEndian>(0)); // important colors
        Ok(())
    }
}

//----------------------------------------------------------------------- Pixel
#[derive(Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    pub fn new() -> Pixel {
        Pixel {
            r: 0u8,
            g: 0u8,
            b: 0u8,
        }
    }
    pub fn black() -> Pixel {
        Pixel {
            r: 0,
            g: 0,
            b: 0,
        }
    }
    pub fn white() -> Pixel {
        Pixel {
            r: 255,
            g: 255,
            b: 255,
        }
    }
    pub fn as_tuple(&self) -> (i32, i32, i32) {
        (self.r as i32, self.g as i32, self.b as i32)
    }
}

//------------------------------------------------------------------------- Bmp

pub struct Bmp {
    _bmp_header: BmpHeader,
    dib_header: DibHeader,
    pub pixels: Vec<Vec<Pixel>>,
}

impl Bmp {
    // pub fn new(width: u32, height: u32) -> Bmp {
    //     Bmp {
    //         _bmp_header: BmpHeader::new(),
    //         dib_header: DibHeader::new(width, height),
    //         pixels: Bmp::create_pixels(width as usize, height as usize),
    //     }
    // }
    fn create_pixels(width: usize, height: usize) -> Vec<Vec<Pixel>> {
        let mut pixels = Vec::with_capacity(width);
        for _ in 0..width {
            pixels.push(vec![Pixel::new(); height]);
        }
        pixels
    }
    pub fn width(&self) -> u32 {
        self.dib_header.width
    }
    pub fn height(&self) -> u32 {
        self.dib_header.height
    }
    pub fn load(path_str: &str) -> IoResult<Bmp> {
        // Into<Path>
        let path = Path::new(&path_str);
        let file = try!(File::open(&path));
        let mut file = ::std::io::BufReader::new(file);
        let bh = try!(BmpHeader::load(&mut file));
        let dh = try!(DibHeader::load(&mut file));

        let mut pixels = Bmp::create_pixels(dh.width as usize, dh.height as usize);

        try!(file.seek(SeekFrom::Start(bh.image_data_offset as u64)));
        for y in (0..dh.height as usize).rev()  { // BMPs are stored bottom up
            for x in 0..dh.width as usize {
                pixels[x][y] = Pixel {
                    r: try!(file.read_u8()),
                    g: try!(file.read_u8()),
                    b: try!(file.read_u8()),
                };
            }
        }

        Ok(Bmp {
            _bmp_header: bh,
            dib_header: dh,
            pixels: pixels,
        })
    }
    pub fn save(&self, path_str: &str) -> IoResult<()> {
        let path = Path::new(&path_str);
        match File::create(&path) {
            Ok(file) => {
                let mut file = ::std::io::BufWriter::new(file);
                try!(self._bmp_header.save(&mut file));
                try!(self.dib_header.save(&mut file));
                for y in (0..self.dib_header.height).rev() { // BMPs are stored bottom up
                    for x in 0..self.dib_header.width {
                        let pixel = &self.pixels[x as usize][y as usize];
                        try!(file.write_u8(pixel.r));
                        try!(file.write_u8(pixel.g));
                        try!(file.write_u8(pixel.b));
                    }
                }

                file.flush()
            },
            Err(err) => panic!(err)
        }
    }
}

impl Debug for Bmp {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "bits-per-pixel={}, width={}, height={}", self.dib_header.bpp, self.width(), self.height())
    }
}
