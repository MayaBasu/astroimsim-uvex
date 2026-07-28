use astroimsim_data::prelude::{DetectorArray, SpatialEffect};
use astroimsim_geometry::coordinate_system::{CoordinateSystem, Coordinates};
use astroimsim_geometry::grid2d::GRID2D;
use astroimsim_geometry::points::Point;
use crate::uvex_telescope::SpatialEffectArray;

pub fn dead_pixels(detector_array:&DetectorArray) -> SpatialEffectArray{
    let dir = "/Users/mayabasu/Desktop/uvex/detector_effects/fuv_dead_pixels";
    let mut dead_pixels_vec : Vec<(usize, SpatialEffect)> = Vec::new();
    for i in 0..9{
        let path = format!("{dir}/fuv_dead_pixels_{:?}.fits",i);
        let grid = detector_array.detectors[i].grid.clone();

        let mut dead_pixels = SpatialEffect::new_empty(
            format!("Dead Pixel map for detector {:?}", i),
            grid,
            path.to_string());
        dead_pixels.load_data(0);
        dead_pixels_vec.push((i,dead_pixels))
    }
    SpatialEffectArray{
        label: "Dead Pixel Maps".to_string(),
        effects: dead_pixels_vec,
    }
}



pub fn read_noise(detector_array:&DetectorArray) -> SpatialEffectArray{
    let dir = "/Users/mayabasu/Desktop/uvex/detector_effects/fuv_read_noise";
    let mut effects: Vec<(usize, SpatialEffect)> = Vec::new();
    for i in 0..9{
        let path = format!("{dir}/fuv_read_noise_{:?}.fits",i);
        let grid = detector_array.detectors[i].grid.clone();

        let mut effect = SpatialEffect::new_empty(
            format!("Read noise map for detector {:?}", i),
            grid,
            path.to_string());
        effect.load_data(0);
        effects.push((i, effect))
    }
    SpatialEffectArray{
        label: "Read noise Maps".to_string(),
        effects,
    }
}


pub fn load_detector_effects(detector_array: &DetectorArray, file_name:&str, directory:&str)-> SpatialEffectArray{
    let mut effects: Vec<(usize, SpatialEffect)> = Vec::new();
    for i in 0..9{
        let path = format!("{directory}/{file_name}_{:?}.fits",i);
        let grid = detector_array.detectors[i].grid.clone();

        let mut effect = SpatialEffect::new_empty(
            format!("{file_name} for detector {:?}", i),
            grid,
            path.to_string());
        effect.load_data(0);
        effects.push((i, effect))
    }
    SpatialEffectArray{
        label: file_name.to_string(),
        effects,
    }
}
