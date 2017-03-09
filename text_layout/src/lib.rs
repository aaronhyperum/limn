//! Text layout logic.

extern crate rusttype;

pub mod types;
pub mod cursor;
pub mod glyph;
pub mod line;

use std::f64;
use rusttype::Scale;
use self::line::{LineRects, LineInfo, LineInfos};

pub type FontSize = u32;
/// The RustType `PositionedGlyph` type used by conrod.
pub type PositionedGlyph = rusttype::PositionedGlyph<'static>;

pub type Font = rusttype::Font<'static>;

pub use types::{Align, Scalar, Rectangle, Dimensions};

/// The way in which text should wrap around the width.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Wrap {
    NoWrap,
    /// Wrap at the first character that exceeds the width.
    Character,
    /// Wrap at the first word that exceeds the width.
    Whitespace,
}

pub fn get_text_dimensions(text: &str,
                           font: &Font,
                           font_size: Scalar,
                           line_height: Scalar,
                           wrap: Wrap)
                           -> Dimensions {

    let line_infos = LineInfos::new(text, font, font_size, wrap, f64::MAX);
    let max_width = line_infos.fold(0.0, |max, line_info| f64::max(max, line_info.width));
    Dimensions {
        width: max_width,
        height: line_infos.count() as f64 * line_height,
    }
}pub fn get_text_height(text: &str,
                        font: &Font,
                        font_size: Scalar,
                        line_height: Scalar,
                        wrap: Wrap,
                        width: Scalar)
                        -> Scalar {
    let line_infos = LineInfos::new(text, font, font_size, wrap, width);
    line_infos.count() as f64 * line_height
}
pub fn get_line_rects(text: &str,
                      rect: Rectangle,
                      font: &Font,
                      font_size: Scalar,
                      line_height: Scalar,
                      line_wrap: Wrap,
                      x_align: Align,
                      y_align: Align)
                      -> Vec<Rectangle> {

    let line_infos: Vec<LineInfo> = LineInfos::new(text, font, font_size, line_wrap, rect.width)
        .collect();
    let line_infos = line_infos.iter().cloned();
    let line_rects = LineRects::new(line_infos, font_size, rect, x_align, y_align, line_height);
    line_rects.collect()
}

pub fn get_positioned_glyphs(text: &str,
                             rect: Rectangle,
                             font: &Font,
                             font_size: Scalar,
                             line_height: Scalar,
                             line_wrap: Wrap,
                             x_align: Align,
                             y_align: Align)
                             -> Vec<PositionedGlyph>
{
    let line_infos: Vec<LineInfo> = LineInfos::new(text, font, font_size, line_wrap, rect.width)
        .collect();
    let line_infos = line_infos.iter().cloned();
    let line_texts = line_infos.clone().map(|info| &text[info.byte_range()]);
    let line_rects = LineRects::new(line_infos, font_size, rect, x_align, y_align, line_height);

    let mut positioned_glyphs = Vec::new();
    for (line_text, line_rect) in line_texts.zip(line_rects) {

        let point = rusttype::Point {
            x: line_rect.left as f32,
            y: line_rect.top as f32 + font_size as f32,
        };
        positioned_glyphs.extend(font.layout(line_text, Scale::uniform(font_size as f32), point)
            .map(|g| g.standalone()));
    }
    positioned_glyphs
}

/// An iterator yielding each line within the given `text` as a new `&str`, where the start and end
/// indices into each line are provided by the given iterator.
#[derive(Clone)]
pub struct Lines<'a, I>
    where I: Iterator<Item = std::ops::Range<usize>>
{
    text: &'a str,
    ranges: I,
}

/// Produce an iterator yielding each line within the given `text` as a new `&str`, where the
/// start and end indices into each line are provided by the given iterator.
pub fn lines<I>(text: &str, ranges: I) -> Lines<I>
    where I: Iterator<Item = std::ops::Range<usize>>
{
    Lines {
        text: text,
        ranges: ranges,
    }
}

impl<'a, I> Iterator for Lines<'a, I>
    where I: Iterator<Item = std::ops::Range<usize>>
{
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let Lines { text, ref mut ranges } = *self;
        ranges.next().map(|range| &text[range])
    }
}


/// Converts the given font size in "points" to its font size in pixels.
pub fn pt_to_px(font_size_in_points: Scalar) -> f32 {
    font_size_in_points as f32
}

/// Converts the given font size in "points" to a uniform `rusttype::Scale`.
pub fn pt_to_scale(font_size_in_points: Scalar) -> Scale {
    Scale::uniform(pt_to_px(font_size_in_points))
}
