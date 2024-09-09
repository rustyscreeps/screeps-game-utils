use std::iter::FusedIterator;

use screeps::{RoomCoordinate, RoomXY};

use crate::room_coordinate::{range_exclusive, range_inclusive};

// An iterator over ordered pairs of RoomCoordinates; first coordinate is the major axis.
#[derive(Debug, Clone)]
struct PairIter {
    // SAFETY INVARIANT: forward.1 and backward.1 are within b_min..=b_max when !done
    b_min: RoomCoordinate,
    b_max: RoomCoordinate,
    // SAFETY INVARIANT: forward <= backward, by lex order, when !done
    forward: (RoomCoordinate, RoomCoordinate),
    backward: (RoomCoordinate, RoomCoordinate),
    done: bool,
}

impl PairIter {
    fn new(min: (RoomCoordinate, RoomCoordinate), max: (RoomCoordinate, RoomCoordinate)) -> Self {
        Self {
            b_min: min.1,
            b_max: max.1,
            forward: min,
            backward: max,
            // SAFETY INVARIANT: if this is false, then the b_min/b_max criterion is true,
            // and max (backward) is lex-order >= min (forward).
            done: max.0 < min.0 || max.1 < min.1,
        }
    }
}

impl Iterator for PairIter {
    type Item = (RoomCoordinate, RoomCoordinate);

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let res = Some(self.forward);

        if self.forward == self.backward {
            self.done = true;
        } else if self.forward.1 == self.b_max {
            // SAFETY: self.backward.1 <= self.b_max, so self.forward.0 < self.backward.0, meaning we can increment by 1.
            self.forward = (
                unsafe { RoomCoordinate::unchecked_new(self.forward.0.u8() + 1) },
                self.b_min,
            );
        } else {
            // SAFETY: self.forward.1 < self.b_max, so we can step up by 1.
            self.forward.1 = unsafe { RoomCoordinate::unchecked_new(self.forward.1.u8() + 1) };
        }

        res
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        if self.done {
            return init;
        }

        #[cold]
        fn cold_call<B>(
            this: PairIter,
            init: B,
            mut f: impl FnMut(B, <PairIter as Iterator>::Item) -> B,
        ) -> B {
            if this.forward.0 == this.backward.0 {
                return range_inclusive(this.forward.1, this.backward.1)
                    .map(|b| (this.forward.0, b))
                    .fold(init, f);
            }

            let forward_partial_acc = range_inclusive(this.forward.1, this.b_max)
                .map(|b| (this.forward.0, b))
                .fold(init, &mut f);

            let middle_partials_acc = range_exclusive(this.forward.0, this.backward.0).fold(
                forward_partial_acc,
                |inner_acc, a| {
                    range_inclusive(this.b_min, this.b_max)
                        .map(|b| (a, b))
                        .fold(inner_acc, &mut f)
                },
            );

            range_inclusive(this.b_min, this.backward.1)
                .map(|b| (this.backward.0, b))
                .fold(middle_partials_acc, f)
        }

        if self.forward.1 == self.b_min && self.backward.1 == self.b_max {
            range_inclusive(self.forward.0, self.backward.0).fold(init, |acc, a| {
                range_inclusive(self.b_min, self.b_max)
                    .map(|b| (a, b))
                    .fold(acc, &mut f)
            })
        } else {
            cold_call(self, init, f)
        }
    }
}

impl FusedIterator for PairIter {}

impl ExactSizeIterator for PairIter {
    fn len(&self) -> usize {
        if self.done {
            return 0;
        }

        let forward = (self.forward.0.u8(), self.forward.1.u8());
        let backward = (self.backward.0.u8(), self.backward.1.u8());

        if forward.0 == backward.0 {
            return (backward.1 - forward.1 + 1) as usize;
        }

        let full_ranges = (backward.0 - forward.0 - 1) as usize;
        full_ranges * (self.b_max.u8() - self.b_min.u8() + 1) as usize
            + (self.b_max.u8() - forward.1 + 1) as usize
            + (backward.1 - self.b_min.u8() + 1) as usize
    }
}

