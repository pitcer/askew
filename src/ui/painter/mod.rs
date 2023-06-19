use anyhow::Result;

use view::WindowView;

use crate::config::rgb::{Alpha, Rgb};
use crate::config::Config;
use crate::ui::color_scheme::ColorScheme;
use crate::ui::command::message::MessageType;
use crate::ui::command::CommandState;
use crate::ui::frame::panel::bar::TextPanel;
use crate::ui::frame::panel::pixel::Pixel;
use crate::ui::frame::panel::Panel;
use crate::ui::painter::font::{FontLayout, FontLoader, GlyphRasterizer};

pub mod font;
pub mod view;

pub struct Painter {
    font_loader: FontLoader,
    glyph_rasterizer: GlyphRasterizer,
    status_layout: FontLayout,
    command_layout: FontLayout,
    color_scheme: ColorScheme,
}

impl Painter {
    pub fn new(config: &Config) -> Result<Self> {
        let font_loader = FontLoader::new(&config.font_path)?;
        let glyph_rasterizer = GlyphRasterizer::new();
        let status_layout = FontLayout::new(config.font_size);
        let command_layout = FontLayout::new(config.font_size);
        let color_scheme = ColorScheme::from_config(config);
        Ok(Self {
            font_loader,
            glyph_rasterizer,
            status_layout,
            command_layout,
            color_scheme,
        })
    }

    pub fn paint(&mut self, view: WindowView<'_>, mut panel: Panel<'_>) -> Result<()> {
        panel.fill(Pixel::from_rgba(
            self.color_scheme.background_color,
            Alpha::max(),
        ));
        if let Some(background) = &view.frame.background() {
            panel.draw_pixmap(0, 0, background.as_ref());
        }
        let area = panel.area();
        let size = area.size();
        let split_layout = [size.height() as usize - 44, 22, 22];
        let [panel, status, command] = panel.split_vertical(split_layout);

        let canvas = &view.frame.canvas();
        let mut name = canvas.curve_type().to_string();
        name.truncate(6);
        self.status_layout
            .setup(&self.font_loader)
            .append_text(&format!(
                "{} {} {}/{} {}",
                view.mode,
                name,
                canvas.properties().current_curve + 1,
                canvas.curves().len(),
                canvas.properties().current_point_index
            ));
        let mut status_bar = TextPanel::new(
            status,
            self.color_scheme.text_color,
            self.color_scheme.status_bar_color,
        );
        status_bar.fill();
        status_bar.draw_layout(
            &self.font_loader,
            &self.status_layout,
            &mut self.glyph_rasterizer,
        );

        let mut setup = self.command_layout.setup(&self.font_loader);
        match &view.command {
            CommandState::Closed(command) => {
                if let Some(message) = command.message() {
                    let color = Self::message_color(message.message_type(), &self.color_scheme);
                    setup.append_color_text(message.text(), color);
                } else {
                    setup.append_text(" ");
                }
            }
            CommandState::Open(command) => {
                let buffer = command.input();
                setup.append_color_text(buffer, self.color_scheme.text_color);
                setup.append_color_text("█", self.color_scheme.text_color);
            }
        }
        let mut command_bar = TextPanel::new(
            command,
            self.color_scheme.text_color,
            self.color_scheme.command_bar_color,
        );
        command_bar.fill();
        command_bar.draw_layout(
            &self.font_loader,
            &self.command_layout,
            &mut self.glyph_rasterizer,
        );

        canvas.rasterize(panel)?;

        Ok(())
    }

    fn message_color(message_type: &MessageType, color_scheme: &ColorScheme) -> Rgb {
        match message_type {
            MessageType::Info => color_scheme.text_color,
            MessageType::Error => color_scheme.text_error_color,
        }
    }
}
