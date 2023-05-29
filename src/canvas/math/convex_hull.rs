use std::cmp::Ordering;
use std::fmt::Debug;

use num_traits::Num;

use crate::canvas::math::point::Point;

#[derive(Debug)]
pub struct GrahamScan<T> {
    points: Vec<Point<T>>,
}

#[derive(Debug, Eq, PartialEq)]
enum Orientation {
    Equal,
    Clockwise,
    Counterclockwise,
}

impl<T> GrahamScan<T>
where
    T: Copy + PartialOrd + Num,
{
    pub fn new(points: Vec<Point<T>>) -> Self {
        debug_assert!(points.len() >= 3);

        Self { points }
    }

    pub fn convex_hull(mut self) -> Vec<Point<T>> {
        let (lowest_left_index, lowest_left) =
            self.find_lowest_left().expect("points should not be empty");
        self.points.swap(0, lowest_left_index);

        self.points[1..].sort_unstable_by(|first, second| {
            let orientation = Self::orientation(lowest_left, *first, *second);
            match orientation {
                Orientation::Equal => {
                    let lowest_to_first_distance = lowest_left.distance_squared(*first);
                    let lowest_to_second_distance = lowest_left.distance_squared(*second);
                    lowest_to_first_distance
                        .partial_cmp(&lowest_to_second_distance)
                        .expect("Ordering should be defined")
                }
                Orientation::Clockwise => Ordering::Greater,
                Orientation::Counterclockwise => Ordering::Less,
            }
        });

        // TODO: try to do this in place (without that additional stack)
        let mut stack = Vec::with_capacity(self.points.len());
        stack.extend_from_slice(&self.points[0..3]);

        for point in &self.points[3..] {
            while Self::orientation(
                stack[stack.len() - 2],
                stack[stack.len() - 1],
                *point,
            ) != Orientation::Counterclockwise
            {
                stack.pop();
            }
            stack.push(*point)
        }

        stack.shrink_to_fit();
        stack
    }

    /// Returns:
    /// * `Clockwise` if `second` is on the right to `first` in respect to `base`
    /// * `Counterclockwise` if `second` is on the left to `first` in respect to `base`
    /// * `Equal` if `first` and `second` are collinear in respect to `base`
    fn orientation(base: Point<T>, first: Point<T>, second: Point<T>) -> Orientation {
        let lowest_to_first = first - base;
        let first_to_second = second - first;
        let cross_product_magnitude = lowest_to_first.cross_product_magnitude(first_to_second);
        if cross_product_magnitude == T::zero() {
            Orientation::Equal
        } else if cross_product_magnitude > T::zero() {
            Orientation::Counterclockwise
        } else {
            Orientation::Clockwise
        }
    }

    fn find_lowest_left(&self) -> Option<(usize, Point<T>)> {
        self.points.iter().copied().enumerate().reduce(
            |lowest @ (_, lowest_point), current @ (_, current_point)| {
                if current_point.vertical() < lowest_point.vertical()
                    || (current_point.vertical() == lowest_point.vertical()
                        && current_point.horizontal() < lowest_point.horizontal())
                {
                    current
                } else {
                    lowest
                }
            },
        )
    }
}
