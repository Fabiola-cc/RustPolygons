use std::fmt;

#[derive(Debug)]
pub struct Framebuffer {
    width: usize,
    height: usize,
    data: Vec<u8>,
    current_color: (u8, u8, u8),
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Framebuffer {
        let size = width * height * 3;
        let data = vec![0; size];
        Framebuffer {
            width,
            height,
            data,
            current_color: (0, 0, 0),
        }
    }

    pub fn set_background_color(&mut self, hex: u32) {
        let color = Self::hex_to_rgb(hex);
        self.data.chunks_mut(3).for_each(|pixel| {
            pixel[0] = color.0;
            pixel[1] = color.1;
            pixel[2] = color.2;
        });
    }

    pub fn set_current_color(&mut self, hex: u32) {
        self.current_color = Self::hex_to_rgb(hex);
    }

    pub fn point(&mut self, x: f32, y: f32) {
        let x = x.round() as isize;
        let y = y.round() as isize;
        if x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
            return;
        }
        let x = x as usize;
        let y = y as usize;
        self.set_pixel(x, y, self.current_color.0, self.current_color.1, self.current_color.2);
    }

    pub fn clear(&mut self) {
        self.data.fill(0);
    }

    fn set_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        if x >= self.width || y >= self.height {
            return;
        }
        let flipped_y = self.height - 1 - y;  // Invertir el valor de y
        let index = (flipped_y * self.width + x) * 3;
        self.data[index] = r;
        self.data[index + 1] = g;
        self.data[index + 2] = b;
    }

    pub fn get_pixel(&self, x: isize, y: isize) -> Option<(u8, u8, u8)> {
        if x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        let flipped_y = self.height - 1 - y;  // Invertir el valor de y
        let index = (flipped_y * self.width + x) * 3;
        Some((self.data[index], self.data[index + 1], self.data[index + 2]))
    }

    pub fn render_buffer(&self, filename: &str) -> std::io::Result<()> {
        crate::bmp::write_bmp_file(filename, &self.data, self.width, self.height)
    }

    fn hex_to_rgb(hex: u32) -> (u8, u8, u8) {
        (
            ((hex >> 16) & 0xFF) as u8,
            ((hex >> 8) & 0xFF) as u8,
            (hex & 0xFF) as u8,
        )
    }
}

impl fmt::Display for Framebuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = (y * self.width + x) * 3;
                write!(f, "({}, {}, {}) ", self.data[index], self.data[index + 1], self.data[index + 2])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_framebuffer_creation() {
        let fb = Framebuffer::new(10, 10);
        assert_eq!(fb.width, 10);
        assert_eq!(fb.height, 10);
        assert_eq!(fb.data.len(), 300); // 10 * 10 * 3
    }

    #[test]
    fn test_set_get_pixel() {
        let mut fb = Framebuffer::new(10, 10);
        fb.set_pixel(5, 5, 255, 0, 0);
        let pixel = fb.get_pixel(5, 5);
        assert_eq!(pixel, Some((255, 0, 0)));

        let out_of_bounds_pixel = fb.get_pixel(15, 15);
        assert_eq!(out_of_bounds_pixel, None);
    }

    #[test]
    fn test_color() {
        let mut fb = Framebuffer::new(10, 10);
        fb.set_current_color(0xffee00);
        assert_eq!(fb.current_color, (255, 238, 0));
    }

    #[test]
    fn test_clear() {
        let mut fb = Framebuffer::new(10, 10);
        fb.set_background_color(0xFF0000);
        fb.clear();
        assert_eq!(fb.get_pixel(0, 0), Some((0, 0, 0)));
        assert_eq!(fb.get_pixel(9, 9), Some((0, 0, 0)));
    }

    #[test]
    fn test_set_background_color() {
        let mut fb = Framebuffer::new(10, 10);
        fb.set_background_colo
    }
}