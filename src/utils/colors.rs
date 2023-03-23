/*
 * gerb
 *
 * Copyright 2022 - Manos Pitsidianakis
 *
 * This file is part of gerb.
 *
 * gerb is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * gerb is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with gerb. If not, see <http://www.gnu.org/licenses/>.
 */

use gtk::{gdk, glib};
use std::hash::Hash;

// [ref:needs_user_doc]
// [ref:needs_dev_doc]
#[derive(Clone, Debug, PartialEq, Eq, Copy, Hash, glib::Boxed)]
#[boxed_type(name = "Color", nullable)]
#[repr(transparent)]
pub struct Color(pub(crate) (u8, u8, u8, u8));

impl Color {
    // Constants re-exports
    pub const BLACK: Self = Self::from_hex("#000000");
    pub const BLUE: Self = Self::from_hex("#0000ff");
    pub const GREEN: Self = Self::from_hex("#00ff00");
    pub const RED: Self = Self::from_hex("#ff0000");
    pub const WHITE: Self = Self::from_hex("#ffffff");
    pub const TRANSPARENT: Self = Self::new_alpha(255, 255, 255, 255);

    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self::new_alpha(red, green, blue, 255)
    }

    pub const fn new_alpha(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self((red, green, blue, alpha))
    }

    pub const fn with_alpha(self, new_alpha: u8) -> Self {
        Self::new_alpha((self.0).0, (self.0).1, (self.0).2, new_alpha)
    }

    pub fn new_f64(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        Self((
            (red * 255.0) as u8,
            (green * 255.0) as u8,
            (blue * 255.0) as u8,
            (alpha * 255.0) as u8,
        ))
    }

    pub fn with_alpha_f64(self, new_alpha: f64) -> Self {
        Self::new_alpha(
            (self.0).0,
            (self.0).1,
            (self.0).2,
            (new_alpha * 255.0) as u8,
        )
    }

    pub fn try_parse(s: &str) -> Option<Self> {
        let val = gdk::RGBA::parse(s).ok()?;
        Some(Self::from(val))
    }

    pub fn try_from_hex(s: &str) -> Option<Self> {
        hex_color_to_rgb(s).map(|(r, g, b)| Self::new(r, g, b))
    }

    pub const fn from_hex(s: &str) -> Self {
        Self(hex(s))
    }

    #[inline(always)]
    pub fn red(&self) -> u8 {
        (self.0).0
    }

    #[inline(always)]
    pub fn green(&self) -> u8 {
        (self.0).1
    }

    #[inline(always)]
    pub fn blue(&self) -> u8 {
        (self.0).2
    }

    #[inline(always)]
    pub fn alpha(&self) -> u8 {
        (self.0).3
    }

    pub fn is_visible(&self) -> bool {
        *self != Self::TRANSPARENT
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::BLACK
    }
}

impl From<gdk::RGBA> for Color {
    fn from(val: gdk::RGBA) -> Self {
        Self((
            (val.red() * 255.0) as u8,
            (val.green() * 255.0) as u8,
            (val.blue() * 255.0) as u8,
            (val.alpha() * 255.0) as u8,
        ))
    }
}

impl From<Color> for gdk::RGBA {
    fn from(color: Color) -> Self {
        Self::new(
            f64::from((color.0).0) / 255.0,
            f64::from((color.0).1) / 255.0,
            f64::from((color.0).2) / 255.0,
            f64::from((color.0).3) / 255.0,
        )
    }
}

impl From<&Color> for gdk::RGBA {
    fn from(color: &Color) -> Self {
        Self::new(
            f64::from((color.0).0) / 255.0,
            f64::from((color.0).1) / 255.0,
            f64::from((color.0).2) / 255.0,
            f64::from((color.0).3) / 255.0,
        )
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (r, g, b, a) = self.0;
        let [r, g, b, _] = [u64::from(r), u64::from(g), u64::from(b), u64::from(a)];
        write!(fmt, "#{r:02X}{g:02X}{b:02X}")
    }
}

pub fn hex_color_to_rgb(s: &str) -> Option<(u8, u8, u8)> {
    if s.starts_with('#')
        && s.len() == 7
        && s[1..].as_bytes().iter().all(|&b| {
            b.is_ascii_digit() || (b'a'..=b'f').contains(&b) || (b'A'..=b'F').contains(&b)
        })
    {
        Some((
            u8::from_str_radix(&s[1..3], 16).ok()?,
            u8::from_str_radix(&s[3..5], 16).ok()?,
            u8::from_str_radix(&s[5..7], 16).ok()?,
        ))
    } else if s.starts_with('#')
        && s.len() == 4
        && s[1..].as_bytes().iter().all(|&b| {
            b.is_ascii_digit() || (b'a'..=b'f').contains(&b) || (b'A'..=b'F').contains(&b)
        })
    {
        Some((
            (17 * u8::from_str_radix(&s[1..2], 16).ok()?),
            (17 * u8::from_str_radix(&s[2..3], 16).ok()?),
            (17 * u8::from_str_radix(&s[3..4], 16).ok()?),
        ))
    } else {
        None
    }
}

pub(crate) const fn hex(s: &str) -> (u8, u8, u8, u8) {
    let s = s.as_bytes();

    if s.len() != 7
        || s[0] != b'#'
        || !s[1].is_ascii_hexdigit()
        || !s[2].is_ascii_hexdigit()
        || !s[3].is_ascii_hexdigit()
        || !s[4].is_ascii_hexdigit()
        || !s[5].is_ascii_hexdigit()
        || !s[6].is_ascii_hexdigit()
    {
        panic!("not a valid hex color value.");
    }

    let mut arr = [0, 0, 0];
    let mut i = 1;
    while i < 7 {
        let a = (i - 1) / 2;
        if s[i] >= b'A' && s[i] <= b'F' {
            arr[a] += s[i] - b'A' + 10;
        } else if s[i] >= b'a' && s[i] <= b'f' {
            arr[a] += s[i] - b'a' + 10;
        } else if s[i] >= b'0' && s[i] <= b'9' {
            arr[a] += s[i] - b'0';
        }
        if i % 2 == 1 && arr[a] != 0 {
            arr[a] = ((arr[a]) as u32 * 16) as u8;
        }
        i += 1;
    }
    (arr[0], arr[1], arr[2], 255)
}

