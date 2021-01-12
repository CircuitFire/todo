use frames::crossterm::{style::Color, event::KeyCode};

pub trait AsBuffer{
    fn as_buffer(&self) -> Box<[u8]>;
}

impl AsBuffer for bool{
    fn as_buffer(&self) -> Box<[u8]>{
        Box::new([*self as u8])
    }
}

impl AsBuffer for u32{
    fn as_buffer(&self) -> Box<[u8]>{
        Box::new([((*self >> 24) as u8), ((*self >> 16) as u8), ((*self >> 8) as u8), (*self as u8)])
    }
}

impl AsBuffer for char{
    fn as_buffer(&self) -> Box<[u8]>{
        let temp = *self as u32;
        Box::new([((temp >> 24) as u8), ((temp >> 16) as u8), ((temp >> 8) as u8), (temp as u8)])
    }
}

impl AsBuffer for u8{
    fn as_buffer(&self) -> Box<[u8]>{
        Box::new([*self])
    }
}

impl AsBuffer for KeyCode {
    fn as_buffer(&self) -> Box<[u8]>{
        let mut buf = Box::new([0;5]);
        self.into_buffer(&mut buf[..]);
        buf
    }
}

impl AsBuffer for Color {
    fn as_buffer(&self) -> Box<[u8]>{
        match *self{
            Color::Rgb{r, g, b} => Box::new([r, g, b]),
            _ => Box::new([255, 255, 255]),
        }
    }
}

pub trait IntoBuffer{
    fn into_buffer(&self, buffer: &mut [u8]);
}

impl IntoBuffer for KeyCode {
    fn into_buffer(&self, buf: &mut [u8]) {
        match self {
            KeyCode::Backspace       => (),
            KeyCode::BackTab         => buf[0] = 1,
            KeyCode::Char(character) => {
                buf[0] = 2;
                buf[1..].clone_from_slice(&character.as_buffer());
            },
            KeyCode::Delete          => buf[0] = 3,
            KeyCode::Down            => buf[0] = 4,
            KeyCode::End             => buf[0] = 5,
            KeyCode::Enter           => buf[0] = 6,
            KeyCode::Esc             => buf[0] = 7,
            KeyCode::F(num)          => {
                buf[0] = 8;
                buf[1..2].clone_from_slice(&num.as_buffer());
            },
            KeyCode::Home            => buf[0] = 9,
            KeyCode::Insert          => buf[0] = 10,
            KeyCode::Left            => buf[0] = 11,
            KeyCode::Null            => buf[0] = 12,
            KeyCode::PageDown        => buf[0] = 13,
            KeyCode::PageUp          => buf[0] = 14,
            KeyCode::Right           => buf[0] = 15,
            KeyCode::Tab             => buf[0] = 16,
            KeyCode::Up              => buf[0] = 17,
        }
    }
}

impl IntoBuffer for Color {
    fn into_buffer(&self, buf: &mut [u8]) {
        match *self{
            Color::Rgb{r, g, b} => {
                buf[0] = r;
                buf[1] = g;
                buf[2] = b;
            },
            _ => {
                buf[0] = 255;
                buf[1] = 255;
                buf[2] = 255;
            },
        }
    }
}

pub fn buff_to_bool(buffer: &[u8]) -> bool {
    buffer[0] != 0
}

pub fn buff_to_u32(buffer: &[u8]) -> u32 {
    (buffer[0] as u32) << 24 | (buffer[1] as u32) << 16 | (buffer[2] as u32) << 8 | buffer[3] as u32
}

pub fn buff_to_char(buffer: &[u8]) -> Option<char> {
    let temp = (buffer[0] as u32) << 24 | (buffer[1] as u32) << 16 | (buffer[2] as u32) << 8 | buffer[3] as u32;
    std::char::from_u32(temp)
}

pub fn buff_to_u8(buffer: &[u8]) -> u8 {
    buffer[0]
}

pub fn buff_to_keycode(buffer: &[u8]) -> Option<KeyCode> {
    match buffer[0] {
        0  => Some(KeyCode::Backspace),
        1  => Some(KeyCode::BackTab),
        2  => Some(KeyCode::Char(buff_to_char(&buffer[1..])?)),
        3  => Some(KeyCode::Delete),
        4  => Some(KeyCode::Down),
        5  => Some(KeyCode::End),
        6  => Some(KeyCode::Enter),
        7  => Some(KeyCode::Esc),
        8  => Some(KeyCode::F(buff_to_u8(&buffer[1..]))),
        9  => Some(KeyCode::Home),
        10 => Some(KeyCode::Insert),
        11 => Some(KeyCode::Left),
        12 => Some(KeyCode::Null),
        13 => Some(KeyCode::PageDown),
        14 => Some(KeyCode::PageUp),
        15 => Some(KeyCode::Right),
        16 => Some(KeyCode::Tab),
        17 => Some(KeyCode::Up),
        _  => None,
    }
}

pub fn buff_to_color(buffer: &[u8]) -> Color {
    Color::Rgb{r: buffer[0], g: buffer[1], b: buffer[2]}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bool_buff() {
        let test = true;
        let test_buf = test.as_buffer();
        assert_eq!(test, buff_to_bool(&test_buf));
    }

    #[test]
    fn u32_buff() {
        let test = 35;
        let test_buf = test.as_buffer();
        assert_eq!(test, buff_to_u32(&test_buf));
    }

    #[test]
    fn char_buff() {
        let test = 't';
        let test_buf = test.as_buffer();
        assert_eq!(test, buff_to_char(&test_buf).unwrap());
    }

    #[test]
    fn u8_buff() {
        let test = 12;
        let test_buf = test.as_buffer();
        assert_eq!(test, buff_to_u8(&test_buf));
    }

    #[test]
    fn keycode_1() {
        let test = KeyCode::Backspace;
        let test_buf = test.as_buffer();
        assert_eq!(test, buff_to_keycode(&test_buf).unwrap());
    }

    #[test]
    fn keycode_2() {
        let test = KeyCode::Char('h');
        let test_buf = test.as_buffer();
        assert_eq!(test, buff_to_keycode(&test_buf).unwrap());
    }

    #[test]
    fn keycode_3() {
        let test = KeyCode::F(7);
        let test_buf = test.as_buffer();
        assert_eq!(test, buff_to_keycode(&test_buf).unwrap());
    }

    #[test]
    fn color() {
        let test = Color::Rgb{r: 255, b: 255, g: 255};
        let test_buf = test.as_buffer();
        assert_eq!(test, buff_to_color(&test_buf));
    }
}