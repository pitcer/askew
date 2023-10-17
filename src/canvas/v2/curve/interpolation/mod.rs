use tiny_skia::PixmapMut;

use crate::canvas::v2::base_polyline::BasePolyline;
use crate::canvas::v2::control_points_curve::ControlPointsCurve;
use crate::canvas::v2::curve::interpolation::event_handler::InterpolationCurveEventHandler;
use crate::canvas::v2::{DrawOn, Update};
use crate::{
    canvas::curve::control_points::{CurvePoint, GetControlPoints},
    canvas::curve::converter::{PathConverter, ToPath},
    canvas::curve::samples::Samples,
    canvas::math,
    canvas::math::point::Point,
};

pub mod event_handler;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InterpolationCurve {
    pub control_points: ControlPointsCurve<CurvePoint>,
    pub polyline: BasePolyline<false>,
    pub properties: InterpolationCurveProperties,
    pub samples: Samples,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InterpolationCurveProperties {
    pub nodes: InterpolationNodes,
}

#[derive(Debug, Copy, Clone, Default, serde::Serialize, serde::Deserialize, clap::ValueEnum)]
pub enum InterpolationNodes {
    EquallySpaced,
    #[default]
    Chebyshev,
}

impl InterpolationCurve {
    pub fn new(
        control_points: ControlPointsCurve<CurvePoint>,
        polyline: BasePolyline<false>,
        properties: InterpolationCurveProperties,
        samples: Samples,
    ) -> Self {
        Self { control_points, polyline, properties, samples }
    }

    pub fn event_handler(&mut self) -> InterpolationCurveEventHandler<'_> {
        InterpolationCurveEventHandler::new(self)
    }
}

impl Update for InterpolationCurve {
    fn update(&mut self) {
        if self.control_points.points.length() > 1 {
            let length = self.control_points.points.length();
            let (ts, first, last) = match self.properties.nodes {
                InterpolationNodes::Chebyshev => {
                    let ts = (1..=length)
                        .map(|index| math::chebyshev(length, index))
                        .collect::<Vec<_>>();
                    let first = ts[0];
                    let last = ts[length - 1];
                    (ts, first, last)
                }
                InterpolationNodes::EquallySpaced => {
                    let ts = (0..length)
                        .map(|index| index as f32 / (length - 1) as f32)
                        .collect::<Vec<_>>();
                    (ts, 0.0, 1.0)
                }
            };

            let (xs, ys): (Vec<_>, Vec<_>) =
                self.control_points.points.iterator().map(|point| (*point).into()).unzip();
            let path = self
                .samples
                .equally_spaced(first..=last)
                .map(move |t| Point::new(math::lagrange(t, &ts, &xs), math::lagrange(t, &ts, &ys)));
            self.polyline.rebuild_paths(path);
        }

        self.control_points.rebuild_paths();
    }
}

impl DrawOn for InterpolationCurve {
    fn draw_on(&self, pixmap: &mut PixmapMut<'_>) {
        self.polyline.draw_on(pixmap);
        self.control_points.draw_on(pixmap);
    }
}

impl InterpolationCurveProperties {
    pub fn new(nodes: InterpolationNodes) -> Self {
        Self { nodes }
    }
}