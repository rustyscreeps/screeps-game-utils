#![feature(test)]
extern crate test;

#[cfg(test)]
mod benches {
    use screeps::local::LocalRoomTerrain;
    use screeps_utils::algorithms::distance_transform::*;
    use test::{black_box, Bencher};

    #[bench]
    fn bench_chebyshev_distance_transform(b: &mut Bencher) {
        let terrain = LocalRoomTerrain::new_from_bits(Box::new([0; 2500]));
        b.iter(|| black_box(chebyshev_distance_transform_from_terrain(&terrain)));
    }
}
