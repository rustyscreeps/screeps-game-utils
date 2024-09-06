use std::iter::FusedIterator;

use screeps::{Room, RoomCoordinate, RoomXY};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Order {
    ColumnMajor,
    RowMajor,
}

#[derive(Debug)]
pub struct GridIter {
    b_min: u8,
    b_max: u8,
    forward: (u8, u8),
    backward: (u8, u8),
    order: Order,
    done: bool,
}

impl GridIter {
    pub fn new(top_left: RoomXY, bottom_right: RoomXY, order: Order) -> Self {
        let top = top_left.y.u8();
        let bottom = bottom_right.y.u8();
        let left = top_left.x.u8();
        let right = bottom_right.x.u8();
        let (a_min, a_max, b_min, b_max) = match order {
            Order::ColumnMajor => (left, right, top, bottom),
            Order::RowMajor => (top, bottom, left, right),
        };
        Self {
            b_min,
            b_max,
            forward: (a_min, b_min),
            backward: (a_max, b_max),
            order,
            done: top > bottom || left > right,
        }
    }

    fn get_xy(&self, a: RoomCoordinate, b: RoomCoordinate) -> RoomXY {
        match self.order {
            Order::ColumnMajor => RoomXY { x: a, y: b },
            Order::RowMajor => RoomXY { x: b, y: a },
        }
    }
}

impl Iterator for GridIter {
    type Item = RoomXY;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        // SAFETY: self.forward will always be a value in the range
        // (a_min..=a_max, self.b_min..=self.b_max), which were
        // obtained from existing RoomXY values passed in Self::new.
        let res = unsafe {
            Some(self.get_xy(
                RoomCoordinate::unchecked_new(self.forward.0),
                RoomCoordinate::unchecked_new(self.forward.1),
            ))
        };

        if self.forward == self.backward {
            self.done = true;
        } else if self.forward.1 == self.b_max {
            self.forward = (self.forward.0 + 1, self.b_min);
        } else {
            self.forward.1 += 1;
        }

        res
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        return (len, Some(len));
    }

    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        // SAFETY for all RoomCoordinate::unchecked_new usages:
        // All the passed in u8 values are derived from forward, backward,
        // b_min, or b_max. Forward and backward will always be in the range
        // (a_min..=a_max, b_min..=b_max) established in Self::new, and all
        // four of those come directly from existing RoomXY instances.
        if self.done {
            return init;
        }

        let a = unsafe { RoomCoordinate::unchecked_new(self.forward.0) };

        if self.forward.0 == self.backward.0 {
            return (self.forward.1..=self.backward.1)
                .map(|b| self.get_xy(a, unsafe { RoomCoordinate::unchecked_new(b) }))
                .fold(init, f);
        }

        match self.order {
            Order::ColumnMajor => {
                let first_column_acc = {
                    let x = a;
                    (self.forward.1..=self.b_max)
                        .map(|y_raw| unsafe { RoomCoordinate::unchecked_new(y_raw) })
                        .map(|y| RoomXY { x, y })
                        .fold(init, &mut f)
                };

                let middle_columns_acc = (self.forward.0 + 1..self.backward.0)
                    .map(|x_raw| unsafe { RoomCoordinate::unchecked_new(x_raw) })
                    .fold(first_column_acc, |inner_acc, x| {
                        (self.b_min..=self.b_max)
                            .map(|y_raw| unsafe { RoomCoordinate::unchecked_new(y_raw) })
                            .map(|y| RoomXY { x, y })
                            .fold(inner_acc, &mut f)
                    });

                {
                    let x = unsafe { RoomCoordinate::unchecked_new(self.backward.0) };
                    (self.b_min..=self.backward.1)
                        .map(|y_raw| unsafe { RoomCoordinate::unchecked_new(y_raw) })
                        .map(|y| RoomXY { x, y })
                        .fold(middle_columns_acc, f)
                }
            }
            Order::RowMajor => {
                let first_row_acc = {
                    let y = a;
                    (self.forward.1..=self.b_max)
                        .map(|x_raw| unsafe { RoomCoordinate::unchecked_new(x_raw) })
                        .map(|x| RoomXY { x, y })
                        .fold(init, &mut f)
                };

                let middle_rows_acc = (self.forward.0 + 1..self.backward.0)
                    .map(|y_raw| unsafe { RoomCoordinate::unchecked_new(y_raw) })
                    .fold(first_row_acc, |inner_acc, y| {
                        (self.b_min..=self.b_max)
                            .map(|x_raw| unsafe { RoomCoordinate::unchecked_new(x_raw) })
                            .map(|x| RoomXY { x, y })
                            .fold(inner_acc, &mut f)
                    });

                {
                    let y = unsafe { RoomCoordinate::unchecked_new(self.backward.0) };
                    (self.b_min..=self.backward.1)
                        .map(|x_raw| unsafe { RoomCoordinate::unchecked_new(x_raw) })
                        .map(|x| RoomXY { x, y })
                        .fold(middle_rows_acc, f)
                }
            }
        }
    }
}

impl FusedIterator for GridIter {}

