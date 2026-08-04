use std::io::Write;
use std::fs;
use rayon::iter::*;
use std::time::Instant;
use plotpy::Plot;
use rand::distr::Distribution;
use rand_distr::Poisson;
use astroimsim_geometry::prelude::*;
use astroimsim_spectra::spectral_response::SpectralResponseCurve;
use astroimsim_data::prelude::*;
use astroimsim_geometry::prelude::Coordinates::{ABSOLUTE, RELATIVE};
pub(crate) use crate::config::{UVEXConfiguration, UseEffect};
use crate::detector_effects;
use crate::hallucinations::{hallucinate_dead_pixel_map, hallucinate_normal_distribution};
/*
Run time inputs:
fuv/nuv exposure time
sky background
*/

pub struct SpatialEffectArray{
    pub label: String,
    pub effects: Vec<(usize, SpatialEffect)>
}



pub struct UVEX{
    pub config: UVEXConfiguration,

    pub fuv_spectral_response: SpectralResponseCurve,
    pub nuv_spectral_response: SpectralResponseCurve,

    pub fuv_flatfield: SpatialEffect,
    pub nuv_flatfield: SpatialEffect,

    pub fuv_psf: PsfGrid,
    pub nuv_psf: PsfGrid,

    pub blurred_fuv_psf: PsfGrid,
    pub blurred_nuv_psf: PsfGrid,

    pub dead_pixel_maps: SpatialEffectArray,
    pub read_noise_maps: SpatialEffectArray,
    pub dark_current_maps: SpatialEffectArray,

    pub detector_array: DetectorArray,

}

impl UVEX{
    pub fn initialize(
        config:UVEXConfiguration
    )->UVEX {
        let num_pixels = 4096;  //pixels per detector

        let detector_width = 1.0; //detector width in degrees
        let x_detectors = 3; //number of detectors in x directions
        let y_detectors = 3;
        let detector_plane_center = Point::new(-0.56, -0.06,ABSOLUTE);
        let x_gap = config.x_gap; //gap size in degrees TODO
        let y_gap = config.y_gap;
        let center_keys = ("XFLD".to_string(), "YFLD".to_string()); //keys for FITS header of PSF files

        //Load in FUV and NUV flat field illumination
        let mut fuv_flatfield = SpatialEffect::new_empty(
            "Flatfield 4k Illumination for the FUV path".to_string(),
            UVEX::flatfield_grid(),
            config.fuv_flatfield_illumination.1.clone());
        fuv_flatfield.load_data(8+30);

        let mut nuv_flatfield = SpatialEffect::new_empty(
            "Flatfield 4k Illumination for the NUV path".to_string(),
            UVEX::flatfield_grid(),
            config.nuv_flatfield_illumination.1.clone());
        nuv_flatfield.load_data(8+30);


        //load in psf files
        println!("loading fuv psf files");
        let mut fuv_psf = PsfGrid::new(
            "FUV PSF grid".to_string(),
            UVEX::empty_fuv(),
            config.fuv_psf.1.clone(),
            center_keys.clone());
        fuv_psf.load_data_frames(64,64);

        let mut nuv_psf = PsfGrid::new(
            "NUV PSF grid".to_string(),
            UVEX::empty_fuv(),
            config.nuv_psf.1.clone(),
            center_keys.clone());
        nuv_psf.load_data_frames(64,64);


        //Load spectral response data
        let (fuv_spectral_response,nuv_spectral_response) = UVEX::fuv_nuv(config.clone());
        //Compose the detector effects


        //make detector grid
        let (detector_array, detector_grid)= UVEX::uvex_detector_array(
            num_pixels,
            x_detectors,
            y_detectors,
            x_gap,
            y_gap,
            detector_width,
            detector_plane_center);

        let dead_pixels = detector_effects::dead_pixels(&detector_array);
        let read_noise = detector_effects::read_noise(&detector_array);
        let dark_current = detector_effects::load_detector_effects(&detector_array,
        "fuv_dark_current", &*config.fuv_dark_current.1);


        match config.gaussian_blur_std.0 {
            UseEffect::On => {
                println!("rying to load from {:?}",config.fuv_psf.2.clone());
                //fuv_psf.gaussian_blur(config.fuv_psf.2.clone(), config.gaussian_blur_std.1);
                //nuv_psf.gaussian_blur(config.nuv_psf.2.clone(), config.gaussian_blur_std.1);
            }
            UseEffect::Off => {println!("Gaussian blur is turned off")}
        }



        let mut blurred_fuv_psf = PsfGrid::new(
            "blurred FUV PSF grid".to_string(),
            UVEX::empty_fuv(),
            config.fuv_psf.2.clone(),
            center_keys.clone());
        blurred_fuv_psf.load_data_frames(64,64);

        let mut blurred_nuv_psf = PsfGrid::new(
            "blurredNUV PSF grid".to_string(),
            UVEX::empty_fuv(),
            config.nuv_psf.2.clone(),
            center_keys.clone());
        blurred_nuv_psf.load_data_frames(64,64);









        let uvex = UVEX{
            config:config.clone(),
            fuv_spectral_response,
            nuv_spectral_response,
            fuv_flatfield,
            nuv_flatfield,
            fuv_psf,
            nuv_psf,

            blurred_fuv_psf,
            blurred_nuv_psf,

            detector_array,
            dead_pixel_maps: dead_pixels,
            read_noise_maps: read_noise,
            dark_current_maps: dark_current,
        };





        uvex

    }

