use anyhow::{anyhow, Result};
use tiny_skia::{Path, PathBuilder, PixmapMut};

use crate::canvas::curve::control_points::CurvePoint;
use crate::canvas::v2::visual_path::private::VisualPathProperties;
use crate::canvas::v2::DrawOn;

pub mod line;
pub mod point;

impl<T> DrawOn for T
where
    T: VisualPath,
{
    fn draw_on(&self, pixmap: PixmapMut<'_>) -> Result<()> {
        self.draw_on(pixmap)
    }
}

pub trait VisualPath: private::VisualPath {
    fn new(properties: Self::Properties) -> Self
    where
        Self: Sized,
    {
        let path = None;
        Self::new_internal(path, properties)
    }

    fn from_points<P>(
        points: impl ExactSizeIterator<Item = P>,
        properties: Self::Properties,
    ) -> Result<Self>
    where
        P: Into<CurvePoint>,
        Self: Sized,
    {
        let path = Self::build_path(points, &properties)?;
        let path = Some(path);
        Ok(Self::new_internal(path, properties))
    }

    fn draw_on(&self, pixmap: PixmapMut<'_>) -> Result<()> {
        if self.properties().visible() {
            let path = self
                .path()
                .as_ref()
                .ok_or_else(|| anyhow!("path should be built before drawing"))?;
            self.draw_on_internal(pixmap, path)?;
        }
        Ok(())
    }

    fn rebuild_path<P>(&mut self, points: impl ExactSizeIterator<Item = P>) -> Result<()>
    where
        P: Into<CurvePoint>,
    {
        let path = match self.path_mut().take() {
            None => Self::build_path(points, self.properties())?,
            Some(path) => {
                let builder = path.clear();
                Self::build_path_from_builder(builder, points, self.properties())?
            }
        };
        *self.path_mut() = Some(path);

        Ok(())
    }
}

impl<T> VisualPath for T where T: private::VisualPath {}

mod private {
    use super::{CurvePoint, Path, PathBuilder, PixmapMut, Result};

    pub trait VisualPath {
        type Properties: VisualPathProperties;

        fn new_internal(path: Option<Path>, properties: Self::Properties) -> Self;

        fn draw_on_internal(&self, pixmap: PixmapMut<'_>, path: &Path) -> Result<()>;

        fn build_path_from_builder<P>(
            builder: PathBuilder,
            points: impl Iterator<Item = P>,
            properties: &Self::Properties,
        ) -> Result<Path>
        where
            P: Into<CurvePoint>;

        fn build_path<P>(
            points: impl ExactSizeIterator<Item = P>,
            properties: &Self::Properties,
        ) -> Result<Path>
        where
            P: Into<CurvePoint>,
        {
            let length = points.len();
            let path = PathBuilder::with_capacity(length, length);
            let path = Self::build_path_from_builder(path, points, properties)?;
            Ok(path)
        }

        fn properties(&self) -> &Self::Properties;

        fn path(&self) -> &Option<Path>;

        fn path_mut(&mut self) -> &mut Option<Path>;
    }

    pub trait VisualPathProperties {
        fn visible(&self) -> bool;
    }
}