impl DoubleEndedIterator for PairIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let res = Some(self.backward);

        if self.backward == self.forward {
            self.done = true;
        } else if self.backward.1 == self.b_min {
            // SAFETY: self.forward.1 >= self.b_min, so self.forward.0 < self.backward.0, meaning we can decrement by 1.
            self.backward = (
                unsafe { RoomCoordinate::unchecked_new(self.backward.0.u8() - 1) },
                self.b_max,
            );
        } else {
            // SAFETY: self.backward.1 > self.b_min, so we can step down by 1.
            self.backward.1 = unsafe { RoomCoordinate::unchecked_new(self.backward.1.u8() - 1) };
        }

        res
    }

    fn rfold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        if self.done {
            return init;
        }

        #[cold]
        fn cold_call<B>(
            this: PairIter,
            init: B,
            mut f: impl FnMut(B, <PairIter as Iterator>::Item) -> B,
        ) -> B {
            if this.forward.0 == this.backward.0 {
                return range_inclusive(this.forward.1, this.backward.1)
                    .map(|b| (this.forward.0, b))
                    .rfold(init, f);
            }

            let backward_partial_acc = range_inclusive(this.b_min, this.backward.1)
                .map(|b| (this.backward.0, b))
                .rfold(init, &mut f);

            let middle_partials_acc = range_exclusive(this.forward.0, this.backward.0).rfold(
                backward_partial_acc,
                |inner_acc, a| {
                    range_inclusive(this.b_min, this.b_max)
                        .map(|b| (a, b))
                        .rfold(inner_acc, &mut f)
                },
            );

            range_inclusive(this.forward.1, this.b_max)
                .map(|b| (this.forward.0, b))
                .rfold(middle_partials_acc, f)
        }

        if self.forward.1 == self.b_min && self.backward.1 == self.b_max {
            range_inclusive(self.forward.0, self.backward.0).rfold(init, |acc, a| {
                range_inclusive(self.b_min, self.b_max)
                    .map(|b| (a, b))
                    .rfold(acc, &mut f)
            })
        } else {
            cold_call(self, init, f)
        }
    }
}

/// An enum for controlling the iteration order of a [`GridIter`]. Thinking of a [`GridIter`]
/// as a nested for-loop, `XMajor` would correspond to
///
/// ```
/// for x in 0..=10 {
///     for y in 0..=10 {
///         // Do things
///     }
/// }
/// ```
///
/// whereas `YMajor` would coorespond to
///
/// ```
/// for y in 0..=10 {
///     for x in 0..=10 {
///         // Do things
///     }
/// }
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Order {
    XMajor,
    YMajor,
}

/// An iterator over a grid of [`RoomXY`], inclusive of the boundary edges.
#[derive(Debug, Clone)]
pub struct GridIter {
    inner: PairIter,
    order: Order,
}

impl GridIter {
    /// Creates a `GridIter` over the rectangular grid of `RoomXY` specified by the top-left
    /// and bottom-right corners provided. Will determine whether to iterate `x` or `y` first
    /// using the passed-in [`Order`].
    ///
    /// It is safe to pass in invalid corner specifications (e.g. `top_left.x > bottom_right.x`),
    /// the returned `GridIter` will be immediately completed.
    ///
    /// # Example
    ///
    /// ```
    /// use screeps_utils::room_xy::{GridIter, Order};
    /// use screeps::local::{RoomXY, RoomCoordinate};
    ///
    /// for xy in GridIter::new(
    ///     RoomXY {
    ///         x: RoomCoordinate::new(0).unwrap(),
    ///         y: RoomCoordinate::new(0).unwrap(),
    ///     },
    ///     RoomXY {
    ///         x: RoomCoordinate::new(1).unwrap(),
    ///         y: RoomCoordinate::new(2).unwrap(),
    ///     },
    ///     Order::XMajor
    /// ) {
    ///     // Will print (x: 0, y: 0), then (x: 0, y: 1), (x: 0, y: 2), (x: 1, y: 0), etc.
    ///     println!("{:?}", xy);
    /// }
    /// ```
    pub fn new(top_left: RoomXY, bottom_right: RoomXY, order: Order) -> Self {
        let top = top_left.y;
        let bottom = bottom_right.y;
        let left = top_left.x;
        let right = bottom_right.x;
        let (a_min, a_max, b_min, b_max) = match order {
            Order::XMajor => (left, right, top, bottom),
            Order::YMajor => (top, bottom, left, right),
        };
        Self {
            inner: PairIter::new((a_min, b_min), (a_max, b_max)),
            order,
        }
    }

