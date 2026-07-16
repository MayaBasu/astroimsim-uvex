use crate::benchmarktesting::benchpress;
use crate::hallucinations::{hallucinate_dead_pixel_map, hallucinate_normal_distribution};
use crate::uvex_telescope::{UVEXConfiguration, UVEX};

pub mod uvex_telescope;
mod hallucinations;
mod benchmarktesting;

fn main() {


    hallucinate_dead_pixel_map(4096,
                               "/Users/mayabasu/Desktop/uvex_psf_files/dead_pixel.fits",
                               1000,
                               50,
                               400,
                               50);
    hallucinate_normal_distribution(4096,"/Users/mayabasu/Desktop/uvex_psf_files/dar_current.fits", 0.0001,0.001);
    //benchpress()
    //UVEX::generate_random_dead_pixel_map();

    let details = UVEXConfiguration::default();
    UVEX::initialize(details);

    /*

    let fuv_path = "/Users/mayabasu/Desktop/uvex_psf_files/FUV PSF";
    let flatfield = "/Users/mayabasu/Desktop/uvex_psf_files/FUV_flat_field_illumination.fits";
    let mut uvex = uvex_telescope::UVEX::initialize(fuv_path,flatfield);
    uvex.compare_flatfields(600);

     */






}


