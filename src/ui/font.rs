use std::fs;
use std::num::NonZeroUsize;
use std::path::Path;

use anyhow::{anyhow, Result};
use fontdue::layout::{CoordinateSystem, GlyphRasterConfig, TextStyle};
use fontdue::{Font, FontSettings, Metrics};
use lru::LruCache;

use crate::canvas::math::point::Point;
use crate::ui::color::{Alpha, Rgb};

#[derive(Debug)]
pub struct FontLoader {
    font: Font,
}

impl FontLoader {
    pub fn new(font_path: impl AsRef<Path>) -> Result<Self> {
        let font_settings = FontSettings::default();
        let font_data = fs::read(font_path)?;
        let font = Font::from_bytes(font_data, font_settings).map_err(|error| anyhow!(error))?;
        Ok(Self { font })
    }

    fn font(&self) -> &Font {
        &self.font
    }
}

// SAFETY: 95 is not equal to 0
const CACHE_CAPACITY: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(95) };

pub type GlyphColor = Option<Rgb>;
type GlyphCache = LruCache<GlyphRasterConfig, (Metrics, Vec<u8>)>;
pub type GlyphPosition = fontdue::layout::GlyphPosition<GlyphColor>;
pub type Layout = fontdue::layout::Layout<GlyphColor>;

#[derive(Debug)]
pub struct GlyphRasterizer {
    cache: GlyphCache,
}

impl GlyphRasterizer {
    #[must_use]
    pub fn new() -> Self {
        let cache = LruCache::new(CACHE_CAPACITY);
        Self { cache }
    }

    pub fn rasterize<'a>(
        &'a mut self,
        font: &FontLoader,
        position: &'a GlyphPosition,
    ) -> GlyphRaster<'a> {
        let config = position.key;
        let (metrics, raster) = self
            .cache
            .get_or_insert(config, || font.font().rasterize_config(config));
        GlyphRaster::new(position, metrics, raster)
    }
}

impl Default for GlyphRasterizer {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FontLayout {
    layout: Layout,
    default_font_size: f32,
}

impl FontLayout {
    #[must_use]
    pub fn new(default_font_size: u32) -> Self {
        let layout = Layout::new(CoordinateSystem::PositiveYDown);
        let default_font_size = default_font_size as f32;
        Self {
            layout,
            default_font_size,
        }
    }

    pub fn setup<'a>(&'a mut self, font: &'a FontLoader) -> LayoutSetup<'a> {
        self.layout.clear();
        LayoutSetup::new(font.font(), &mut self.layout, self.default_font_size)
    }

    #[must_use]
    pub fn height(&self) -> u32 {
        self.layout.height().ceil() as u32
    }

    #[must_use]
    pub fn glyph_positions(&self) -> &[GlyphPosition] {
        self.layout.glyphs()
    }
}

pub struct LayoutSetup<'a> {
    layout: &'a mut Layout,
    default_font_size: f32,
    font: &'a Font,
}

impl<'a> LayoutSetup<'a> {
    fn new(font: &'a Font, layout: &'a mut Layout, default_font_size: f32) -> Self {
        Self {
            layout,
            default_font_size,
            font,
        }
    }

    pub fn append_color_text(&mut self, text: &str, color: Rgb) {
        let default_font_size = self.default_font_size;
        self.append_style(text, default_font_size, Some(color));
    }

    pub fn append_text(&mut self, text: &str) {
        let default_font_size = self.default_font_size;
        self.append_style(text, default_font_size, None);
    }

    pub fn append_sized_text(&mut self, text: &str, size: u32) {
        self.append_style(text, size as f32, None);
    }

    fn append_style(&mut self, text: &str, size: f32, color: Option<Rgb>) {
        let style = TextStyle::with_user_data(text, size, 0, color);
        self.layout.append(&[self.font], &style);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GlyphRaster<'a> {
    position: &'a GlyphPosition,
    metrics: &'a Metrics,
    raster: &'a [u8],
}

impl<'a> GlyphRaster<'a> {
    #[must_use]
    pub fn new(position: &'a GlyphPosition, metrics: &'a Metrics, raster: &'a [u8]) -> Self {
        Self {
            position,
            metrics,
            raster,
        }
    }

    pub fn iterator(&'a self) -> impl Iterator<Item = GlyphPixel> + 'a {
        (0..self.metrics.height)
            .flat_map(|vertical| {
                (0..self.metrics.width).map(move |horizontal| (horizontal, vertical))
            })
            .map(move |(horizontal, vertical)| {
                let position = Point::new(
                    self.position.x as usize + horizontal,
                    self.position.y as usize + vertical,
                );
                let alpha = self.raster[horizontal + vertical * self.metrics.width];
                let alpha = Alpha::new(alpha);
                GlyphPixel { position, alpha }
            })
    }
}

pub struct GlyphPixel {
    pub position: Point<usize>,
    pub alpha: Alpha,
}
