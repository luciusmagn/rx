use crate::session::SessionCoords;

use arrayvec::ArrayVec;
use rgx::kit::Rgba8;
use arrayvec::ArrayVec;

pub struct Palette {
    pub colors: ArrayVec<[Rgba8; 256]>,
    pub hover: Option<Rgba8>,
    pub cellsize: f32,
    pub height: usize,
    pub x: f32,
    pub y: f32,
}

impl Palette {
    pub fn new(cellsize: f32, height: usize) -> Self {
        Self {
            colors: ArrayVec::new(),
            hover: None,
            cellsize,
            height,
            x: 0.,
            y: 0.,
        }
    }

    pub fn add(&mut self, color: Rgba8) {
        if !self.colors.contains(&color) {
            self.colors.push(color);
        }
    }

    pub fn gradient(&mut self, colorstart: Rgba8, colorend: Rgba8, number: usize) {
        fn blend_component(start: u8, end: u8, coef: f32) -> u8 {
            (start as f32 * (1.0 - coef) + end as f32 * coef).round() as u8
        }

        let step: f32 = 1.0 / ((number - 1) as f32);
        for i in 0..number {
            let coef = i as f32 * step;
            let color: Rgba8 = Rgba8 {
                r: blend_component(colorstart.r, colorend.r, coef),
                g: blend_component(colorstart.g, colorend.g, coef),
                b: blend_component(colorstart.b, colorend.b, coef),
                a: blend_component(colorstart.a, colorend.a, coef),
            };

            self.colors.push(color);
        }
    }

    pub fn clear(&mut self) {
        self.colors.clear();
    }

    pub fn size(&self) -> usize {
        self.colors.len()
    }

    pub fn handle_cursor_moved(&mut self, p: SessionCoords) {
        let (x, y) = (p.x, p.y);
        let mut x = x as i32 - self.x as i32;
        let mut y = y as i32 - self.y as i32;
        let cellsize = self.cellsize as i32;
        let size = self.size() as i32;
        let height = self.height as i32;
        let columns = (self.size() as f32 / self.height as f32).ceil() as i32;

        let width = if size > height {
            cellsize * columns
        } else {
            cellsize
        };
        let height = i32::min(size, height) * cellsize;

        if x >= width || y >= height || x < 0 || y < 0 {
            self.hover = None;
            return;
        }

        x /= cellsize;
        y /= cellsize;

        let index = y + x * (height / cellsize);

        self.hover = if index < size {
            // We index from the back because the palette is reversed
            // before it is displayed, due to the Y axis pointing up,
            // where as the palette is created starting at the top
            // and going down.
            Some(self.colors[self.size() - index as usize - 1])
        } else {
            None
        };
    }
}
