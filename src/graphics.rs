// Graphic
use crate::rgb::RGB;
use crate::gba_color::GBAColor;
use crate::font::Font;

// Display Size : 240 * 160 (左上がx = 0, y =0)
pub struct Graphics {
    vram_address: u32,
    screen_x: u16,
    screen_y: u16,
    font: Font,
}

impl Graphics {
    pub fn new() -> Self {
        Graphics {
            vram_address: 0x06000000,
            screen_x: 240,
            screen_y: 160,
            font: Font::new(),
        }
    }

    pub fn draw_dot(&self, x: u16, y: u16, color: &RGB) {
        let offset: u32 = ((y * self.screen_x) + x) as u32;
        let vram: *mut u16 = (self.vram_address + (offset * 2)) as *mut u16;
        unsafe {
            *vram = color.convert_u16_color();
        }
    }

    pub fn draw_char(&self, ch:char, x:u16, y:u16, color:&RGB) {
        let char_data:[u8; 16] = self.font.get_character(ch);
        for index in 0..15 {
            let byte_data:u8 = char_data[index];
            let offset_y = index as u16;
            if (byte_data & 0x80) != 0x00 { self.draw_dot(x + 0, y + offset_y, color); }
            if (byte_data & 0x40) != 0x00 { self.draw_dot(x + 1, y + offset_y, color); }
            if (byte_data & 0x20) != 0x00 { self.draw_dot(x + 2, y + offset_y, color); }
            if (byte_data & 0x10) != 0x00 { self.draw_dot(x + 3, y + offset_y, color); }
            if (byte_data & 0x08) != 0x00 { self.draw_dot(x + 4, y + offset_y, color); }
            if (byte_data & 0x04) != 0x00 { self.draw_dot(x + 5, y + offset_y, color); }
            if (byte_data & 0x02) != 0x00 { self.draw_dot(x + 6, y + offset_y, color); }
            if (byte_data & 0x01) != 0x00 { self.draw_dot(x + 7, y + offset_y, color); }
        }
    }

    pub fn draw_string(&self, string:&str, x:u16, y:u16, color:&RGB) {
        let mut offset_x: u16 = 0;
        for character in string.chars() {
            self.draw_char(character, (x + offset_x), y, color);
            offset_x += self.font.font_width();
        }
    }

    pub fn width(&self) -> u16 {
        return self.screen_x;
    }

    pub fn height(&self) -> u16 {
        return self.screen_y;
    }
}