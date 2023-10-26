use tiny_skia::PixmapMut;

use crate::canvas::base_line::VisualBaseLine;
use crate::canvas::control_points::point::CurvePoint;
use crate::canvas::control_points::ControlPoints;
use crate::canvas::control_points_curve::VisualControlPoints;
use crate::canvas::shape::{DrawOn, Update};
use crate::{canvas::math, canvas::math::point::Point, canvas::samples::Samples};

pub mod request;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InterpolationCurve {
    pub points: ControlPoints<CurvePoint>,
    pub control_points: VisualControlPoints,
    pub polyline: VisualBaseLine<false>,
    pub properties: InterpolationCurveProperties,
    pub samples: Samples,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
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
    #[must_use]
    pub fn new(
        points: ControlPoints<CurvePoint>,
        control_points: VisualControlPoints,
        polyline: VisualBaseLine<false>,
        properties: InterpolationCurveProperties,
        samples: Samples,
    ) -> Self {
        Self { points, control_points, polyline, properties, samples }
    }
}

impl Update for InterpolationCurve {
    fn update(&mut self) {
        if self.points.length() > 1 {
            let length = self.points.length();
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
                self.points.iterator().map(|point| (*point).into()).unzip();
            let path = self
                .samples
                .equally_spaced(first..=last)
                .map(move |t| Point::new(math::lagrange(t, &ts, &xs), math::lagrange(t, &ts, &ys)));
            self.polyline.rebuild_paths(path);
        }

        self.control_points.rebuild_paths(&self.points);
    }
}

impl DrawOn for InterpolationCurve {
    fn draw_on(&self, pixmap: &mut PixmapMut<'_>) {
        self.polyline.draw_on(pixmap);
        self.control_points.draw_on(pixmap);
    }
}

impl InterpolationCurveProperties {
    #[must_use]
    pub fn new(nodes: InterpolationNodes) -> Self {
        Self { nodes }
    }
}
