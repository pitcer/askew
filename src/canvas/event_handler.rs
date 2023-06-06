use anyhow::Result;
use winit::dpi::PhysicalPosition;

use crate::canvas::curve::control_points::bezier::Bezier;
use crate::canvas::curve::control_points::interpolation::Interpolation;
use crate::canvas::curve::control_points::polyline::Polyline;
use crate::canvas::curve::control_points::rational_bezier::RationalBezier;
use crate::canvas::curve::control_points::{
    ControlPointsCurve, CurvePoint, GetControlPoints, WeightedPoint,
};
use crate::canvas::curve::formula::FormulaCurve;
use crate::canvas::curve::Curve;
use crate::canvas::math::point::Point;
use crate::canvas::math::vector::Vector;
use crate::canvas::properties::CanvasProperties;
use crate::enum_apply;
use crate::event::handler::{
    AddPointHandler, ChangePointWeightHandler, DeletePointHandler, MovePointHandler,
};
use crate::event::CurveEvent;

pub struct EventHandler {}

impl EventHandler {
    pub fn handle_event(
        &self,
        curve: &mut Curve,
        properties: &mut CanvasProperties,
        event: CurveEvent,
    ) -> Result<()> {
        enum_apply!(curve, Curve::ControlPoints | Curve::Formula => |curve| {
            let mut handler = CurveEventHandler::new(curve, properties);
            handler.handle_event(event)
        })
    }
}

struct CurveEventHandler<'a, C> {
    curve: &'a mut C,
    properties: &'a mut CanvasProperties,
}

impl<'a, C> CurveEventHandler<'a, C> {
    pub fn new(curve: &'a mut C, properties: &'a mut CanvasProperties) -> Self {
        Self { curve, properties }
    }
}

impl<'a> CurveEventHandler<'a, FormulaCurve> {
    pub fn handle_event(&mut self, _event: CurveEvent) -> Result<()> {
        Ok(())
    }
}

impl<'a> CurveEventHandler<'a, ControlPointsCurve> {
    pub fn handle_event(&mut self, event: CurveEvent) -> Result<()> {
        enum_apply!(self.curve, ControlPointsCurve::Polyline | ControlPointsCurve::Bezier | ControlPointsCurve::Interpolation | ControlPointsCurve::RationalBezier=> |curve| {
            let mut handler = CurveEventHandler::new(curve, self.properties);
            match event {
                CurveEvent::ChangeCurrentIndex(change) => handler.change_current_index(change as isize),
                CurveEvent::ChangeWeight(weight) => handler.change_weight(weight),
                CurveEvent::ToggleConvexHull => {
                    handler.properties.show_convex_hull = !handler.properties.show_convex_hull;
                    Ok(())
                }
                CurveEvent::DeleteCurrentPoint => handler.delete_point(),
                CurveEvent::MoveCurrentPoint(vector) => handler.move_point(vector),
                CurveEvent::AddPoint(point) => {
                    let point = Self::scale_position(point);

                    handler.add_point(point)
                }
                CurveEvent::Resize { area } => {self.properties.area = area; Ok(())},

            }
        })
    }

    fn scale_position(position: PhysicalPosition<f64>) -> Point<f32> {
        Point::new(position.x as f32, position.y as f32)
    }
}

impl<'a, C> CurveEventHandler<'a, C>
where
    C: GetControlPoints,
{
    pub fn change_current_index(&mut self, change: isize) -> Result<()> {
        let i = self.properties.current_point_index as isize + change;
        let v = self.curve.control_points().length() as isize;
        let r = i % v;
        self.properties.current_point_index = if r < 0 {
            if v < 0 {
                r - v
            } else {
                r + v
            }
        } else {
            r
        } as usize;
        Ok(())
    }
}

impl<'a, C> CurveEventHandler<'a, C>
where
    C: MovePointHandler + DeletePointHandler,
{
    pub fn delete_point(&mut self) -> Result<()> {
        self.curve
            .handle_delete_point(self.properties.current_point_index)
    }

    pub fn move_point(&mut self, change: Vector<f32>) -> Result<()> {
        self.curve
            .handle_move_point(self.properties.current_point_index, change)
    }
}

impl<'a, C> CurveEventHandler<'a, C>
where
    C: AddPointHandler<Point = CurvePoint>,
{
    pub fn add_point(&mut self, pt: Point<f32>) -> Result<()> {
        self.curve.handle_add_point(pt)
    }
}

impl<'a> CurveEventHandler<'a, RationalBezier> {
    pub fn add_point(&mut self, pt: Point<f32>) -> Result<()> {
        self.add_point_weighted(pt)
    }
}

impl<'a, C> CurveEventHandler<'a, C>
where
    C: AddPointHandler<Point = WeightedPoint<f32, f32>>,
{
    pub fn add_point_weighted(&mut self, point: Point<f32>) -> Result<()> {
        let point = WeightedPoint::new(point, self.properties.default_weight);
        self.curve.handle_add_point(point)
    }
}

impl<'a> CurveEventHandler<'a, Bezier> {
    pub fn change_weight(&mut self, _change: f32) -> Result<()> {
        Ok(())
    }
}

impl<'a> CurveEventHandler<'a, Interpolation> {
    pub fn change_weight(&mut self, _change: f32) -> Result<()> {
        Ok(())
    }
}

impl<'a> CurveEventHandler<'a, Polyline> {
    pub fn change_weight(&mut self, _change: f32) -> Result<()> {
        Ok(())
    }
}

impl<'a, C> CurveEventHandler<'a, C>
where
    C: ChangePointWeightHandler,
{
    pub fn change_weight(&mut self, change: f32) -> Result<()> {
        self.curve
            .handle_change_point_weight(self.properties.current_point_index, |current| {
                if change < 0.0 {
                    current / -change
                } else {
                    current * change
                }
            })
    }
}
