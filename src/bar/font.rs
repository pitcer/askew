use std::fs;
use std::num::NonZeroUsize;

use anyhow::{anyhow, Result};
use fontdue::layout::{CoordinateSystem, GlyphPosition, GlyphRasterConfig, Layout, TextStyle};
use fontdue::{Font, FontSettings, Metrics};
use lru::LruCache;

#[derive(Debug)]
pub struct FontLoader {
    font: Font,
}

impl FontLoader {
    pub fn new(font_path: &str) -> Result<Self> {
        let font_settings = FontSettings::default();
        let font_data = fs::read(font_path)?;
        let font = Font::from_bytes(font_data, font_settings).map_err(|error| anyhow!(error))?;
        Ok(Self { font })
    }

    fn rasterize_config(&self, config: GlyphRasterConfig) -> (Metrics, Vec<u8>) {
        self.font.rasterize_config(config)
    }

    fn font(&self) -> &Font {
        &self.font
    }
}

// SAFETY: 256 is not equal to 0
const CACHE_CAPACITY: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(128) };

#[derive(Debug)]
pub struct GlyphRasterizer<'a> {
    font: &'a FontLoader,
    cache: LruCache<GlyphRasterConfig, (Metrics, Vec<u8>)>,
}

impl<'a> GlyphRasterizer<'a> {
    pub fn new(font: &'a FontLoader) -> Self {
        let cache = LruCache::new(CACHE_CAPACITY);
        Self { font, cache }
    }

    pub fn rasterize(&mut self, glyph: &GlyphPosition) -> &(Metrics, Vec<u8>) {
        let config = glyph.key;
        self.cache
            .get_or_insert(config, || self.font.rasterize_config(config))
    }
}

pub struct FontLayout<'a> {
    font: &'a FontLoader,
    layout: Layout,
    default_font_size: f32,
}

impl<'a> FontLayout<'a> {
    pub fn new(font: &'a FontLoader, default_font_size: u32) -> Self {
        let layout = Layout::new(CoordinateSystem::PositiveYDown);
        let default_font_size = default_font_size as f32;
        Self {
            font,
            layout,
            default_font_size,
        }
    }

    pub fn configure(&mut self) -> LayoutConfigurator<'_> {
        self.layout.clear();
        LayoutConfigurator::new(self.font, &mut self.layout, self.default_font_size)
    }

    pub fn height(&self) -> u32 {
        self.layout.height().ceil() as u32
    }

    pub fn glyphs(&self) -> &[GlyphPosition] {
        self.layout.glyphs()
    }
}

pub struct LayoutConfigurator<'a> {
    font: &'a FontLoader,
    layout: &'a mut Layout,
    default_font_size: f32,
}

impl<'a> LayoutConfigurator<'a> {
    fn new(font: &'a FontLoader, layout: &'a mut Layout, default_font_size: f32) -> Self {
        Self {
            font,
            layout,
            default_font_size,
        }
    }

    pub fn with_text(self, text: &str) -> Self {
        let default_font_size = self.default_font_size;
        self.with_style(text, default_font_size)
    }

    pub fn with_sized_text(self, text: &str, size: u32) -> Self {
        self.with_style(text, size as f32)
    }

    fn with_style(self, text: &str, size: f32) -> Self {
        let style = TextStyle::new(text, size, 0);
        self.layout.append(&[self.font.font()], &style);
        self
    }
}
