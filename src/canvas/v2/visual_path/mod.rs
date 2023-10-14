use anyhow::Result;
use tiny_skia::{Path, PathBuilder, PixmapMut, Point};

use crate::canvas::v2::visual_path::private::{VisualPathDetails, VisualPathProperties};
use crate::canvas::v2::DrawOn;

pub mod line;
pub mod point;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VisualPath<T>
where
    T: VisualPathDetails,
{
    #[serde(skip)]
    path: Option<Path>,
    pub properties: T::Properties,
}

impl<T> VisualPath<T>
where
    T: VisualPathDetails,
{
    pub fn new(properties: T::Properties) -> Self {
        let path = None;
        Self { path, properties }
    }

    pub fn from_points<P>(
        points: impl ExactSizeIterator<Item = P>,
        properties: T::Properties,
    ) -> Result<Self>
    where
        P: Into<Point>,
    {
        let path =
            properties.visible().then(|| Self::build_new_path(points, &properties)).flatten();
        Ok(Self { path, properties })
    }

    pub fn rebuild_path<P>(&mut self, points: impl ExactSizeIterator<Item = P>)
    where
        P: Into<Point>,
    {
        if self.properties.visible() {
            let path = match self.path.take() {
                None => Self::build_new_path(points, &self.properties),
                Some(path) => {
                    let builder = path.clear();
                    Self::build_path_from_builder(builder, points, &self.properties)
                }
            };
            self.path = path;
        }
    }

    fn build_new_path<P>(
        points: impl ExactSizeIterator<Item = P>,
        properties: &T::Properties,
    ) -> Option<Path>
    where
        P: Into<Point>,
    {
        let length = points.len();
        let builder = PathBuilder::with_capacity(length, length);
        Self::build_path_from_builder(builder, points, properties)
    }

    fn build_path_from_builder<P>(
        builder: PathBuilder,
        points: impl ExactSizeIterator<Item = P>,
        properties: &T::Properties,
    ) -> Option<Path>
    where
        P: Into<Point>,
    {
        let points = points.map(P::into);
        T::build_path(builder, points, properties)
    }
}

impl<T> DrawOn for VisualPath<T>
where
    T: VisualPathDetails,
{
    fn draw_on(&self, pixmap: &mut PixmapMut<'_>) {
        if self.properties.visible() {
            if let Some(path) = &self.path {
                T::draw_on(pixmap, path, &self.properties);
            }
        }
    }
}

mod private {
    use super::{Path, PathBuilder, PixmapMut, Point};

    pub trait VisualPathDetails {
        type Properties: VisualPathProperties;

        fn draw_on(pixmap: &mut PixmapMut<'_>, path: &Path, properties: &Self::Properties);

        /// Returns None if path cannot be built from the given points, e.g. there are too little
        /// of them or their coordinates are out of bounds (infinite).
        fn build_path(
            builder: PathBuilder,
            points: impl ExactSizeIterator<Item = Point>,
            properties: &Self::Properties,
        ) -> Option<Path>;
    }

    pub trait VisualPathProperties {
        fn visible(&self) -> bool;
    }
}