    pub fn area()->f64 {4410.0}

    pub fn flatfield_grid() -> GRID2D{

        let num_pixels =4096-16-60;
        let flatfield_width_degrees = (4.52);
        let center_absolute = Point::new(-0.5,0.0,Coordinates::ABSOLUTE);

        let pixel_to_deg_scale = flatfield_width_degrees/num_pixels as f64; //Degrees in FOV to pixels


        let flatfield_x_axis = (pixel_to_deg_scale,0.0);
        let flatfield_y_axis = (0.0,pixel_to_deg_scale);

        let coordinate_system = CoordinateSystem::new(
            flatfield_x_axis,
            flatfield_y_axis,
            (0.0,0.0),
            "Detector Coordinate System",
            "magenta");
        GRID2D::new_empty((num_pixels,num_pixels),(1.0,1.0),Point::new(-0.56,0.06,ABSOLUTE).convert(&RELATIVE(coordinate_system.clone())).values(),0.0000001,Coordinates::RELATIVE(coordinate_system))
        /*
        GRID2D::new_from_width(
            (num_pixels,num_pixels),(num_pixels as f64,num_pixels as f64), (0.0,0.0),0.0000001,Coordinates::RELATIVE(coordinate_system))

         */


    }

    pub fn spectral_grid()->GRID1D{

        GRID1D::new_empty(1.0,120.0,1000.0,0.01,1.0)

    }
    pub fn generate_missing_data(config: UVEXConfiguration){

        let read_noise_mean = config.read_noise.1;
        let read_noise_std = config.read_noise.2;

        let dark_current_mean = config.dark_current.1;
        let dark_current_std = config.dark_current.2;

        let num_scattered_pixels=config.dead_pixels.1;
        let num_rows= config.dead_pixels.2;
        let num_columns= config.dead_pixels.3;
        let num_rectangles=config.dead_pixels.4;

        for i in 0..9{
            let fuv_dead_pixel_path = format!("{}/fuv_dead_pixels_{i}.fits",config.fuv_dead_pixels.1);
            let fuv_dark_current_path = format!("{}/fuv_dark_current_{i}.fits",config.fuv_dark_current.1);
            let fuv_read_noise_path = format!("{}/fuv_read_noise_{i}.fits",config.fuv_read_noise.1);

            let nuv_dead_pixel_path = format!("{}/nuv_dead_pixels_{i}.fits",config.nuv_dead_pixels.1);
            let nuv_dark_current_path = format!("{}/nuv_dark_current_{i}.fits",config.nuv_dark_current.1);
            let nuv_read_noise_path = format!("{}/nuv_read_noise_{i}.fits",config.nuv_read_noise.1);
            match config.dark_current.0{
                UseEffect::On => {
                    hallucinate_normal_distribution(4096, &*fuv_dark_current_path,dark_current_mean,dark_current_std);
                    hallucinate_normal_distribution(4096, &*nuv_dark_current_path,dark_current_mean,dark_current_std);
                }
                UseEffect::Off => {println!("skipped dark current")}
            }
            match config.read_noise.0{
                UseEffect::On => {
                    hallucinate_normal_distribution(4096, &*fuv_read_noise_path,read_noise_mean,read_noise_std);
                    hallucinate_normal_distribution(4096, &*nuv_read_noise_path,read_noise_mean,read_noise_std);
                }
                UseEffect::Off => {println!("read noise")}
            }
            match config.dead_pixels.0{
                UseEffect::On => {
                    hallucinate_dead_pixel_map(
                        4096,
                        &*fuv_dead_pixel_path,
                        num_scattered_pixels,
                        num_rows,
                        num_columns,
                        num_rectangles);
                    hallucinate_dead_pixel_map(
                        4096,
                        &*nuv_dead_pixel_path,
                        num_scattered_pixels,
                        num_rows,
                        num_columns,
                        num_rectangles);
                }
                UseEffect::Off => {}
            }
        }

    }

