#![feature(test)]
extern crate test;

#[cfg(test)]
mod benches {
    use screeps::local::{LocalCostMatrix, RoomCoordinate, RoomXY};
    use screeps_utils::{room_coordinate::*, room_xy::*};
    use test::{black_box, Bencher};

    fn make_xy(x: u8, y: u8) -> RoomXY {
        RoomXY {
            x: RoomCoordinate::new(x).unwrap(),
            y: RoomCoordinate::new(y).unwrap(),
        }
    }

    #[bench]
    fn bench_full_grid_iter_cm_set(b: &mut Bencher) {
        let mut cm = LocalCostMatrix::new();
        b.iter(|| {
            GridIter::new(
                black_box(make_xy(0, 0)),
                black_box(make_xy(49, 49)),
                Order::XMajor,
            )
            .for_each(|xy| black_box(&mut cm).set(xy, 255));
        });
    }

    #[bench]
    fn bench_full_for_loop_cm_set(b: &mut Bencher) {
        let mut cm = LocalCostMatrix::new();
        b.iter(|| {
            for x in black_box(0)..black_box(50) {
                let x = RoomCoordinate::new(x).unwrap();
                for y in black_box(0)..black_box(50) {
                    let y = RoomCoordinate::new(y).unwrap();
                    black_box(&mut cm).set(RoomXY { x, y }, 255);
                }
            }
        });
    }

    #[bench]
    fn bench_full_range_iter_cm_set(b: &mut Bencher) {
        let mut cm = LocalCostMatrix::new();
        let zero = RoomCoordinate::new(0).unwrap();
        let max = RoomCoordinate::new(49).unwrap();
        b.iter(|| {
            for x in range_inclusive(black_box(zero), black_box(max)) {
                for y in range_inclusive(black_box(zero), black_box(max)) {
                    black_box(&mut cm).set(RoomXY { x, y }, 255);
                }
            }
        });
    }

    #[bench]
    fn bench_chebyshev_iter_cm_set(b: &mut Bencher) {
        let mut cm = LocalCostMatrix::new();
        let centre = make_xy(10, 10);
        b.iter(|| {
            chebyshev_range_iter(black_box(centre), 3)
                .for_each(|xy| black_box(&mut cm).set(xy, 255));
        });
    }

    #[bench]
    fn bench_chebyshev_saturating_cm_set(b: &mut Bencher) {
        let mut cm = LocalCostMatrix::new();
        let centre = make_xy(10, 10);
        b.iter(|| {
            let min_x = black_box(centre).x.saturating_add(-3);
            let max_x = black_box(centre).x.saturating_add(3);
            let min_y = black_box(centre).y.saturating_add(-3);
            let max_y = black_box(centre).y.saturating_add(3);
            for x in range_inclusive(min_x, max_x) {
                for y in range_inclusive(min_y, max_y) {
                    black_box(&mut cm).set(RoomXY { x, y }, 255);
                }
            }
        });
    }

    #[bench]
    fn bench_chebyshev_checked_cm_set(b: &mut Bencher) {
        let mut cm = LocalCostMatrix::new();
        let centre = make_xy(10, 10);
        b.iter(|| {
            for x in (-3..=3).filter_map(|offset| black_box(centre).x.checked_add(offset)) {
                for y in (-3..=3).filter_map(|offset| black_box(centre).y.checked_add(offset)) {
                    black_box(&mut cm).set(RoomXY { x, y }, 255);
                }
            }
        });
    }

    #[bench]
    fn bench_manhattan_iter_cm_set(b: &mut Bencher) {
        let mut cm = LocalCostMatrix::new();
        let centre = make_xy(10, 10);
        b.iter(|| {
            manhattan_range_iter(black_box(centre), 3)
                .for_each(|xy| black_box(&mut cm).set(xy, 255));
        });
    }

    #[bench]
    fn bench_manhattan_saturating_cm_set(b: &mut Bencher) {
        let mut cm = LocalCostMatrix::new();
        let centre = make_xy(10, 10);
        b.iter(|| {
            let min_x = black_box(centre).x.saturating_add(-3);
            let max_x = black_box(centre).x.saturating_add(3);
            for (x, offset) in range_inclusive(min_x, max_x).zip(-3_i8..=3) {
                let y_radius = 3 - offset.abs();
                let min_y = black_box(centre).y.saturating_add(-y_radius);
                let max_y = black_box(centre).y.saturating_add(y_radius);
                for y in range_inclusive(min_y, max_y) {
                    black_box(&mut cm).set(RoomXY { x, y }, 255);
                }
            }
        });
    }

    #[bench]
    fn bench_manhattan_checked_cm_set(b: &mut Bencher) {
        let mut cm = LocalCostMatrix::new();
        let centre = make_xy(10, 10);
        b.iter(|| {
            for (x, offset) in (-3..=3)
                .filter_map(|offset| black_box(centre).x.checked_add(offset).map(|x| (x, offset)))
            {
                let y_radius = 3 - offset.abs();
                for y in (-y_radius..=y_radius)
                    .filter_map(|offset| black_box(centre).y.checked_add(offset))
                {
                    black_box(&mut cm).set(RoomXY { x, y }, 255);
                }
            }
        })
    }
}
