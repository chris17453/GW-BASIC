//! Graphics module for GW-BASIC

use crate::error::{Error, Result};
use crate::value::Value;

/// Simulated screen buffer
pub struct Screen {
    width: usize,
    height: usize,
    buffer: Vec<Vec<char>>,
    cursor_x: usize,
    cursor_y: usize,
    fg_color: u8,
    bg_color: u8,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        Screen {
            width,
            height,
            buffer: vec![vec![' '; width]; height],
            cursor_x: 0,
            cursor_y: 0,
            fg_color: 7,
            bg_color: 0,
        }
    }

    pub fn cls(&mut self) {
        self.buffer = vec![vec![' '; self.width]; self.height];
        self.cursor_x = 0;
        self.cursor_y = 0;
    }

    pub fn locate(&mut self, row: usize, col: usize) -> Result<()> {
        if row >= self.height || col >= self.width {
            return Err(Error::RuntimeError(format!(
                "LOCATE position out of range: ({}, {})",
                row, col
            )));
        }
        self.cursor_y = row;
        self.cursor_x = col;
        Ok(())
    }

    pub fn color(&mut self, fg: Option<u8>, bg: Option<u8>) {
        if let Some(foreground) = fg {
            self.fg_color = foreground;
        }
        if let Some(background) = bg {
            self.bg_color = background;
        }
    }

    pub fn pset(&mut self, x: i32, y: i32, color: Option<u8>) -> Result<()> {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return Ok(()); // Silently ignore out-of-bounds
        }
        // In a real implementation, this would set a pixel
        // For now, we just mark the position with a character
        self.buffer[y as usize][x as usize] = '#';
        Ok(())
    }

    pub fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: Option<u8>) -> Result<()> {
        // Bresenham's line algorithm (simplified)
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;
        let mut x = x1;
        let mut y = y1;

        loop {
            self.pset(x, y, color)?;
            if x == x2 && y == y2 {
                break;
            }
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
        Ok(())
    }

    pub fn circle(&mut self, x: i32, y: i32, radius: i32, color: Option<u8>) -> Result<()> {
        // Midpoint circle algorithm (simplified)
        let mut dx = radius;
        let mut dy = 0;
        let mut err = 0;

        while dx >= dy {
            self.pset(x + dx, y + dy, color)?;
            self.pset(x + dy, y + dx, color)?;
            self.pset(x - dy, y + dx, color)?;
            self.pset(x - dx, y + dy, color)?;
            self.pset(x - dx, y - dy, color)?;
            self.pset(x - dy, y - dx, color)?;
            self.pset(x + dy, y - dx, color)?;
            self.pset(x + dx, y - dy, color)?;

            if err <= 0 {
                dy += 1;
                err += 2 * dy + 1;
            }
            if err > 0 {
                dx -= 1;
                err -= 2 * dx + 1;
            }
        }
        Ok(())
    }

    pub fn get_cursor(&self) -> (usize, usize) {
        (self.cursor_y, self.cursor_x)
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.height, self.width)
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self::new(80, 25)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_creation() {
        let screen = Screen::new(80, 25);
        assert_eq!(screen.width, 80);
        assert_eq!(screen.height, 25);
    }

    #[test]
    fn test_cls() {
        let mut screen = Screen::new(80, 25);
        screen.cls();
        assert_eq!(screen.cursor_x, 0);
        assert_eq!(screen.cursor_y, 0);
    }

    #[test]
    fn test_locate() {
        let mut screen = Screen::new(80, 25);
        screen.locate(10, 20).unwrap();
        assert_eq!(screen.cursor_y, 10);
        assert_eq!(screen.cursor_x, 20);
    }
}