    pub fn empty_fuv() -> GRID2D {

        let coord = CoordinateSystem{
            x_axis: (1.0,0.0),
            y_axis: (0.0,1.0),
            center: (0.0, 0.0),
            color: "red",
            label: "fuv",
        };
        let mut grid = GRID2D::new_empty(
            (18,18), //x_num
            (0.2,0.2), //x_step_size
            (-0.56, -0.06), //y_num
            0.1, //y_step_size
            Coordinates::RELATIVE(coord)
        );
        grid.label = "fuv".to_string();
        grid
    }

    pub fn uvex_detector_array(
        num_pixels: usize,
        x_num:usize,
        y_num:usize,
        x_gap_deg:f64,
        y_gap_deg:f64,
        detector_width_deg:f64,
        detector_grid_center:Point) -> (DetectorArray,GRID2D){

        let pixel_to_deg_scale = detector_width_deg/num_pixels as f64; //Degrees in FOV to pixels
        let detectors_x_axis = (pixel_to_deg_scale,0.0);
        let detectors_y_axis = (0.0,pixel_to_deg_scale);

        let coordinate_system = CoordinateSystem{
            x_axis: detectors_x_axis,
            y_axis: detectors_y_axis,
            center: (0.0, 0.0),
            color: "red",
            label: "detector grid",
        };
        let detector_grid = GRID2D::new_empty(
            (x_num,y_num),
            (detector_width_deg + x_gap_deg,detector_width_deg + y_gap_deg),
            detector_grid_center.to_absolute().values(),//(-0.56, -0.06),
            0.001,
            ABSOLUTE);

        let mut detectors = Vec::new();
        for point in 0..detector_grid.num_points{
            let point_location = detector_grid.locate(point);
            // println!("Point location of point {point} is {:?}",point_location);
            let center = Point::new(point_location.x, point_location.y, Coordinates::ABSOLUTE);
            //println!("Center is at {:?}",center);
            detectors.push(UVEX::new_uvex_detector(
                point.to_string(),
                center,
                num_pixels,
                RELATIVE(coordinate_system.clone())))

        }

        (DetectorArray{
            label: "UVEX Detector Array".to_string(),
            detectors,
            coordinate_system,
        }, detector_grid)
    }

    pub fn new_uvex_detector(label: String, center:Point,num_pixels:usize,coordinates: Coordinates) -> Detector {

        let grid = GRID2D::new_empty((num_pixels, num_pixels), (1.0, 1.0), center.convert(&coordinates).values(), 0.001, coordinates);
        let mut data = Vec::with_capacity(num_pixels*num_pixels);
        for _row in 0..num_pixels{
            let mut row_vec = Vec::with_capacity(num_pixels);
            for _column in 0..num_pixels{
                row_vec.push([0.0;2]) //TODO initialize straight with background or no data for efficiency?
            }
            data.push(row_vec);
        }
        Detector {label, grid, data}
    }


