use crate::benchmarktesting::benchpress;
use crate::hallucinations::{ hallucinate_normal_distribution};
use crate::uvex_telescope::UVEXConfiguration;

pub mod uvex_telescope;
mod hallucinations;
mod benchmarktesting;

fn main() {
    hallucinate_normal_distribution(4096,"dark_current.fits", 0.0001,0.001);
    benchpress()

   // let details = UVEXConfiguration::default().to_yaml("config.yaml");

    /*

    let fuv_path = "/Users/mayabasu/Desktop/uvex_psf_files/FUV PSF";
    let flatfield = "/Users/mayabasu/Desktop/uvex_psf_files/FUV_flat_field_illumination.fits";
    let mut uvex = uvex_telescope::UVEX::initialize(fuv_path,flatfield);
    uvex.compare_flatfields(600);

     */






}


