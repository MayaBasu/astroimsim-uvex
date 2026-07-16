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
use serde::Serialize;
use crate::hallucinations::hallucinate_dead_pixel_map;
use crate::uvex_telescope::UseEffect::On;
/*
Run time inputs:
fuv/nuv exposure time
sky background

 */
#[derive(Serialize)]
pub enum UseEffect{
    On,
    Off
}
#[derive(Serialize)]

pub struct UVEXConfiguration {
    pub fuv_contamination: (UseEffect,&'static str),
    pub nuv_contamination: (UseEffect,&'static str),

    pub fuv_response: (UseEffect,&'static str),
    pub nuv_response: (UseEffect,&'static str),

    pub nuv_qe_response: (UseEffect,&'static str), //TO BE REMOVED SOON

    pub dichroic: (UseEffect,&'static str),
    pub mirror_response: (UseEffect,&'static str),

    pub fuv_psf: (UseEffect,&'static str),
    pub nuv_psf: (UseEffect,&'static str),

    pub fuv_flatfield_illumination: (UseEffect,&'static str),
    pub nuv_flatfield_illumination: (UseEffect,&'static str),

    pub fuv_read_noise:  (UseEffect,&'static str),
    pub nuv_read_noise: (UseEffect,&'static str),

    pub fuv_dark_current: (UseEffect,&'static str),
    pub nuv_dark_current: (UseEffect,&'static str),

    pub fuv_dead_pixels: (UseEffect,&'static str),
    pub nuv_dead_pixels: (UseEffect,&'static str),

    pub x_gap: f64,
    pub y_gap: f64,

}


impl UVEXConfiguration{
    pub fn default()-> UVEXConfiguration{
        UVEXConfiguration{
            fuv_contamination: (On, "/Users/mayabasu/Desktop/uvex_psf_files/spectral_reponse_files/UVIM_FUV_contamination.dat"),
            nuv_contamination: (On,"/Users/mayabasu/Desktop/uvex_psf_files/spectral_reponse_files/UVIM_NUV_contamination.dat"),

            fuv_response: (On,"/Users/mayabasu/Desktop/uvex_psf_files/spectral_reponse_files/UVIM_FUV_filter_response.dat"),
            nuv_response: (On,"/Users/mayabasu/Desktop/uvex_psf_files/spectral_reponse_files/UVIM_NUV_filter_response.dat"),

            nuv_qe_response: (On,"/Users/mayabasu/Desktop/uvex_psf_files/spectral_reponse_files/UVIM_NUV_QE.dat"),

            dichroic: (On,"/Users/mayabasu/Desktop/uvex_psf_files/spectral_reponse_files/UVIM_dichroic_response.dat"),
            mirror_response: (On,"/Users/mayabasu/Desktop/uvex_psf_files/spectral_reponse_files/mirror_reflectivity.dat"),

            fuv_psf: (On,"/Users/mayabasu/Desktop/uvex_psf_files/FUV PSF"),
            nuv_psf: (On,"/Users/mayabasu/Desktop/uvex_psf_files/NUV PSF"),

            fuv_flatfield_illumination: (On,"/Users/mayabasu/Desktop/uvex_psf_files/FUV_vignetting_model_4096.fits"),
            nuv_flatfield_illumination: (On,"/Users/mayabasu/Desktop/uvex_psf_files/NUV_vignetting_model_4096.fits"),

            fuv_read_noise:  (On,"TODO"),
            nuv_read_noise: (On,"TODO"),

            fuv_dark_current: (On,"TODO"),
            nuv_dark_current: (On,"TODO"),

            fuv_dead_pixels:(On, "TODO"),
            nuv_dead_pixels: (On,"TODO"),

            x_gap: 0.0,
            y_gap: 0.0,
        }
    }

    pub fn to_yaml(&self, path:&'static str){
        println!("Writing configuration to {:?}", path);
        let serialized_self = serde_yaml::to_string(&self).expect("Failed to YAMLify the object");
        let mut file = fs::File::create(path).expect("Couldn't create the config file");
        write!(file, "{}", serialized_self).expect("Failed to write YAML to config file");

    }

}

pub struct UVEX{
    pub config: UVEXConfiguration,

    pub fuv_spectral_response: SpectralResponseCurve,
    pub nuv_spectral_response: SpectralResponseCurve,

    pub fuv_flatfield: SpatialEffect,
    pub nuv_flatfield: SpatialEffect,

    pub fuv_psf: PsfGrid,
    pub nuv_psf: PsfGrid,

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
        let center_keys = ("XFLD", "YFLD"); //keys for FITS header of PSF files

        //Load in FUV and NUV flat field illumination
        let mut fuv_flatfield = SpatialEffect::new_empty(
            "Flatfield 4k Illumination for the FUV path",
            UVEX::flatfield_grid(),
            config.fuv_flatfield_illumination.1);
        fuv_flatfield.load_data(8+30);

        let mut nuv_flatfield = SpatialEffect::new_empty(
            "Flatfield 4k Illumination for the NUV path",
            UVEX::flatfield_grid(),
            config.nuv_flatfield_illumination.1);
        nuv_flatfield.load_data(8+30);



        //load in psf files
        let mut fuv_psf = PsfGrid::new(
            "FUV PSF grid",
            UVEX::empty_fuv(),
            config.fuv_psf.1,
            center_keys);
        fuv_psf.load_data_frames(64,64);

        let mut nuv_psf = PsfGrid::new(
            "NUV PSF grid",
            UVEX::empty_fuv(),
            config.nuv_psf.1,
            center_keys);
        nuv_psf.load_data_frames(64,64);


        //Load spectral response data
        let (fuv_spectral_response,nuv_spectral_response) = UVEX::fuv_nuv();

        //make detector grid
        let (detector_array, detector_grid)= UVEX::uvex_detector_array(
            num_pixels,
            x_detectors,
            y_detectors,
            x_gap,
            y_gap,
            detector_width,
            detector_plane_center);


        UVEX{
            config,
            fuv_spectral_response,
            nuv_spectral_response,
            fuv_flatfield,
            nuv_flatfield,
            fuv_psf,
            nuv_psf,
            detector_array,
        }




    }

    pub fn area()->f64{
        4410.0
    }

    pub fn spectral_grid()->GRID1D{
        GRID1D::new_empty(1.0,120.0,1000.0,0.01,1.0)
    }

    pub fn generate_random_dead_pixel_map(){
        hallucinate_dead_pixel_map(4096,
                                   "/Users/mayabasu/Desktop/uvex_psf_files/dead_pixel.fits",1000,0,0,0);
    }



    pub fn dead_pixel_map(&self, detector:usize)-> SpatialEffect{
        let path = "/Users/mayabasu/Desktop/uvex_psf_files/dead_pixel.fits";
        let grid = self.detector_array.detectors[detector].grid.clone();
        let dead_pixels = SpatialEffect::new_empty(
            "Dead Pixel map for detector",
            grid,
            path);
        dead_pixels
    }


    pub fn darkcurrent_map(&self, detector:usize)-> SpatialEffect{
        let path = "/Users/mayabasu/Desktop/uvex_psf_files/dar_current.fits";
        let grid = self.detector_array.detectors[detector].grid.clone();
        let dead_pixels = SpatialEffect::new_empty(
            "Dark current map for detector",
            grid,
            path);
        dead_pixels
    }


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
            center_absolute.values(),
            "Detector Coordinate System",
            "magenta");

        GRID2D::new_from_width(
            (num_pixels,num_pixels),(1.0,1.0), (0.0,0.0),0.0000001,Coordinates::RELATIVE(coordinate_system))


    }




    pub fn changable_grid(num_points:usize) -> GRID2D{

        let num_pixels =num_points-16;
        let flatfield_width_degrees = 5.0;
        let center_absolute = Point::new(-0.5,0.0,Coordinates::ABSOLUTE);

        //let pixel_to_deg_scale = flatfield_width_degrees/num_pixels as f64; //Degrees in FOV to pixels

       // let pixel_to_deg_scale = (4096.0-16.0)/(num_points as f64 -16 as f64) as f64 *0.001;
        let flatfield_x_axis = (1.0,0.0);
        let flatfield_y_axis = (0.0,1.0);

        let coordinate_system = CoordinateSystem::new(
            flatfield_x_axis,
            flatfield_y_axis,
            center_absolute.values(),
            "Detector Coordinate System",
            "magenta");

        GRID2D::new_from_width(
            (num_pixels,num_pixels),(20.0*0.49852905123804847,20.0*0.49852905123804847), (0.0,0.0),0.0000001,Coordinates::RELATIVE(coordinate_system))


    }
    /*

    pub fn compare_flatfields(self,num_points:usize)-> Vec<f64>{
        let flat_two = SpatialEffect::spawn_downsample(&self.flatfield_4k_illumination.clone(), UVEX::changable_grid(num_points));
        let mut warnings = 0;
        let mut max_error = 0.0;
        let mut over_2 = 0;

        let errors = (0..self.flatfield_4k_illumination.grid.num_points).map(|i|{

            let random_point = self.flatfield_4k_illumination.grid.locate(i);//flat_two.grid.random();
            let fine_grain_result = self.flatfield_4k_illumination.get_data(&random_point);
            let corse_grain_result = flat_two.get_data(&random_point);
            let error = (fine_grain_result - corse_grain_result)/ fine_grain_result;

          //  println!("{:?}    {:?}   {:?}",
          //          self.flatfield_4k_illumination.grid.fit_grid(&random_point),
           //          flat_two.grid.fit_grid(&random_point),
            //         error*100.0);
           // println!("{:?}",error);


            if error*100.0>1.0{
                warnings += 1;
                if error*100.0 > max_error{
                    max_error = error*100.0;
                }
                if error*100.0>1.0{
                    over_2 +=1;
                }
                println!("Warning, error of {:?} percent at {:?}, it was {:?} verses {:?}", error*100.0, self.flatfield_4k_illumination.grid.xy_indices(i), fine_grain_result,corse_grain_result);


            }

            error*100.0
        }).collect();


        println!("{:?}",flat_two.grid.locate(0));
        println!("{:?} warnings, max error is  {:?}, {:?} entries over 1 erpcent, fraction is {:?}",
                 warnings,
                 max_error,
                 over_2,
                (over_2 as f64/self.flatfield_4k_illumination.grid.num_points as f64)*100.0);
        errors

    }

     */

    pub fn compare(){
        let coordinates = CoordinateSystem::new((1.0,0.0), (0.0,1.0),(0.0,0.0),"test", "red");

        let grid1 = GRID2D::new_empty((3,3),(1.0,1.0),(0.0,0.0),0.001,RELATIVE(coordinates.clone()));
        let grid2 = GRID2D::new_empty((5,5),(0.5,0.50),(0.0,0.0),0.001,RELATIVE(coordinates));
        let mut plot = Plot::new();
        grid2.plot_outline(&mut plot,"yellow");
        grid1.plot_outline(&mut plot,"red");
        grid1.plot_points(&mut plot,PlotPoint::Random);
        grid2.plot_points(&mut plot,PlotPoint::Random);

        //plot.show("test").unwrap();

        let matrix = vec![
            vec![1.0,1.0,1.0],
            vec![2.0,2.0,2.0],
            vec![3.0,3.0,3.0],
        ];

        let grid1_data = SpatialEffect::from_matrix("grid1",grid1,"N/A",matrix);
        let result = grid1_data.spawn_downsample(grid2);
        println!("{:?}",grid1_data);
        println!("{:?}",result);

        for index in 0..grid1_data.grid.num_points{
            let location = grid1_data.grid.locate(index);
            println!("{:?}, {:?}, {:?}", index, grid1_data.get_data(&location),location)
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
            (1.0 + x_gap_deg,1.0 + y_gap_deg),
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
                stringify!(point),
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

    pub fn new_uvex_detector(label: &'static str, center:Point,num_pixels:usize,coordinates: Coordinates) -> Detector {

        let grid = GRID2D::new_empty((num_pixels, num_pixels), (1.0, 1.0), center.convert(&coordinates).values(), 0.001, coordinates);
        let mut data = Vec::with_capacity(num_pixels*num_pixels);
        for _row in 0..num_pixels{
            let mut row_vec = Vec::with_capacity(num_pixels);
            for _column in 0..num_pixels{
                row_vec.push([0.0;4])
            }
            data.push(row_vec);
        }
        Detector {label, grid, data}
    }



    pub fn fuv_nuv()->(SpectralResponseCurve,SpectralResponseCurve){

        let directory = "/Users/mayabasu/Desktop/uvex_psf_files/spectral_reponse_files";
        const FUV_CONTAMINATION_PATH: &'static str = "/Users/mayabasu/Desktop/uvex_psf_files/spectral_reponse_files/UVIM_FUV_contamination.dat";
        const NUV_CONTAMINATION_PATH: &'static str = "/Users/mayabasu/Desktop/uvex_psf_files/spectral_reponse_files/UVIM_NUV_contamination.dat";
        const FUV_RESPONSE_PATH: &'static str = "/Users/mayabasu/Desktop/uvex_psf_files/spectral_reponse_files/UVIM_FUV_filter_response.dat";
        const NUV_RESPONSE_PATH: &'static str = "/Users/mayabasu/Desktop/uvex_psf_files/spectral_reponse_files/UVIM_NUV_filter_response.dat";
        const NUV_QE_CURVE_PATH: &'static str = "/Users/mayabasu/Desktop/uvex_psf_files/spectral_reponse_files/UVIM_NUV_QE.dat";
        const DICHROIC_PATH: &'static str = "/Users/mayabasu/Desktop/uvex_psf_files/spectral_reponse_files/UVIM_dichroic_response.dat";
        const MIRROR_PATH: &'static str = "/Users/mayabasu/Desktop/uvex_psf_files/spectral_reponse_files/mirror_reflectivity.dat";

        pub const FUV_CONTAMINATION_GRID: GRID1D = GRID1D::new_empty(1.0,110.0,999.0,0.01,1.0);
        pub const NUV_CONTAMINATION_GRID: GRID1D = GRID1D::new_empty(1.0,110.0,999.0,0.01,1.0);
        pub const FUV_RESPONSE_GRID: GRID1D = GRID1D::new_empty(1.0,100.0,1100.0,0.01,1.0);
        pub const NUV_RESPONSE_GRID: GRID1D = GRID1D::new_empty(1.0,120.0,1050.0,0.01,1.0);
        pub const NUV_QE_CURVE_GRID: GRID1D = GRID1D::new_empty(1.0,100.0,1100.0,0.01,1000.0);
        pub const DICHROIC_GRID: GRID1D = GRID1D::new_empty(1.0,120.0,1000.0,0.01,1000.0);
        pub const MIRROR_GRID:GRID1D = GRID1D::new_empty(1.0,110.0,1100.0,0.01,1.0);

        //sd   let FUV_CONTAMINATION: SpectralResponseCurve = SpectralResponseCurve::new("FUV_CONTAMINATION",FUV_CONTAMINATION_GRID,FUV_CONTAMINATION_PATH);
        // let NUV_CONTAMINATION: SpectralResponseCurve = SpectralResponseCurve::new("NUV_CONTAMINATION",NUV_CONTAMINATION_GRID,NUV_CONTAMINATION_PATH);
        let mut FUV_DICHROIC: SpectralResponseCurve = SpectralResponseCurve::new("FUV_DICHROIC",DICHROIC_GRID,DICHROIC_PATH,1,"      ");
        let mut NUV_DICHROIC: SpectralResponseCurve = SpectralResponseCurve::new("NUV_DICHROIC",DICHROIC_GRID,DICHROIC_PATH,2,"      ");
        let mut  NUV_QE: SpectralResponseCurve = SpectralResponseCurve::new("NUV_QE",NUV_QE_CURVE_GRID,NUV_QE_CURVE_PATH,1,"   ");
        let mut FUV_CONTAMINATION:SpectralResponseCurve = SpectralResponseCurve::new("FUV Contamination",FUV_CONTAMINATION_GRID,FUV_CONTAMINATION_PATH,1,"   ");
        //FUV_CONTAMINATION.write_to_dat("contamination","FUV contamination");
        let mut NUV_FILTER_CURVE:SpectralResponseCurve = SpectralResponseCurve::new("NUV Filter response",NUV_RESPONSE_GRID,NUV_RESPONSE_PATH,1,"   ");
        let mut FUV_FILTER_CURVE:SpectralResponseCurve = SpectralResponseCurve::new("FUV Filter response",FUV_RESPONSE_GRID,FUV_RESPONSE_PATH,1,"   ");

        let mut MIRROR_CURVE:SpectralResponseCurve = SpectralResponseCurve::new("Mirror", MIRROR_GRID, MIRROR_PATH,1,"    ");
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


    pub fn run(self, source_list: FullSpectrumSourceList){

        let start = Instant::now();
      //  let detector_grid =self.detector_array.detectors[0].grid.clone();

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










    }


}