impl ExactSizeIterator for GridIter {
    fn len(&self) -> usize {
        if self.done {
            return 0;
        }

        if self.forward.0 == self.backward.0 {
            return (self.backward.1 - self.forward.1 + 1) as usize;
        }

        let full_ranges = (self.backward.0 - self.forward.0 - 1) as usize;
        full_ranges * (self.b_max - self.b_min + 1) as usize
            + (self.b_max - self.forward.1 + 1) as usize
            + (self.backward.1 - self.b_min + 1) as usize
    }
}

impl DoubleEndedIterator for GridIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        // SAFETY: self.backward will always be a value in the range
        // (a_min..=a_max, self.b_min..=self.b_max), which were
        // obtained from existing RoomXY values passed in Self::new.
        let res = unsafe {
            Some(self.get_xy(
                RoomCoordinate::unchecked_new(self.backward.0),
                RoomCoordinate::unchecked_new(self.backward.1),
            ))
        };

        if self.backward == self.forward {
            self.done = true;
        } else if self.backward.1 == self.b_min {
            self.backward = (self.backward.0 - 1, self.b_max);
        } else {
            self.backward.1 -= 1;
        }

        res
    }

    fn rfold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        // SAFETY for all RoomCoordinate::unchecked_new usages:
        // All the passed in u8 values are derived from forward, backward,
        // b_min, or b_max. Forward and backward will always be in the range
        // (a_min..=a_max, b_min..=b_max) established in Self::new, and all
        // four of those come directly from existing RoomXY instances.
        if self.done {
            return init;
        }

        let a = unsafe { RoomCoordinate::unchecked_new(self.backward.0) };

        if self.backward.0 == self.forward.0 {
            return (self.forward.1..=self.backward.1)
                .map(|b| self.get_xy(a, unsafe { RoomCoordinate::unchecked_new(b) }))
                .rfold(init, f);
        }

        match self.order {
            Order::ColumnMajor => {
                let last_column_acc = {
                    let x = a;
                    (self.b_min..=self.backward.1)
                        .map(|y_raw| unsafe { RoomCoordinate::unchecked_new(y_raw) })
                        .map(|y| RoomXY { x, y })
                        .rfold(init, &mut f)
                };

                let middle_columns_acc = (self.forward.0 + 1..self.backward.0)
                    .map(|x_raw| unsafe { RoomCoordinate::unchecked_new(x_raw) })
                    .rfold(last_column_acc, |inner_acc, x| {
                        (self.b_min..=self.b_max)
                            .map(|y_raw| unsafe { RoomCoordinate::unchecked_new(y_raw) })
                            .map(|y| RoomXY { x, y })
                            .rfold(inner_acc, &mut f)
                    });

                {
                    let x = unsafe { RoomCoordinate::unchecked_new(self.forward.0) };
                    (self.forward.1..=self.b_max)
                        .map(|y_raw| unsafe { RoomCoordinate::unchecked_new(y_raw) })
                        .map(|y| RoomXY { x, y })
                        .rfold(middle_columns_acc, f)
                }
            }
            Order::RowMajor => {
                let last_row_acc = {
                    let y = a;
                    (self.b_min..=self.backward.1)
                        .map(|x_raw| unsafe { RoomCoordinate::unchecked_new(x_raw) })
                        .map(|x| RoomXY { x, y })
                        .rfold(init, &mut f)
                };

                let middle_rows_acc = (self.forward.0 + 1..self.backward.0)
                    .map(|y_raw| unsafe { RoomCoordinate::unchecked_new(y_raw) })
                    .rfold(last_row_acc, |inner_acc, y| {
                        (self.b_min..=self.b_max)
                            .map(|x_raw| unsafe { RoomCoordinate::unchecked_new(x_raw) })
                            .map(|x| RoomXY { x, y })
                            .rfold(inner_acc, &mut f)
                    });

                {
                    let y = unsafe { RoomCoordinate::unchecked_new(self.forward.0) };
                    (self.forward.1..=self.b_max)
                        .map(|x_raw| unsafe { RoomCoordinate::unchecked_new(x_raw) })
                        .map(|x| RoomXY { x, y })
                        .rfold(middle_rows_acc, f)
                }
            }
        }
    }
}

pub fn grid_iter(top_left: RoomXY, bottom_right: RoomXY, order: Order) -> GridIter {
    GridIter::new(top_left, bottom_right, order)
}

pub fn chebyshev_range_iter(centre: RoomXY, radius: u8) -> impl Iterator<Item = RoomXY> {
    let signed_radius = radius.min(50) as i8;
    (-signed_radius..=signed_radius)
        .filter_map(move |x| centre.x.checked_add(x))
        .flat_map(move |x| {
            (-signed_radius..=signed_radius)
                .filter_map(move |y| centre.y.checked_add(y))
                .map(move |y| RoomXY { x, y })
        })
}

pub fn manhattan_range_iter(centre: RoomXY, radius: u8) -> impl Iterator<Item = RoomXY> {
    let signed_radius = radius.min(100) as i8;
    (-signed_radius..=signed_radius)
        .filter_map(move |x| centre.x.checked_add(x).map(|x_coord| (x, x_coord)))
        .flat_map(move |(x_offset, x)| {
            let y_range = signed_radius - x_offset.abs();
            (-y_range..=y_range)
                .filter_map(move |y| centre.y.checked_add(y))
                .map(move |y| RoomXY { x, y })
        })
}

#[cfg(test)]
mod test {
    use super::*;
}
