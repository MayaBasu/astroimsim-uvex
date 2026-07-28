use astroimsim_data::prelude::FullSpectrumSourceList;
use astroimsim_spectra::power_spectrum::PowerSpectrum;
use astroimsim_spectra::spectral_response::SpectralResponseCurve;
use astroimsim_spectra::visualize::STANDARD_SPECTRAL_GRID;
use crate::benchmarktesting::benchpress;
use crate::hallucinations::{hallucinate_dead_pixel_map, hallucinate_normal_distribution};
use crate::uvex_telescope::{UVEXConfiguration, UVEX};

pub mod uvex_telescope;
mod hallucinations;
mod benchmarktesting;
mod config;
mod detector_effects;
use crate::config::UseEffect;
use crate::config::UseEffect::{On, Off};

fn main() {

    /*


    hallucinate_dead_pixel_map(4096,
                               "/Users/mayabasu/Desktop/uvex_psf_files/dead_pixel.fits",
                               1000,
                               50,
                               400,
                               50);
    hallucinate_normal_distribution(4096,"/Users/mayabasu/Desktop/uvex_psf_files/dar_current.fits", 0.0001,0.001);
    */
    //benchpress()

    //UVEX::generate_random_dead_pixel_map();

    let configuration = UVEXConfiguration::default(Off);
    configuration.to_yaml("config.yaml");
    let configuration = UVEXConfiguration::from_yaml("config.yaml");

    //UVEX::generate_missing_data(configuration)
    let mut uvex = UVEX::initialize(configuration);
    let mut spectrum = PowerSpectrum::flat_AB(20.0,STANDARD_SPECTRAL_GRID,"Input Spectrum");
    uvex.run((10.0,10.0),FullSpectrumSourceList::full_spectrum_point_source_field(0, 100.0, 1000.0, spectrum, &uvex.detector_array.detectors[0].grid));



   // let psf_grid = uvex.fuv_psf;
   // psf_grid.gaussian_blur("/Users/mayabasu/Desktop/blurred_psf",10.0)

    /*

    let fuv_path = "/Users/mayabasu/Desktop/uvex_psf_files/FUV PSF";
    let flatfield = "/Users/mayabasu/Desktop/uvex_psf_files/FUV_flat_field_illumination.fits";
    let mut uvex = uvex_telescope::UVEX::initialize(fuv_path,flatfield);
d    uvex.compare_flatfields(600);

     */






}


