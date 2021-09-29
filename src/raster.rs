pub struct GreyscaleRaster {
    width: usize,
    height: usize,
    data: Vec<u8>
}

impl GreyscaleRaster {
    pub fn new(width: usize, height: usize) -> GreyscaleRaster {
        GreyscaleRaster {
            width,
            height,
            data: vec![0; width*height]
        }
    }

    pub fn get_size(&self) -> (usize, usize) {
        return (self.width, self.height)
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: u8) {
        self.data[y as usize*self.width + x as usize] = pixel;
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> u8 {
        self.data[y as usize*self.width + x as usize]
    }

    pub fn as_raw_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn debug_log(&self) {
        for y in 0..self.height {
            let mut line = String::from("");

            for x in 0..self.width {
                let pixel = self.get_pixel(x as u32, y as u32);
                let c = match pixel {
                    0..=31    => ' ',
                    32..=63   => '.',
                    64..=95   => '-',
                    96..=127  => '=',
                    128..=159 => '+',
                    160..=191 => '*',
                    192..=223 => '#',
                    224..=255 => '%',
                };
                line.push(c);
            }

            println!("{}", line);
        }
    }
}