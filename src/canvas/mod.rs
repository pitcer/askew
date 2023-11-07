use std::path::Path;

use anyhow::Result;
use rand::Rng;
use tiny_skia::PixmapMut;

use shape::Shape;

use crate::canvas::math::point::Point;
use crate::canvas::math::rectangle::Rectangle;
use crate::canvas::objects::Objects;
use crate::canvas::properties::CanvasState;
use crate::canvas::request::declare::AddPoint;
use crate::config::{CanvasConfig, ShapeType};
use crate::request::RequestHandlerMut;

pub mod base_line;
pub mod control_points;
pub mod control_points_curve;
pub mod math;
pub mod objects;
pub mod paint;
pub mod polygon;
pub mod properties;
pub mod request;
pub mod samples;
pub mod shape;
pub mod transition;
pub mod visual_path;

#[derive(Debug)]
pub struct Canvas {
    objects: Objects,
    size: Rectangle<f32>,
    state: CanvasState,
    config: CanvasConfig,
}

impl Canvas {
    #[must_use]
    pub fn new(objects: Objects, size: Rectangle<f32>, config: CanvasConfig) -> Self {
        let state = CanvasState::default();
        Self { objects, size, state, config }
    }

    pub fn new_empty(size: Rectangle<f32>, config: CanvasConfig) -> Self {
        let objects = Objects::new(&config);
        Self::new(objects, size, config)
    }

    pub fn from_file(
        path: impl AsRef<Path>,
        size: Rectangle<f32>,
        config: CanvasConfig,
    ) -> Result<Canvas> {
        let objects = Objects::from_file(path)?;
        let mut canvas = Self::new(objects, size, config);
        canvas.update_all();
        Ok(canvas)
    }

    pub fn replace_objects_from_file(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let mut objects = Objects::from_file(path)?;
        objects.update_all();
        self.objects = objects;
        Ok(())
    }

    fn update_all(&mut self) {
        self.objects.update_all();
    }

    pub fn save_to_file(&self, path: impl AsRef<Path>) -> Result<()> {
        self.objects.save_to_file(path)?;
        Ok(())
    }

    pub fn resize(&mut self, size: Rectangle<f32>) {
        self.size = size;
    }

    pub fn draw_on_all(&self, pixmap: &mut PixmapMut<'_>) {
        self.objects.draw_on_all(pixmap);
    }

    pub fn generate_random_points(&mut self, number_of_points: u32) -> Result<()> {
        let mut random = rand::thread_rng();
        let origin = self.size.origin();
        let size = self.size.size();

        for _ in 0..number_of_points {
            let horizontal = random.gen_range(origin.horizontal()..=size.width());
            let vertical = random.gen_range(origin.vertical()..=size.height());
            let point = Point::new(horizontal, vertical);
            self.handle_mut(AddPoint::new(point))?;
        }
        Ok(())
    }

    #[must_use]
    pub fn curve_type(&self) -> ShapeType {
        self.current_curve().curve_type()
    }

    #[must_use]
    pub fn current_curve(&self) -> &Shape {
        self.objects.get(self.state.current_curve).expect("current object id should be valid")
    }

    pub fn current_curve_mut(&mut self) -> &mut Shape {
        self.objects.get_mut(self.state.current_curve).expect("current object id should be valid")
    }

    #[must_use]
    pub fn objects_length(&self) -> usize {
        self.objects.length()
    }

    pub fn into_config(self) -> CanvasConfig {
        self.config
    }

    #[must_use]
    pub fn state(&self) -> &CanvasState {
        &self.state
    }

    #[must_use]
    pub fn state_mut(&mut self) -> &mut CanvasState {
        &mut self.state
    }

    #[must_use]
    pub fn config(&self) -> &CanvasConfig {
        &self.config
    }

    #[must_use]
    pub fn size(&self) -> Rectangle<f32> {
        self.size
    }
}
