use plotpy::{Image, Plot, StrError};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use crate::coordinate_system::{CoordinateSystem, Coordinates};
use crate::grid2d::{GRID2D, Location};

use std::time::{Duration, Instant};
use rand::distr::{Distribution, Uniform};
use rand_distr::Poisson;
use crate::coordinate_system::Coordinates::{ABSOLUTE, RELATIVE};
use crate::flatfieldillumination::write;
use crate::geometry::Point;
use crate::psf_grid::PsfGrid;
use crate::point_sources::{Bands, SourceList, Spectrum};

#[derive(Clone)]
pub struct Detector {
    pub label: String,
    pub(crate) grid: GRID2D,
    pub(crate) data: Vec<Vec<f32>>
}

//12-3, 9-11

impl Detector {
    pub fn show_read_out(&mut self, source_list: SourceList,psf_grid:PsfGrid){

        let start = Instant::now();
        let mut data = &mut self.data;
        /*
        //add background
        for row in 0..4000{
            let mut row_vec = Vec::new();
            for column in 0..4000{
                //if row < matrix_x && column < matrix_y{
                   // row_vec.push(matrix[column][row]);

                //}else{
                //    row_vec.push(0.0);
               // }
                //row_vec.push((((row + column) as f32)/100.0))
                row_vec.push(0.0)
            }
            data.push(row_vec);
        }

         */
        // println!("{}, {}",data.len(),data[0].len());

        let mut dropped = 0;
        for point in source_list.sources{
            let luminosity = match point.spectrum{
                Spectrum::Full(_, _,_) => {panic!("Implement me!! uwu")}
                Spectrum::Bands(bands,u) => {match bands[0]{
                    Bands::FUV(fuv_luminosity) => {//println!("Displaying the FUV channel");
                        fuv_luminosity}
                    Bands::NUV(nuv_luminosity) => {//println!("Displaying the FUV channel");
                        nuv_luminosity}
                }} //TODO have both FUV and NUV
            } as f32;
            let mut rng = rand::rng();


            match self.grid.inside_or_outside(&point.point){ //TODO remove many unneeded clone() calls by borrowing Points
                Location::Outside => {dropped +=1}
                Location::Inside => {let psf = psf_grid.interpolated_psf(&point.point);

                    let ((x_mod,y_mod),binned_psf) = self.grid.bin_up_patch(point.point,&psf,10);
                    //println!("{:?}",(x_mod,y_mod));
                    let binned_matrix_x = binned_psf[0].len();
                    let binned_matrix_y = binned_psf.len();

                    for row in 0..binned_matrix_y{
                        for column in 0..binned_matrix_x{


                            //println!("{}{}",column + y, row + y);
                            if column + y_mod < self.grid.x_num && row + x_mod < self.grid.y_num{



                                let flux =binned_psf[column][row]*luminosity;
                                if flux == 0.0{
                                    continue
                                }else{
                                    // println!("flux is {:?}", flux);
                                    let bin = Poisson::new(flux as f64).unwrap();
                                    data[column + y_mod][row + x_mod] += bin.sample(&mut rng) as f32;
                                }



                            }else{
                                // println!("dropping pixel");
                            }

                            // println!("modifying pixel {:?} to be {:?}",(row + x_mod,column + y_mod),binned_psf[column][row]);
                        }
                    }}
            }






        }


        // data[0][0]  += 100.0;

        let size = data.len();
        let size2 = data[0].len();
        write("/Users/mayabasu/Desktop/Output/slkfj.fits",data);
        let sum:f32  = data.iter().flatten().sum();
        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}, dropped {:?}", duration,dropped);

        println!("made array, sum is  :{}, size is {:?}, {:?}",sum, size,size2);





    }

}


pub struct DetectorArray{
    label: String,
    pub(crate) detectors: Vec<Detector>,
    coordinate_system: CoordinateSystem
}

impl DetectorArray{
    pub fn uvex_detector_array(x_gap:f64, y_gap:f64) -> DetectorArray{
        let num_pixels =3*4096;
        let detector_width_degrees = 3.0;
        let center = Point::new(-0.5,0.0,Coordinates::ABSOLUTE);
        let num_detectors_y = 1;
        let num_detectors_x = 1;

        let pixel_to_deg_scale = detector_width_degrees/num_pixels as f64; //Degrees in FOV to pixels
        let detectors_x_axis = (pixel_to_deg_scale,0.0);
        let detectors_y_axis = (0.0,pixel_to_deg_scale);
        /*
        let coordinate_system = CoordinateSystem::new(
            detectors_x_axis,
            detectors_y_axis,
            center.values(),
            "Detector Coordinate System".to_string(),
            "magenta".to_string());

         */

        let coordinate_system = CoordinateSystem{
            x_axis: detectors_x_axis,
            y_axis: detectors_y_axis,
            center: (0.0, 0.0),
            color: "red".to_string(),
            label: "detector grid".to_string(),
        };



        let detector_grid = GRID2D::new_empty(
            (num_detectors_x,num_detectors_x),
            (1.0 + x_gap,1.0 + y_gap),
            (-0.56, -0.06),
            0.001,
            ABSOLUTE);

        let mut detectors = Vec::new();
        for point in 0..detector_grid.num_points{
            let point_location = detector_grid.locate(point);
            // println!("Point location of point {point} is {:?}",point_location);
            let center = Point::new(point_location.x, point_location.y, Coordinates::ABSOLUTE);
            //println!("Center is at {:?}",center);
            detectors.push(Detector::new_uvex(
                point.to_string(),
                center,
                num_pixels,
                RELATIVE(coordinate_system.clone())))

        }

        DetectorArray{
            label: "UVEX Detector Array".to_string(),
            detectors,
            coordinate_system,
        }


    }
}