        //TODO fix paths
    pub fn fuv_nuv(config:UVEXConfiguration)->(SpectralResponseCurve,SpectralResponseCurve){

        pub const FUV_CONTAMINATION_GRID: GRID1D = GRID1D::new_empty(1.0,110.0,999.0,0.01,1.0);
        pub const NUV_CONTAMINATION_GRID: GRID1D = GRID1D::new_empty(1.0,110.0,999.0,0.01,1.0);
        pub const FUV_RESPONSE_GRID: GRID1D = GRID1D::new_empty(1.0,100.0,1100.0,0.01,1.0);
        pub const NUV_RESPONSE_GRID: GRID1D = GRID1D::new_empty(1.0,120.0,1050.0,0.01,1.0);
        pub const NUV_QE_CURVE_GRID: GRID1D = GRID1D::new_empty(1.0,100.0,1100.0,0.01,1000.0);
        pub const DICHROIC_GRID: GRID1D = GRID1D::new_empty(1.0,120.0,1000.0,0.01,1000.0);
        pub const MIRROR_GRID:GRID1D = GRID1D::new_empty(1.0,110.0,1100.0,0.01,1.0);

        //sd   let FUV_CONTAMINATION: SpectralResponseCurve = SpectralResponseCurve::new("FUV_CONTAMINATION",FUV_CONTAMINATION_GRID,FUV_CONTAMINATION_PATH);
        // let NUV_CONTAMINATION: SpectralResponseCurve = SpectralResponseCurve::new("NUV_CONTAMINATION",NUV_CONTAMINATION_GRID,NUV_CONTAMINATION_PATH);
        let mut FUV_DICHROIC: SpectralResponseCurve = SpectralResponseCurve::new("FUV_DICHROIC".to_string(), DICHROIC_GRID, config.dichroic.1.clone(), 1, "      ");
        let mut NUV_DICHROIC: SpectralResponseCurve = SpectralResponseCurve::new("NUV_DICHROIC".to_string(),DICHROIC_GRID,config.dichroic.1.clone(),2,"      ");
        let mut  NUV_QE: SpectralResponseCurve = SpectralResponseCurve::new("NUV_QE".to_string(),NUV_QE_CURVE_GRID,config.nuv_qe_response.1.clone(),1,"   ");
        let mut FUV_CONTAMINATION:SpectralResponseCurve = SpectralResponseCurve::new("FUV Contamination".to_string(),FUV_CONTAMINATION_GRID,config.fuv_contamination.1.clone(),1,"   ");
        //FUV_CONTAMINATION.write_to_dat("contamination","FUV contamination");
        let mut NUV_FILTER_CURVE:SpectralResponseCurve = SpectralResponseCurve::new("NUV Filter response".to_string(),NUV_RESPONSE_GRID,config.nuv_response.1.clone(),1,"   ");
        let mut FUV_FILTER_CURVE:SpectralResponseCurve = SpectralResponseCurve::new("FUV Filter response".to_string(),FUV_RESPONSE_GRID,config.fuv_response.1.clone(),1,"   ");

        let mut MIRROR_CURVE:SpectralResponseCurve = SpectralResponseCurve::new("Mirror".to_string(), MIRROR_GRID, config.mirror_response.1.clone(),1,"    ");
        let mut MIRROR_CURVE_3 = MIRROR_CURVE.clone();
        MIRROR_CURVE_3.self_compose(3);


        let fuv = SpectralResponseCurve::compose(vec![
            MIRROR_CURVE_3.clone(),
            FUV_CONTAMINATION.clone(),
            FUV_DICHROIC,
            FUV_FILTER_CURVE,

        ]);
        let nuv = SpectralResponseCurve::compose(vec![
            MIRROR_CURVE_3,
            FUV_CONTAMINATION,
            NUV_DICHROIC,
            NUV_FILTER_CURVE,
            NUV_QE
        ]);
        (fuv,nuv)
    }