    fn get_xy(&self, a: RoomCoordinate, b: RoomCoordinate) -> RoomXY {
        match self.order {
            Order::XMajor => RoomXY { x: a, y: b },
            Order::YMajor => RoomXY { x: b, y: a },
        }
    }
}

impl Iterator for GridIter {
    type Item = RoomXY;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(a, b)| self.get_xy(a, b))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        match self.order {
            Order::XMajor => self
                .inner
                .fold(init, move |acc, (x, y)| f(acc, RoomXY { x, y })),
            Order::YMajor => self
                .inner
                .fold(init, move |acc, (y, x)| f(acc, RoomXY { x, y })),
        }
    }
}

impl FusedIterator for GridIter {}

impl ExactSizeIterator for GridIter {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl DoubleEndedIterator for GridIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|(a, b)| self.get_xy(a, b))
    }

    fn rfold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        match self.order {
            Order::XMajor => self
                .inner
                .rfold(init, move |acc, (x, y)| f(acc, RoomXY { x, y })),
            Order::YMajor => self
                .inner
                .rfold(init, move |acc, (y, x)| f(acc, RoomXY { x, y })),
        }
    }
}

/// Creates an iterator over all [`RoomXY`] around the designated centre (including the centre)
/// within the given [Chebyshev distance](https://en.wikipedia.org/wiki/Chebyshev_distance).
/// This is the same distance measure used for attack ranges, or for road lengths between two points, etc.
///
/// # Iteration order
///
/// The order over which points are iterated within the range is unspecified, and may change
/// at any time.
pub fn chebyshev_range_iter(centre: RoomXY, radius: u8) -> impl Iterator<Item = RoomXY> {
    let signed_radius = radius.min(50) as i8;
    let top_left = RoomXY {
        x: centre.x.saturating_add(-signed_radius),
        y: centre.y.saturating_add(-signed_radius),
    };
    let bottom_right = RoomXY {
        x: centre.x.saturating_add(signed_radius),
        y: centre.y.saturating_add(signed_radius),
    };
    GridIter::new(top_left, bottom_right, Order::YMajor)
}

/// Creates an iterator over all [`RoomXY`] around the designated centre (including the centre)
/// within the given [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry).
/// This would be used for, e.g., measuring the number of walls needed between 2 points.
///
/// # Iteration order
///
/// The order over which points are iterated within the range is unspecified, and may change
/// at any time.
pub fn manhattan_range_iter(centre: RoomXY, radius: u8) -> impl Iterator<Item = RoomXY> {
    let signed_radius = radius.min(100) as i8;
    let min_x = centre.x.saturating_add(-signed_radius);
    let min_x_offset = min_x.u8() as i8 - centre.x.u8() as i8;
    let max_x = centre.x.saturating_add(signed_radius);
    let max_x_offset = max_x.u8() as i8 - centre.x.u8() as i8;
    range_inclusive(min_x, max_x)
        .zip(min_x_offset..=max_x_offset)
        .flat_map(move |(x, x_offset)| {
            let y_radius = signed_radius - x_offset.abs();
            let min_y = centre.y.saturating_add(-y_radius);
            let max_y = centre.y.saturating_add(y_radius);
            range_inclusive(min_y, max_y).map(move |y| RoomXY { x, y })
        })
}

#[cfg(test)]
mod test {
    use super::*;

    use std::collections::HashSet;

    fn make_xy(x: u8, y: u8) -> RoomXY {
        RoomXY {
            x: RoomCoordinate::new(x).unwrap(),
            y: RoomCoordinate::new(y).unwrap(),
        }
    }

