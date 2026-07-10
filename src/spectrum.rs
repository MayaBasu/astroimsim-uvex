
use  astroimsim_geometry::grid1d::*;

/*
Frequency response files
 */
pub const STANDARD_SPECTRAL_GRID: GRID1D = GRID1D::new_empty(0.5,100.0,1000.0,0.01,1.0);


pub const FUV_CONTAMINATION_GRID: GRID1D = GRID1D::new_empty(1.0,110.0,999.0,0.01,1.0);
pub const NUV_CONTAMINATION_GRID: GRID1D = GRID1D::new_empty(1.0,110.0,999.0,0.01,1.0);
pub const FUV_RESPONSE_GRID: GRID1D = GRID1D::new_empty(1.0,100.0,1100.0,0.01,1.0);
pub const NUV_RESPONSE_GRID: GRID1D = GRID1D::new_empty(1.0,120.0,1050.0,0.01,1.0);
pub const NUV_QE_CURVE_GRID: GRID1D = GRID1D::new_empty(1.0,100.0,1100.0,0.01,1000.0);
pub const DICHROIC_GRID: GRID1D = GRID1D::new_empty(0.5,120.5,1100.0,0.01,1000.0);



pub const FUV_CONTAMINATION: FrequencyFile = FrequencyFile::new_from_grid(FUV_CONTAMINATION_PATH,FUV_CONTAMINATION_GRID);
pub const NUV_CONTAMINATION: FrequencyFile = FrequencyFile::new_from_grid(NUV_CONTAMINATION_PATH,NUV_CONTAMINATION_GRID);
pub const FUV_RESPONSE: FrequencyFile = FrequencyFile::new_from_grid(FUV_RESPONSE_PATH,FUV_RESPONSE_GRID);
pub const NUV_RESPONSE: FrequencyFile = FrequencyFile::new_from_grid(NUV_RESPONSE_PATH,NUV_RESPONSE_GRID);
pub const NUV_QE: FrequencyFile = FrequencyFile::new_from_grid(NUV_QE_CURVE_PATH,NUV_QE_CURVE_GRID);
pub const DICHROIC: FrequencyFile = FrequencyFile::new_from_grid(DICHROIC_PATH,DICHROIC_GRID);

pub fn load(){

    let mut plot = Plot::new();
    let mut file =DICHROIC.load_data(false);
    println!("{:?}",file);
    let curve1 = file.plot();
    println!("got first plot");
    let new_grid = STANDARD_SPECTRAL_GRID;
    let file = file.re_grid(new_grid);
    println!("{:?}",file);
    let curve2 = file.plot();

    println!("PLOTTING {:?}",curve2[0]);
    plotting::run(vec![curve1[0].clone(),curve2[0].clone()]);

    /*
    for curve in curve2{
        println!("adding curve");
        plot.add(&curve);
    }
    for curve in curve1{
        plot.add(&curve);

    }

     */

    // plot.show("slefje").unwrap();

}

