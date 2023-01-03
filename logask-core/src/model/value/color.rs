use std::convert::TryInto;

#[derive(Eq, PartialEq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Color::new(r, g, b)
    }
}

impl From<Color> for (u8, u8, u8) {
    fn from(val: Color) -> Self {
        (val.r, val.g, val.b)
    }
}

impl From<[u8; 3]> for Color {
    fn from([r, g, b]: [u8; 3]) -> Self {
        Color::new(r, g, b)
    }
}

impl From<u32> for Color {
    fn from(color_code: u32) -> Self {
        let r = (color_code >> (8 * 2)) & 0xFF;
        let g = (color_code >> (8 * 1)) & 0xFF;
        let b = color_code & 0xFF;

        Color::new(r as u8, g as u8, b as u8)
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Self { r, g, b }
    }
}

mod tests {
    use super::*;

    #[rstest::rstest(given, expect,
        case(0x00123456, (0x12, 0x34, 0x56))
    )]
    pub fn color_can_be_created_from_u32(given: u32, expect: (u8, u8, u8)) {
        let color: Color = given.into();
        let created: (u8, u8, u8) = color.into();

        assert_eq!(created, expect);
    }
}
