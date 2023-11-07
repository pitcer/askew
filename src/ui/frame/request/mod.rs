use winit::dpi::PhysicalPosition;

use crate::canvas::math::point::Point;
use crate::canvas::math::vector::Vector;
use crate::canvas::request::declare::{
    AddCurve, AddPoint, ChangeCurrentCurveIndex, ChangeCurrentPointIndex, ChangeCurrentPointWeight,
    DeleteCurrentPoint, DeleteCurve, GetConvexHull, GetCurrentPoint, MoveCurrentPoint,
    SetConvexHull,
};
use crate::canvas::shape::request::declare::{GetCurveCenter, MoveCurve, RotateCurve, SelectPoint};
use crate::canvas::Canvas;
use crate::request::{
    Change, Direction, RequestHandler, RequestHandlerMut, RequestSubHandler, RequestSubHandlerMut,
    ResponseMut,
};
use crate::ui::frame::request::declare::{
    Add, ChangeIndex, ChangeWeight, Delete, MouseClick, MousePress, MovePoint, ToggleConvexHull,
};
use crate::ui::frame::Frame;
use crate::ui::mode::Mode;

pub mod declare;

impl RequestSubHandler<Canvas> for Frame {
    fn sub_handler(&self) -> &Canvas {
        self.canvas()
    }
}

impl RequestSubHandlerMut<Canvas> for Frame {
    fn sub_handler_mut(&mut self) -> &mut Canvas {
        self.canvas_mut()
    }
}

impl RequestHandlerMut<ToggleConvexHull> for Frame {
    fn handle_mut(&mut self, _event: ToggleConvexHull) -> ResponseMut<ToggleConvexHull> {
        let convex_hull = self.sub_handle(GetConvexHull)?;
        self.sub_handle_mut(SetConvexHull(!convex_hull))?;
        Ok(())
    }
}

impl RequestHandlerMut<ChangeWeight> for Frame {
    fn handle_mut(&mut self, event: ChangeWeight) -> ResponseMut<ChangeWeight> {
        let factor = match event.0 {
            Change::Decrease => 1.5,
            Change::Increase => -1.5,
        };
        match self.current_mode() {
            Mode::Curve => {
                self.sub_handler_mut()
                    .handle_mut(RotateCurve::new(std::f32::consts::PI * factor * 4.0 / 180.0))?;
            }
            Mode::Point => {
                self.sub_handler_mut().handle_mut(ChangeCurrentPointWeight::new(factor))?;
            }
            Mode::PointSelect | Mode::PointAdd => {}
        }

        Ok(())
    }
}

impl RequestHandlerMut<MouseClick> for Frame {
    fn handle_mut(&mut self, event: MouseClick) -> ResponseMut<MouseClick> {
        let click_point = scale_position(event.0);
        match self.current_mode() {
            Mode::Curve => {
                let Some(center) = self.sub_handler().handle(GetCurveCenter)? else {
                    return Ok(());
                };
                let shift = click_point - center;
                self.sub_handler_mut().handle_mut(MoveCurve::new(shift))?;
            }
            Mode::Point => {
                let point = self.sub_handler().handle(GetCurrentPoint)?;
                let shift = click_point - point;
                self.sub_handler_mut().handle_mut(MoveCurrentPoint::new(shift))?;
            }
            Mode::PointAdd => {
                self.sub_handler_mut().handle_mut(AddPoint::new(click_point))?;
            }
            Mode::PointSelect => {
                let point = self.sub_handler().handle(SelectPoint::new(
                    click_point,
                    self.canvas.config().default_point_radius,
                ))?;
                if let Some(point) = point {
                    self.canvas.state_mut().current_point_index = point;
                }
            }
        }
        Ok(())
    }
}

impl RequestHandlerMut<MousePress> for Frame {
    fn handle_mut(&mut self, event: MousePress) -> ResponseMut<MousePress> {
        let click_point = scale_position(event.0);
        match self.current_mode() {
            Mode::Curve => {
                let Some(center) = self.sub_handler().handle(GetCurveCenter)? else {
                    return Ok(());
                };
                let shift = click_point - center;
                self.sub_handler_mut().handle_mut(MoveCurve::new(shift))?;
            }
            Mode::Point => {
                let point = self.sub_handler().handle(GetCurrentPoint)?;
                let shift = click_point - point;
                self.sub_handler_mut().handle_mut(MoveCurrentPoint::new(shift))?;
            }
            Mode::PointAdd | Mode::PointSelect => {}
        }
        Ok(())
    }
}

impl RequestHandlerMut<MovePoint> for Frame {
    fn handle_mut(&mut self, event: MovePoint) -> ResponseMut<MovePoint> {
        let direction = match event.0 {
            Direction::Up => Vector::new(0.0, -4.0),
            Direction::Down => Vector::new(0.0, 4.0),
            Direction::Left => Vector::new(-4.0, 0.0),
            Direction::Right => Vector::new(4.0, 0.0),
        };
        match self.current_mode() {
            Mode::Curve => self.sub_handler_mut().handle_mut(MoveCurve::new(direction))?,
            Mode::Point => self.sub_handler_mut().handle_mut(MoveCurrentPoint::new(direction))?,
            Mode::PointAdd | Mode::PointSelect => {}
        }
        Ok(())
    }
}

impl RequestHandlerMut<Delete> for Frame {
    fn handle_mut(&mut self, _event: Delete) -> ResponseMut<Delete> {
        match self.current_mode() {
            Mode::Curve => self.sub_handler_mut().handle_mut(DeleteCurve)?,
            Mode::Point => self.sub_handler_mut().handle_mut(DeleteCurrentPoint)?,
            Mode::PointAdd | Mode::PointSelect => {}
        }
        Ok(())
    }
}

impl RequestHandlerMut<ChangeIndex> for Frame {
    fn handle_mut(&mut self, event: ChangeIndex) -> ResponseMut<ChangeIndex> {
        let change = match event.0 {
            Change::Decrease => -1,
            Change::Increase => 1,
        };
        match self.current_mode() {
            Mode::Curve => {
                self.sub_handler_mut().handle_mut(ChangeCurrentCurveIndex::new(change))?;
            }
            Mode::Point => {
                self.sub_handler_mut().handle_mut(ChangeCurrentPointIndex::new(change))?;
            }
            Mode::PointAdd | Mode::PointSelect => {}
        }
        Ok(())
    }
}

impl RequestHandlerMut<Add> for Frame {
    fn handle_mut(&mut self, _event: Add) -> ResponseMut<Add> {
        match self.current_mode() {
            Mode::Curve => self.sub_handler_mut().handle_mut(AddCurve)?,
            Mode::Point => self.mode_mut().enter_add(),
            Mode::PointAdd | Mode::PointSelect => {}
        }
        Ok(())
    }
}

fn scale_position(position: PhysicalPosition<f64>) -> Point<f32> {
    Point::new(position.x as f32, position.y as f32)
}
