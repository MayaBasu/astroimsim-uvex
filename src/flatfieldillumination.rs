use std::time::Instant;
use uvex_fitrs::{Fits, FitsData, FitsDataArray};
use crate::psf::{DataFile, Load, PSF};

use uvex_fitrs::{ Hdu};
use crate::coordinate_system::{CoordinateSystem, Coordinates};
use crate::coordinate_system::Coordinates::ABSOLUTE;
use crate::flatfieldillumination;
use crate::grid2d::GRID2D;
use crate::geometry::Point;
use crate::point_sources::{Bands, PointSource, SourceList, Spectrum};

use Rust
pub struct flatfield{
    pub grid:GRID2D,
    pub data: Vec<Vec<f64>>
}
pub fn load_file(file_path:&str) -> Vec<f64>{
    println!("Loading {:?}",file_path);
    let fits = Fits::open(file_path).expect("Failed to open FITS file");
    let primary_hdu= fits.iter().next().expect("Couldn't find primary HDU");
    let (mut data,shape) = match primary_hdu.read_data() {
        //FitsData::FloatingPoint32(FitsDataArray { shape, data }) => (data,shape),
        FitsData::FloatingPoint64(FitsDataArray { shape, data }) => (data,shape),

        _ => {panic!("huh")}
    }; //TODO add support for f64 etc

    println!("FILE SHAPE IS {:?} WHOOOOOOOOO!",shape);


    let grid = flatfield_grid();
    data

}

pub fn write(path:&str,data:&Vec<Vec<f32>>){
    // Make example dummy data array
    let width = data[0].len();
    let height = data.len();
    let shape = [width, height];
    let data = data.iter().flatten().map(|a|*a).collect();

    let mut primary_hdu = Hdu::new(&shape, data);
    // Insert values in header
    primary_hdu.insert("KEYSTR", "My string");
    primary_hdu.insert("KEYSTR2", "Whatever value you want to save in this FITS files. Continued (long) strings are supported, if you happen to care.");
    primary_hdu.insert("KEYFLOAT", 3.14);
    primary_hdu.insert("KEYINT", 42);

    // Save file
    Fits::create(path, primary_hdu).expect("Failed to create");

}
pub fn flatfield_grid()->GRID2D{

    let num_pixels =3*4096;
    let detector_width_degrees = 3.0;
    let center_absolute = Point::new(-0.5,0.0,Coordinates::ABSOLUTE);

    let pixel_to_deg_scale = detector_width_degrees/num_pixels as f64; //Degrees in FOV to pixels
    let detectors_x_axis = (pixel_to_deg_scale,0.0);
    let detectors_y_axis = (0.0,pixel_to_deg_scale);

    let coordinate_system = CoordinateSystem::new(
        detectors_x_axis,
        detectors_y_axis,
        center_absolute.values(),
        "Detector Coordinate System".to_string(),
        "magenta".to_string());

    GRID2D::new_empty(
        (num_pixels,num_pixels),(1.0,1.0), (0.0,0.0),0.01,Coordinates::RELATIVE(coordinate_system))

}


pub fn load_flatfield_illumination(size:usize)-> flatfield{
    let width = size as u32;
    let height = size as u32;
    let scale = size as f64/512.0;

    let now = Instant::now();
    let flatfield = flatfieldillumination::load_file("/Users/mayabasu/Desktop/uvex_psf_files/FUV_flat_field_illumination.fits");
    let flat:Vec<Vec<f64>> = flatfield.chunks(512).map(|v|v.to_vec()).collect();


    let mut data = Vec::new();
    for row in 0.. height{
        let mut row_vec = Vec::new();
        for column in 0..width{


            let mut val:f64 = 250.0;
            if (row < (flat.len() as f64*scale) as u32) &&(column < (flat[0].len() as f64*scale) as u32){
                val = ((val as f64) * (flat[(row as f64/scale).floor() as usize][((column as f64)/scale).floor() as usize])) as f64
            }
            //println!("val {val}");
            row_vec.push((val as f64));

        }
        data.push(row_vec)


    }

    //flatfieldillumination::write("/Users/mayabasu/Desktop/Output/slkfj.fits",&data);
    let sum:f64 = data.iter().flatten().sum();
    println!("Generated and wrote array in {:?}, sum{sum}", now.elapsed().as_millis());

    flatfield{
        grid:flatfield_grid(),
        data,
    }

}






