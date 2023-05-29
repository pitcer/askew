use std::cmp::Ordering;
use std::fmt::Debug;

use num_traits::Num;

use crate::canvas::math::point::Point;

#[derive(Debug, Clone)]
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
        Self { points }
    }

    pub fn convex_hull(mut self) -> Vec<Point<T>> {
        if self.points.len() <= 1 {
            return self.points;
        }

        let (lowest_left_index, lowest_left) =
            self.find_lowest_left().expect("points should not be empty");
        self.points.swap(0, lowest_left_index);

        if self.points.len() == 2 {
            return self.points;
        }

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

        let (mut stack, mut points) = self.points.split_at_mut(3);
        let mut stack_size = stack.len();

        while let Some(point) = points.first() {
            while Self::orientation(stack[stack_size - 2], stack[stack_size - 1], *point)
                != Orientation::Counterclockwise
            {
                stack_size -= 1;
            }
            let stack_len = stack.len();
            (stack, points) = self.points.split_at_mut(stack_len + 1);
            stack.swap(stack_size, stack_len);
            stack_size += 1;
        }

        self.points.truncate(stack_size);
        self.points.shrink_to_fit();
        self.points
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_1() {
        let points = vec![
            Point::new(1, 2),
            Point::new(2, 0),
            Point::new(3, 1),
            Point::new(1, 1),
            Point::new(0, 1),
            Point::new(1, 0),
            Point::new(3, 0),
        ];
        let convex_hull = vec![
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(3, 0),
            Point::new(3, 1),
            Point::new(1, 2),
            Point::new(0, 1),
        ];
        let scan = GrahamScan::new(points);
        assert_eq!(convex_hull, scan.convex_hull())
    }
}
