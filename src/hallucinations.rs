use rand_distr::Distribution;
use fitrs::{Fits, Hdu};
use std::time::Instant;


pub fn hallucinate_normal_distribution(size:usize, fits_path:&str, ave:f64,std:f64){
    let now = Instant::now();
    let shape = [size, size];
    let normal = rand_distr::Normal::new(ave,std).unwrap();
    let data = (0..shape[0])
        .map(|i| (0..shape[1]).map(|_j| {
            normal.sample(&mut rand::rng())
        }))
        .flatten()
        .collect();
    let mut primary_hdu = Hdu::new(&shape, data);
    primary_hdu.insert("DISTRIBUTION", format!("Normal, ave: {:?} std: {:?}", ave,std));
    Fits::create(fits_path, primary_hdu).expect("Failed to create fits for dark current");
    println!("Hallucinated you a {size}*{size} normally distributed file in {:?} ms at {:?}", now.elapsed().as_millis(),fits_path)
}

pub fn hallucinate_custom_distribution(size:usize, fits_path:&str, weights:Vec<f64>, values:Vec<f64>){
    assert_eq!(weights.len(), values.len(), "Tried to make a custom distribution with different numer of weights/values");
    let now = Instant::now();
    let shape = [size, size];
    let mut weights = rand_distr::weighted::WeightedIndex::new(&weights).expect("Invalid weights");
    let data = (0..shape[0])
        .map(|i| (0..shape[1]).map(|_j| {
            values[weights.sample(&mut rand::rng())]
        }))
        .flatten()
        .collect();
    let mut primary_hdu = Hdu::new(&shape, data);
    primary_hdu.insert("DISTRIBUTION", format!("Custom, weights: {:?} values: {:?}", weights, values));
    Fits::create(fits_path, primary_hdu).expect("Failed to create fits for custom values");
    println!("Hallucinated you a {size}*{size} custom distributed file in {:?} ms at {:?}", now.elapsed().as_millis(),fits_path)
}


pub fn hallucinate_dead_pixel_map(size: usize,
                                  fits_path:&str,
                                  num_scattered_pixels:usize,
                                  num_rows: usize,
                                  num_columns: usize,
                                  num_rectangles:usize){

    let now = Instant::now();
    let shape = [size, size];
    let mut data:Vec<f64> = (0..size*size).map(|_i| 1.0).collect();
    let uniform_distribution = rand_distr::Uniform::new(0,size*size).unwrap();
    for _i in (0..num_scattered_pixels){
        let index = uniform_distribution.sample(&mut rand::rng());
        data[index] = 0.0
    };

    let mut data_2d:Vec<Vec<f64>> = data.chunks(size).map(|vec| vec.to_vec()).collect();
    let uniform_distribution = rand_distr::Uniform::new(0,size).unwrap();
    for _j in (0..num_rows){
        let index = uniform_distribution.sample(&mut rand::rng());
        for column in (0..size){
            data_2d[index][column] = 0.0
        }
    }

    for _j in (0..num_rows){
        let index = uniform_distribution.sample(&mut rand::rng());
        for column in (0..size){
            data_2d[column][index] = 0.0
        }
    }

    let uniform_distribution = rand_distr::Uniform::new(0,size-200).unwrap();
    let size_distribution = rand_distr::Uniform::new(100,199).unwrap();
    for _j in (0..num_rectangles){
        let corner_x = uniform_distribution.sample(&mut rand::rng());
        let corner_y = uniform_distribution.sample(&mut rand::rng());
        let x_size = size_distribution.sample(&mut rand::rng());
        let y_size = size_distribution.sample(&mut rand::rng());
        for column in (0..y_size){
            for row in 0..x_size{
                data_2d[column + corner_y][row + corner_x] = 0.0
            }
        }

    }
    let data = data_2d.iter().map(|v| v.to_owned()).flatten().collect();
    let mut primary_hdu = Hdu::new(&shape, data);

    Fits::create(fits_path, primary_hdu).expect("Failed to create fits for custom values");
    println!("Hallucinated you a {size}*{size} dead pixel map in {:?} ms at {:?}", now.elapsed().as_millis(),fits_path)


}