    pub fn run(&mut self, background:(f64,f64), mut source_list: FullSpectrumSourceList) {
        println!("WARNING RUNNING WITH ARTIFICIAL BRIGHTNESS");

        let start = Instant::now();
        for i in 0..9{
            let detector_grid = self.detector_array.detectors[i].grid.clone();

            self.detector_array.detectors[i].create_constant_background(10.0, 1.0);


            match self.config.read_noise.0 {
                UseEffect::On => {
                    self.detector_array.detectors[i].add_effect(self.read_noise_maps.effects[i].1.clone(), 0);
                }
                UseEffect::Off => { println!("Read Noise is Turnned Off") }
            }
            match self.config.dark_current.0 {
                UseEffect::On => {
                    self.detector_array.detectors[i].add_effect(self.dark_current_maps.effects[i].1.clone(), 0);
                }
                UseEffect::Off => { println!("dark current is turned off") }
            }


            //self.detector_array.detectors[0].write(13);


            let mut dropped = 0;
            let detector_grid = self.detector_array.detectors[i].grid.clone();
            for (num, point) in source_list.sources.iter().enumerate() {
                if num % 100 == 0 {
                    println!("Done {:?} sources for detector number {:?}", num, i)
                }
                let bands = point.to_bands(&self.fuv_spectral_response, &self.nuv_spectral_response, UVEX::area());

                let mut rng = rand::rng();

                match detector_grid.inside_or_outside(&point.point) { //TODO remove many unneeded clone() calls by borrowing Points
                    Location::Outside => { dropped += 1 }
                    Location::Inside => {
                        let psf = self.blurred_fuv_psf.interpolated_psf(&point.point);
                        let ((x_mod, y_mod), binned_psf) = detector_grid.bin_up_patch(point.point.clone(), &psf, 10); //TODO scale is fixed
                        //println!("{:?}",(x_mod,y_mod));
                        let binned_matrix_x = binned_psf[0].len();
                        let binned_matrix_y = binned_psf.len();

                        for row in 0..binned_matrix_y {
                            for column in 0..binned_matrix_x {


                                //println!("{}{}",column + y, row + y);
                                if column + y_mod < detector_grid.x_num && row + x_mod < detector_grid.y_num {
                                    let fuv_flux = binned_psf[column][row] as f64 * bands.fuv * point.scale;
                                    let nuv_flux = binned_psf[column][row] as f64 * bands.nuv * point.scale;
                                    if (fuv_flux == 0.0) && (nuv_flux == 0.0) {
                                        continue
                                    } else {
                                        // println!("flux is {:?}", flux);
                                        //  println!("fuv flux {:?}",fuv_flux);
                                        //    let fuv_poisson = Poisson::new(fuv_flux as f64).unwrap();
                                        //   let nuv_poisson = Poisson::new(nuv_flux as f64).unwrap();
                                        //   let fuv = fuv_poisson.sample(&mut rng) as f64;
                                        //  let nuv = nuv_poisson.sample(&mut rng) as f64;
                                        self.detector_array.detectors[i].data[column + y_mod][row + x_mod][0] += fuv_flux;
                                        self.detector_array.detectors[i].data[column + y_mod][row + x_mod][1] += nuv_flux;
                                        // self.detector_array.detectors[0].data[column + y_mod][row + x_mod][2] += fuv as f64 ;
                                        // self.detector_array.detectors[0].data[column + y_mod][row + x_mod][3] += nuv as f64 ;
                                        //bin.sample(&mut rng) as f32;
                                    }
                                } else {
                                    // println!("dropping pixel");
                                }

                                // println!("modifying pixel {:?} to be {:?}",(row + x_mod,column + y_mod),binned_psf[column][row]);
                            }
                        }
                    }
                }


                // data[0][0]  += 100.0;

            }

            match self.config.fuv_flatfield_illumination.0 {
                UseEffect::On => {
                    for row in 0..self.detector_array.detectors[i].grid.y_num {
                        for column in 0..self.detector_array.detectors[i].grid.x_num {
                            let grid_number = self.detector_array.detectors[i].grid.grid_number(column, row);
                            let location = self.detector_array.detectors[i].grid.locate(grid_number);
                            let fuv_vinietting = self.fuv_flatfield.get_data(&location);
                            //   println!("{fuv_vinietting} {grid_number} {:?}",location.to_absolute());
                            //  println!("{:?}", self.fuv_flatfield.grid.locate(0).to_absolute());
                            //  println!("{:?}", self.detector_array.detectors[i].grid.locate(0).to_absolute());

                            let nuv_vinietting = self.nuv_flatfield.get_data(&location);
                            self.detector_array.detectors[i].data[column][row][0] = self.detector_array.detectors[i].data[column][row][0] * fuv_vinietting;
                            self.detector_array.detectors[i].data[column][row][1] = self.detector_array.detectors[i].data[column][row][1] * nuv_vinietting;
                        }
                    }
                } //TODO remove this clone
                UseEffect::Off => { println!("vinietting is off") }
            }

            match self.config.dead_pixels.0 {
                UseEffect::On => {
                    self.detector_array.detectors[i].multiply_effect(self.dead_pixel_maps.effects[i].1.clone(), 0);
                }
                UseEffect::Off => {
                    println!("Dead pixel map is turrned off")
                }
            }

            let size = self.detector_array.detectors[i].data.len();
            let size2 = self.detector_array.detectors[i].data[0].len();

            let sum: f64 = self.detector_array.detectors[i].data.iter().flatten().flatten().sum();
            println!("Done computation: {:?} for detecgtor {:?}", sum, i);
            self.detector_array.detectors[i].write(i);
            let duration = start.elapsed();
            println!("Time elapsed in expensive_function() is: {:?}, dropped {:?}", duration, dropped);

            println!("made array, sum is  :{}, size is {:?}, {:?}", sum, size, size2);



        }


    /*


        self.detector_array.detectors.into_par_iter().enumerate().for_each(|(i,mut detector)|
            { let mut dropped = 0;
                let detector_grid = detector.grid.clone();
                for (num, point) in source_list.sources.iter().enumerate(){

                    if num % 10000 ==0{
                        println!("Done {:?} sources for detector number {:?}", num,i)
                    }
                    let bands = point.to_bands(&self.fuv_spectral_response,&self.nuv_spectral_response, UVEX::area());

                     let mut rng = rand::rng();

                    match detector_grid.inside_or_outside(&point.point){ //TODO remove many unneeded clone() calls by borrowing Points
                        Location::Outside => {dropped +=1}
                        Location::Inside => {let psf = self.fuv_psf.interpolated_psf(&point.point);
                            let ((x_mod,y_mod),binned_psf) = detector_grid.bin_up_patch(point.point.clone(),&psf,10); //TODO scale is fixed
                            //println!("{:?}",(x_mod,y_mod));
                            let binned_matrix_x = binned_psf[0].len();
                            let binned_matrix_y = binned_psf.len();

                            for row in 0..binned_matrix_y{
                                for column in 0..binned_matrix_x{


                                    //println!("{}{}",column + y, row + y);
                                    if column + y_mod < detector_grid.x_num && row + x_mod < detector_grid.y_num{

                                        let fuv_flux =binned_psf[column][row] as f64*bands.fuv;
                                        let nuv_flux =binned_psf[column][row] as f64*bands.nuv;
                                        if (fuv_flux == 0.0) &&(nuv_flux ==0.0){
                                            continue
                                        }else{
                                            // println!("flux is {:?}", flux);
                                           // println!("{:?}",fuv_flux);
                                             let fuv_poisson = Poisson::new(fuv_flux as f64).unwrap();
                                            let nuv_poisson = Poisson::new(nuv_flux as f64).unwrap();
                                            let fuv = fuv_poisson.sample(&mut rng) as f64;
                                            let nuv = nuv_poisson.sample(&mut rng) as f64;
                                            detector.data[column + y_mod][row + x_mod][0] += fuv_flux;
                                            detector.data[column + y_mod][row + x_mod][1] += nuv_flux;
                                            detector.data[column + y_mod][row + x_mod][2] += fuv as f64 ;
                                            detector.data[column + y_mod][row + x_mod][3] += nuv as f64 ;
                                            //bin.sample(&mut rng) as f32;
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

                let size = detector.data.len();
                let size2 = detector.data[0].len();

                let sum:f64  = detector.data.iter().flatten().flatten().sum();
                println!("Done computation: {:?} for detecgtor {:?}",sum,i);
                detector.write(i);
                let duration = start.elapsed();
                println!("Time elapsed in expensive_function() is: {:?}, dropped {:?}", duration,dropped);

                println!("made array, sum is  :{}, size is {:?}, {:?}",sum, size,size2);


            });

         */










    }


}