#[test]
#[ignore]
fn test_const_color() {
    for a in ('0'..='9').chain('a'..='f') {
        for b in ('0'..='9').chain('a'..='f') {
            for c in ('0'..='9').chain('a'..='f') {
                for d in ('0'..='9').chain('a'..='f') {
                    for e in ('0'..='9').chain('a'..='f') {
                        for f in ('0'..='9').chain('a'..='f') {
                            let s = format!("#{a}{b}{c}{d}{e}{f}");
                            let (r, g, b) = hex_color_to_rgb(&s).unwrap();
                            assert_eq!(hex(&s), (r, g, b, 255), "{s}");
                        }
                    }
                }
            }
        }
    }
}

pub trait ColorExt {
    fn set_source_color(&self, color: Color);
    fn set_source_color_alpha(&self, color: Color);
    fn set_draw_opts(&self, opts: DrawOptions);
    fn show_text_with_bg(&self, text: &str, margin: f64, fg: Color, bg: Color);
}

impl ColorExt for gtk::cairo::Context {
    fn set_source_color(&self, color: Color) {
        self.set_source_rgb(
            f64::from((color.0).0) / 255.0,
            f64::from((color.0).1) / 255.0,
            f64::from((color.0).2) / 255.0,
        );
    }

    fn set_source_color_alpha(&self, color: Color) {
        self.set_source_rgba(
            f64::from((color.0).0) / 255.0,
            f64::from((color.0).1) / 255.0,
            f64::from((color.0).2) / 255.0,
            f64::from((color.0).3) / 255.0,
        );
    }

    fn set_draw_opts(&self, opts: DrawOptions) {
        self.set_source_color_alpha(opts.color);
        self.set_line_width(opts.size);
    }

    fn show_text_with_bg(&self, text: &str, margin: f64, fg: Color, bg: Color) {
        let (x, y) = self.current_point().unwrap();
        let extents = self.text_extents(text).unwrap();
        self.save().unwrap();
        self.set_source_color(bg);
        self.rectangle(
            x - margin,
            y - extents.height - margin,
            2.0f64.mul_add(margin, extents.width),
            2.0f64.mul_add(margin, extents.height),
        );
        self.fill().unwrap();
        self.restore().unwrap();

        self.move_to(x, y);
        self.set_source_color(fg);
        self.show_text(text).unwrap();
    }
}

mod rgba_serde {
    use super::Color;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl Serialize for Color {
        fn serialize<S>(&self, se: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            se.serialize_str(&self.to_string())
        }
    }

    impl<'de> Deserialize<'de> for Color {
        fn deserialize<D>(de: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            use serde::de::Error;

            #[derive(Deserialize, Serialize)]
            #[serde(untagged)]
            enum Val {
                Raw((f64, f64, f64, f64)),
                Text(String),
            }
            let val = <Val>::deserialize(de)?;
            match val {
                Val::Raw((r, g, b, a)) => Ok(Self::new_alpha(
                    (r * 255.0) as u8,
                    (g * 255.0) as u8,
                    (b * 255.0) as u8,
                    (a * 255.0) as u8,
                )),
                Val::Text(ref s) => {
                    if s.contains(',') {
                        let mut acc = [0u8; 4];
                        let mut i = 0;
                        for c in s.split(',') {
                            if let Ok(n) = c.parse::<f64>() {
                                acc[i] = (n * 255.0) as u8;
                            } else {
                                return Err(D::Error::custom(format!(
                                    "{:?} is not a valid RGB color value (i.e. `(0, 0, 1, .5)`).",
                                    s
                                )));
                            }
                            i += 1;
                        }
                        if i < acc.len() {
                            return Err(D::Error::custom(format!(
                                "{:?} is not a valid RGB color value (i.e. `(0, 0, 1, .5)`).",
                                s
                            )));
                        }
                        Ok(Self::new_alpha(acc[0], acc[1], acc[2], acc[3]))
                    } else {
                        Ok(Self::try_from_hex(s).ok_or_else(|| {
                            D::Error::custom(format!("{:?} is not a valid hex color value.", s))
                        })?)
                    }
                }
            }
        }
    }
}

// [ref:needs_user_doc]
// [ref:needs_dev_doc]
#[derive(Clone, PartialEq, Default, Debug, Copy, glib::Boxed)]
#[boxed_type(name = "DrawOptions")]
pub struct DrawOptions {
    pub color: Color,
    pub bg: Option<Color>,
    pub size: f64,
    pub inherit_size: Option<(&'static str, bool)>,
}

impl DrawOptions {
    pub fn scale(mut self, f: f64) -> Self {
        self.size /= f;
        self
    }

    pub fn with_bg(mut self, bg: Color) -> Self {
        self.bg = Some(bg);
        self
    }
}

impl From<(Color, f64)> for DrawOptions {
    fn from((color, size): (Color, f64)) -> Self {
        Self {
            color,
            bg: None,
            size,
            inherit_size: None,
        }
    }
}

impl From<(Color, f64, &'static str)> for DrawOptions {
    fn from((color, size, inherit_size): (Color, f64, &'static str)) -> Self {
        Self {
            color,
            bg: None,
            size,
            inherit_size: Some((inherit_size, true)),
        }
    }
}
