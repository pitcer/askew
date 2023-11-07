use tiny_skia::{PixmapMut, Point};

use crate::canvas::shape::DrawOn;
use crate::canvas::visual_path::line::{VisualLine, VisualLineProperties};
use crate::canvas::visual_path::point::{VisualPoint, VisualPointProperties};
use crate::config::rgb::{Alpha, Rgb};
use crate::config::CanvasConfig;

pub type OpenBaseLine = VisualBaseLine<false>;
pub type ClosedBaseLine = VisualBaseLine<true>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VisualBaseLine<const CLOSED: bool> {
    line: VisualLine<CLOSED>,
    points: VisualPoint,
    #[serde(skip)]
    point_buffer: Vec<Point>,
}

impl<const CLOSED: bool> VisualBaseLine<CLOSED> {
    #[must_use]
    pub fn new(line: VisualLine<CLOSED>, points: VisualPoint) -> Self {
        let point_buffer = Vec::new();
        Self { line, points, point_buffer }
    }

    pub fn rebuild_paths<P>(&mut self, points: impl Iterator<Item = P>)
    where
        P: Into<Point>,
    {
        self.point_buffer.clear();
        let points = points.map(P::into);
        self.point_buffer.extend(points);
        self.line.rebuild_path(self.point_buffer.iter().copied());
        self.points.rebuild_path(self.point_buffer.iter().copied());
    }
}

impl<const CLOSED: bool> DrawOn for VisualBaseLine<CLOSED> {
    fn draw_on(&self, pixmap: &mut PixmapMut<'_>) {
        self.line.draw_on(pixmap);
        self.points.draw_on(pixmap);
    }
}

impl<const CLOSED: bool> Default for VisualBaseLine<CLOSED> {
    fn default() -> Self {
        Self {
            line: VisualLine::new(VisualLineProperties::new(true, 2.0, Rgb::WHITE, Alpha::OPAQUE)),
            points: VisualPoint::new(VisualPointProperties::new(
                false,
                4.0,
                Rgb::WHITE,
                Alpha::OPAQUE,
            )),
            point_buffer: Vec::new(),
        }
    }
}

impl<const CLOSED: bool> From<&CanvasConfig> for VisualBaseLine<CLOSED> {
    fn from(value: &CanvasConfig) -> Self {
        Self {
            line: VisualLine::new(VisualLineProperties::new(
                true,
                value.default_line_width,
                value.line_color,
                Alpha::OPAQUE,
            )),
            points: VisualPoint::new(VisualPointProperties::new(
                false,
                3.0,
                Rgb::WHITE,
                Alpha::OPAQUE,
            )),
            point_buffer: Vec::new(),
        }
    }
}
