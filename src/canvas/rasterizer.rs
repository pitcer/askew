use anyhow::Result;
use tiny_skia::Stroke;

use crate::canvas::curve::control_points::ControlPointsCurveKind;
use crate::canvas::curve::converter::{TinySkiaPathConverter, ToPath};
use crate::canvas::curve::CurveKind;
use crate::canvas::paint::PaintBuilder;
use crate::canvas::properties::CanvasProperties;
use crate::canvas::v2::DrawOn;
use crate::ui::frame::panel::Panel;

#[derive(Debug)]
pub struct Rasterizer;

impl Rasterizer {
    pub fn rasterize<'a>(
        &self,
        curve: &'a CurveKind,
        properties: &'a CanvasProperties,
        panel: &'a mut Panel<'_>,
    ) -> Result<()> {
        let mut pixmap = panel.as_pixmap_mut();
        match curve {
            CurveKind::ControlPoints(curve) => match curve {
                ControlPointsCurveKind::PolylineV2(curve) => curve.draw_on(&mut pixmap),
                ControlPointsCurveKind::Interpolation(curve) => curve.draw_on(&mut pixmap),
                ControlPointsCurveKind::BezierV2(curve) => curve.draw_on(&mut pixmap),
                ControlPointsCurveKind::RationalBezier(curve) => {
                    curve.draw_on(&mut pixmap);
                }
            },
            CurveKind::Formula(curve) => {
                let mut rasterizer = CurveRasterizer::new(curve, properties, panel);
                rasterizer.draw_curve();
            }
        }

        Ok(())
    }
}

struct CurveRasterizer<'a, 'b, T> {
    curve: &'a T,
    properties: &'a CanvasProperties,
    panel: &'a mut Panel<'b>,
}

impl<'a, 'b, T> CurveRasterizer<'a, 'b, T> {
    pub fn new(curve: &'a T, properties: &'a CanvasProperties, panel: &'a mut Panel<'b>) -> Self {
        Self { curve, properties, panel }
    }
}

impl<'a, 'b, T> CurveRasterizer<'a, 'b, T>
where
    T: ToPath,
{
    fn draw_curve(&mut self) {
        if let Some(path) = self.curve.to_path(TinySkiaPathConverter) {
            let paint = PaintBuilder::new().rgb_color(self.properties.line_color).build();
            let stroke = Stroke { width: self.properties.line_width, ..Stroke::default() };
            self.panel.draw_stroke_path(&path, &paint, &stroke);
        }
    }
}
