use std::iter::FusedIterator;

use screeps::{RoomCoordinate, RoomXY};

use crate::room_coordinate::{range_exclusive, range_inclusive};

#[derive(Debug, Clone)]
struct PairIter {
    // SAFETY INVARIANT: forward.1 and backward.1 are within
    // b_min..=b_max at all times
    b_min: RoomCoordinate,
    b_max: RoomCoordinate,
    // SAFETY INVARIANT: forward <= backward at all times
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
            // SAFETY: self.backward.1 <= self.b_max, so
            // self.forward.0 < self.backward.0, meaning we can
            // increment by 1.
            self.forward = (
                unsafe { RoomCoordinate::unchecked_new(self.forward.0.u8() + 1) },
                self.b_min,
            );
        } else {
            // SAFETY: self.forward.1 < self.b_max, so we can step
            // up by 1.
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

        if self.forward.0 == self.backward.0 {
            return range_inclusive(self.forward.1, self.backward.1)
                .map(|b| (self.forward.0, b))
                .fold(init, f);
        }

        let forward_partial_acc = range_inclusive(self.forward.1, self.b_max)
            .map(|b| (self.forward.0, b))
            .fold(init, &mut f);

        let middle_partials_acc = range_exclusive(
            unsafe { RoomCoordinate::unchecked_new(self.forward.0.u8() + 1) },
            self.backward.0,
        )
        .fold(forward_partial_acc, |inner_acc, a| {
            range_inclusive(self.b_min, self.b_max)
                .map(|b| (a, b))
                .fold(inner_acc, &mut f)
        });

        range_inclusive(self.b_min, self.backward.1)
            .map(|b| (self.backward.0, b))
            .fold(middle_partials_acc, f)
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
            // SAFETY: self.forward.1 >= self.b_min, so
            // self.forward.0 < self.backward.0, meaning we can
            // decrement by 1.
            self.backward = (
                unsafe { RoomCoordinate::unchecked_new(self.backward.0.u8() - 1) },
                self.b_max,
            );
        } else {
            // SAFETY: self.backward.1 > self.b_min, so we can step
            // down by 1.
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

        if self.forward.0 == self.backward.0 {
            return range_inclusive(self.forward.1, self.backward.1)
                .map(|b| (self.forward.0, b))
                .rfold(init, f);
        }

        let backward_partial_acc = range_inclusive(self.b_min, self.backward.1)
            .map(|b| (self.backward.0, b))
            .rfold(init, &mut f);

        let middle_partials_acc = range_exclusive(
            unsafe { RoomCoordinate::unchecked_new(self.forward.0.u8() + 1) },
            self.backward.0,
        )
        .rfold(backward_partial_acc, |inner_acc, a| {
            range_inclusive(self.b_min, self.b_max)
                .map(|b| (a, b))
                .rfold(inner_acc, &mut f)
        });

        range_inclusive(self.forward.0, self.b_max)
            .map(|b| (self.forward.0, b))
            .rfold(middle_partials_acc, f)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Order {
    XMajor,
    YMajor,
}

#[derive(Debug, Clone)]
pub struct GridIter {
    inner: PairIter,
    order: Order,
}

impl GridIter {
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

pub fn grid_iter(top_left: RoomXY, bottom_right: RoomXY, order: Order) -> GridIter {
    GridIter::new(top_left, bottom_right, order)
}

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
}
