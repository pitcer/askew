use anyhow::{anyhow, Result};
use tiny_skia::{Path, PathBuilder, PixmapMut};

use crate::canvas::curve::control_points::CurvePoint;
use crate::canvas::v2::visual_path::private::{VisualPathDetails, VisualPathProperties};

pub mod line;
pub mod point;

#[derive(Debug, Clone)]
pub struct VisualPath<T>
where
    T: VisualPathDetails,
{
    path: Option<Path>,
    properties: T::Properties,
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
        P: Into<CurvePoint>,
    {
        let path = Self::build_path(points, &properties)?;
        let path = Some(path);
        Ok(Self { path, properties })
    }

    pub fn draw_on(&self, pixmap: PixmapMut<'_>) -> Result<()> {
        if self.properties.visible() {
            let path =
                self.path.as_ref().ok_or_else(|| anyhow!("path should be built before drawing"))?;
            T::draw_on(pixmap, path, &self.properties)?;
        }
        Ok(())
    }

    pub fn rebuild_path<P>(&mut self, points: impl ExactSizeIterator<Item = P>) -> Result<()>
    where
        P: Into<CurvePoint>,
    {
        let path = match self.path.take() {
            None => Self::build_path(points, &self.properties)?,
            Some(path) => {
                let builder = path.clear();
                T::build_path_from_builder(builder, points, &self.properties)?
            }
        };
        self.path = Some(path);

        Ok(())
    }

    fn build_path<P>(
        points: impl ExactSizeIterator<Item = P>,
        properties: &T::Properties,
    ) -> Result<Path>
    where
        P: Into<CurvePoint>,
    {
        let length = points.len();
        let path = PathBuilder::with_capacity(length, length);
        let path = T::build_path_from_builder(path, points, properties)?;
        Ok(path)
    }
}

mod private {
    use super::{CurvePoint, Path, PathBuilder, PixmapMut, Result};

    pub trait VisualPathDetails {
        type Properties: VisualPathProperties;

        fn draw_on(pixmap: PixmapMut<'_>, path: &Path, properties: &Self::Properties)
            -> Result<()>;

        fn build_path_from_builder<P>(
            builder: PathBuilder,
            points: impl Iterator<Item = P>,
            properties: &Self::Properties,
        ) -> Result<Path>
        where
            P: Into<CurvePoint>;
    }

    pub trait VisualPathProperties {
        fn visible(&self) -> bool;
    }
}
