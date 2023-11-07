use std::fs::File;
use std::path::Path;

use anyhow::Result;
use rand::Rng;
use tiny_skia::PixmapMut;

use shape::{DrawOn, Shape, Update};

use crate::canvas::math::point::Point;
use crate::canvas::math::rectangle::Rectangle;
use crate::canvas::objects::Objects;
use crate::canvas::properties::CanvasProperties;
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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Canvas {
    objects: Objects,
    size: Rectangle<f32>,
    #[serde(skip)]
    properties: CanvasProperties,
    pub config: CanvasConfig,
}

impl Canvas {
    #[must_use]
    pub fn new(size: Rectangle<f32>, config: CanvasConfig) -> Self {
        let properties = CanvasProperties::default();
        let objects = Objects::new(&config);
        Self { objects, size, properties, config }
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Canvas> {
        let file = File::open(path)?;
        let mut canvas = serde_json::from_reader::<_, Canvas>(file)?;
        canvas.update_all();
        Ok(canvas)
    }

    fn update_all(&mut self) {
        for object in self.objects.objects_mut() {
            object.update();
        }
    }

    pub fn save_to_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let file = File::create(path)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }

    pub fn resize(&mut self, size: Rectangle<f32>) {
        self.size = size;
    }

    pub fn draw_on_all(&self, pixmap: &mut PixmapMut<'_>) {
        for object in self.objects.objects() {
            object.draw_on(pixmap);
        }
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
        self.objects.get(self.properties.current_curve).expect("current object id should be valid")
    }

    pub fn current_curve_mut(&mut self) -> &mut Shape {
        self.objects
            .get_mut(self.properties.current_curve)
            .expect("current object id should be valid")
    }

    #[must_use]
    pub fn objects_length(&self) -> usize {
        self.objects.length()
    }

    #[must_use]
    pub fn properties(&self) -> &CanvasProperties {
        &self.properties
    }

    #[must_use]
    pub fn properties_mut(&mut self) -> &mut CanvasProperties {
        &mut self.properties
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
