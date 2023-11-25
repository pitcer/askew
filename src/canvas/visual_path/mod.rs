use tiny_skia::{Path, PathBuilder, PixmapMut, Point};

use crate::canvas::shape::DrawOn;
use crate::canvas::visual_path::private::VisualPathDetails;

pub mod line;
pub mod point;
pub mod property;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VisualPath<T>
where
    T: VisualPathDetails,
{
    visible: bool,
    #[serde(skip)]
    path: Option<Path>,
    properties: T::Properties,
}

impl<T> VisualPath<T>
where
    T: VisualPathDetails,
{
    pub fn new(visible: bool, properties: T::Properties) -> Self {
        let path = None;
        Self { visible, path, properties }
    }

    pub fn rebuild_path<P>(&mut self, points: impl ExactSizeIterator<Item = P>)
    where
        P: Into<Point>,
    {
        let true = self.visible else { return };
        let path = if let Some(path) = self.path.take() {
            let builder = path.clear();
            Self::build_path_from_builder(builder, points, &self.properties)
        } else {
            Self::build_new_path(points, &self.properties)
        };
        self.path = path;
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
        let true = self.visible else { return };
        if let Some(path) = &self.path {
            T::draw_on(pixmap, path, &self.properties);
        }
    }
}

mod private {
    use super::{Path, PathBuilder, PixmapMut, Point};

    pub trait VisualPathDetails {
        type Properties;

        fn draw_on(pixmap: &mut PixmapMut<'_>, path: &Path, properties: &Self::Properties);

        /// Returns None if path cannot be built from the given points, e.g. there are too little
        /// of them or their coordinates are out of bounds (infinite).
        fn build_path(
            builder: PathBuilder,
            points: impl ExactSizeIterator<Item = Point>,
            properties: &Self::Properties,
        ) -> Option<Path>;
    }
}