    #[test]
    fn test_chebyshev_basic() {
        let expected: HashSet<_> = [
            make_xy(10, 10),
            make_xy(9, 10),
            make_xy(9, 9),
            make_xy(9, 11),
            make_xy(10, 9),
            make_xy(10, 11),
            make_xy(11, 9),
            make_xy(11, 10),
            make_xy(11, 11),
        ]
        .into_iter()
        .collect();
        let actual: HashSet<_> = chebyshev_range_iter(make_xy(10, 10), 1).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_chebyshev_0_radius() {
        let expected: HashSet<_> = [make_xy(11, 11)].into_iter().collect();
        let actual: HashSet<_> = chebyshev_range_iter(make_xy(11, 11), 0).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_chebyshev_boundary() {
        let expected: HashSet<_> = [
            make_xy(0, 0),
            make_xy(0, 1),
            make_xy(0, 2),
            make_xy(1, 0),
            make_xy(1, 1),
            make_xy(1, 2),
            make_xy(2, 0),
            make_xy(2, 1),
            make_xy(2, 2),
        ]
        .into_iter()
        .collect();
        let actual: HashSet<_> = chebyshev_range_iter(make_xy(0, 0), 2).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_manhattan_basic() {
        let expected: HashSet<_> = [
            make_xy(9, 10),
            make_xy(10, 10),
            make_xy(11, 10),
            make_xy(10, 9),
            make_xy(10, 11),
        ]
        .into_iter()
        .collect();
        let actual: HashSet<_> = manhattan_range_iter(make_xy(10, 10), 1).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_manhattan_0_radius() {
        let expected: HashSet<_> = [make_xy(10, 10)].into_iter().collect();
        let actual: HashSet<_> = manhattan_range_iter(make_xy(10, 10), 0).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_manhattan_boundary() {
        let expected: HashSet<_> = [
            make_xy(0, 1),
            make_xy(0, 2),
            make_xy(0, 3),
            make_xy(1, 0),
            make_xy(1, 1),
            make_xy(1, 2),
            make_xy(1, 3),
            make_xy(1, 4),
            make_xy(2, 1),
            make_xy(2, 2),
            make_xy(2, 3),
            make_xy(3, 2),
        ]
        .into_iter()
        .collect();
        let actual: HashSet<_> = manhattan_range_iter(make_xy(1, 2), 2).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_grid_iter_basic() {
        let mut iter = GridIter::new(make_xy(0, 0), make_xy(1, 1), Order::XMajor);
        assert_eq!(make_xy(0, 0), iter.next().unwrap());
        assert_eq!(make_xy(0, 1), iter.next().unwrap());
        assert_eq!(make_xy(1, 0), iter.next().unwrap());
        assert_eq!(make_xy(1, 1), iter.next().unwrap());
        assert_eq!(None, iter.next());

        iter = GridIter::new(make_xy(0, 0), make_xy(1, 1), Order::YMajor);
        assert_eq!(make_xy(0, 0), iter.next().unwrap());
        assert_eq!(make_xy(1, 0), iter.next().unwrap());
        assert_eq!(make_xy(0, 1), iter.next().unwrap());
        assert_eq!(make_xy(1, 1), iter.next().unwrap());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_grid_iter_reverse() {
        let mut iter = GridIter::new(make_xy(0, 0), make_xy(1, 1), Order::XMajor);
        assert_eq!(make_xy(1, 1), iter.next_back().unwrap());
        assert_eq!(make_xy(1, 0), iter.next_back().unwrap());
        assert_eq!(make_xy(0, 1), iter.next_back().unwrap());
        assert_eq!(make_xy(0, 0), iter.next_back().unwrap());
        assert_eq!(None, iter.next_back());

        iter = GridIter::new(make_xy(0, 0), make_xy(1, 1), Order::YMajor);
        assert_eq!(make_xy(1, 1), iter.next_back().unwrap());
        assert_eq!(make_xy(0, 1), iter.next_back().unwrap());
        assert_eq!(make_xy(1, 0), iter.next_back().unwrap());
        assert_eq!(make_xy(0, 0), iter.next_back().unwrap());
        assert_eq!(None, iter.next_back());
    }

    #[test]
    fn test_grid_iter_len() {
        let mut iter = GridIter::new(make_xy(0, 0), make_xy(2, 1), Order::XMajor);
        for i in (1..=6).rev() {
            assert_eq!(iter.len(), i);
            iter.next().unwrap();
        }
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);

        iter = GridIter::new(make_xy(0, 0), make_xy(2, 1), Order::XMajor);
        for i in (1..=6).rev() {
            assert_eq!(iter.len(), i);
            iter.next_back().unwrap();
        }
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_grid_iter_bad_corners() {
        let mut iter = GridIter::new(make_xy(10, 10), make_xy(10, 9), Order::XMajor);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);

        iter = GridIter::new(make_xy(10, 10), make_xy(9, 10), Order::XMajor);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);

        iter = GridIter::new(make_xy(10, 10), make_xy(9, 9), Order::XMajor);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_grid_iter_single_square() {
        let coords: Vec<_> =
            GridIter::new(make_xy(25, 25), make_xy(25, 25), Order::XMajor).collect();
        assert_eq!(coords, [make_xy(25, 25)]);
    }

    #[test]
    fn test_grid_iter_mixing_forward_and_back() {
        let mut iter = GridIter::new(make_xy(0, 0), make_xy(2, 1), Order::XMajor);
        assert_eq!(iter.next().unwrap(), make_xy(0, 0));
        assert_eq!(iter.next_back().unwrap(), make_xy(2, 1));
        assert_eq!(iter.next_back().unwrap(), make_xy(2, 0));
        assert_eq!(iter.next().unwrap(), make_xy(0, 1));
        assert_eq!(iter.next().unwrap(), make_xy(1, 0));
        assert_eq!(iter.next_back().unwrap(), make_xy(1, 1));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_grid_iter_fold() {
        let expected = [
            make_xy(0, 0),
            make_xy(0, 1),
            make_xy(0, 2),
            make_xy(1, 0),
            make_xy(1, 1),
            make_xy(1, 2),
        ];
        let actual = GridIter::new(make_xy(0, 0), make_xy(1, 2), Order::XMajor).fold(
            Vec::new(),
            |mut v, xy| {
                v.push(xy);
                v
            },
        );
        assert_eq!(expected.as_slice(), actual);
    }

    #[test]
    fn test_grid_iter_rfold() {
        let expected = [
            make_xy(1, 2),
            make_xy(0, 2),
            make_xy(1, 1),
            make_xy(0, 1),
            make_xy(1, 0),
            make_xy(0, 0),
        ];
        let actual = GridIter::new(make_xy(0, 0), make_xy(1, 2), Order::YMajor).rfold(
            Vec::new(),
            |mut v, xy| {
                v.push(xy);
                v
            },
        );
        assert_eq!(expected.as_slice(), actual);
    }

    #[test]
    fn test_grid_iter_single_row_fold() {
        let mut base_iter = GridIter::new(make_xy(0, 0), make_xy(0, 10), Order::XMajor);
        base_iter.next().unwrap();
        base_iter.next_back().unwrap();
        let expected: Vec<_> = (1..=9).map(|y| make_xy(0, y)).collect();
        let actual = base_iter.clone().fold(Vec::new(), |mut v, xy| {
            v.push(xy);
            v
        });
        assert_eq!(expected, actual);

        let expected: Vec<_> = (1..=9).rev().map(|y| make_xy(0, y)).collect();
        let actual = base_iter.rfold(Vec::new(), |mut v, xy| {
            v.push(xy);
            v
        });
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_grid_iter_fold_general_case() {
        let mut base_iter = GridIter::new(make_xy(0, 0), make_xy(3, 1), Order::XMajor);
        base_iter.next().unwrap();
        base_iter.next_back().unwrap();
        let mut expected = [
            make_xy(0, 1),
            make_xy(1, 0),
            make_xy(1, 1),
            make_xy(2, 0),
            make_xy(2, 1),
            make_xy(3, 0),
        ];
        let actual = base_iter.clone().fold(Vec::new(), |mut v, xy| {
            v.push(xy);
            v
        });
        assert_eq!(expected.as_slice(), actual);

        expected.reverse();
        let actual = base_iter.rfold(Vec::new(), |mut v, xy| {
            v.push(xy);
            v
        });
        assert_eq!(expected.as_slice(), actual);
    }
}
