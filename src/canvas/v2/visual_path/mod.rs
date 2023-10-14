use anyhow::{anyhow, Result};
use either::Either;
use tiny_skia::{Path, PathBuilder, PixmapMut, Point};

use crate::canvas::curve::control_points::CurvePoint;
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

    pub fn rebuild_path<'a, P>(
        &'a mut self,
        points: impl ExactSizeIterator<Item = P> + 'a,
    ) -> Result<
        Either<impl ExactSizeIterator<Item = Point> + 'a, impl ExactSizeIterator<Item = P> + 'a>,
    >
    where
        P: Into<CurvePoint> + Copy,
    {
        if self.properties.visible() {
            let path = match self.path.take() {
                None => Self::build_path(points, &self.properties)?,
                Some(path) => {
                    let builder = path.clear();
                    T::build_path_from_builder(builder, points, &self.properties)?
                }
            };
            let path = self.path.insert(path);
            Ok(Either::Left(path.points().iter().copied()))
        } else {
            Ok(Either::Right(points))
        }
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

    pub fn points(&self) -> Option<&[Point]> {
        self.path.as_ref().map(Path::points)
    }
}

impl<T> DrawOn for VisualPath<T>
where
    T: VisualPathDetails,
{
    fn draw_on(&self, pixmap: &mut PixmapMut<'_>) -> Result<()> {
        if self.properties.visible() {
            let path =
                self.path.as_ref().ok_or_else(|| anyhow!("path should be built before drawing"))?;
            T::draw_on(pixmap, path, &self.properties)?;
        }
        Ok(())
    }
}

macro_rules! rebuild_many {
    ([$first_path:expr, $($path:expr),* $(,)?], $points:expr) => {
        $first_path.rebuild_path($points)?;
        // Reusing points generated in the previous step assumes that path generation does not
        // change points and their order.
        let points = $first_path.points().expect("Path should be present");
        $({
            let points = points.iter().copied();
            $path.rebuild_path(points)?;
        })*
    };
}

pub(crate) use rebuild_many;

mod private {
    use super::{CurvePoint, Path, PathBuilder, PixmapMut, Result};

    pub trait VisualPathDetails {
        type Properties: VisualPathProperties;

        fn draw_on(
            pixmap: &mut PixmapMut<'_>,
            path: &Path,
            properties: &Self::Properties,
        ) -> Result<()>;

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
