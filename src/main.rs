use astroimsim_data::prelude::FullSpectrumSourceList;
use astroimsim_geometry::coordinate_system::{CoordinateSystem, Coordinates};
use astroimsim_geometry::grid2d::GRID2D;
use astroimsim_spectra::power_spectrum::PowerSpectrum;
use astroimsim_spectra::spectral_response::SpectralResponseCurve;
use astroimsim_spectra::visualize::STANDARD_SPECTRAL_GRID;
use clap::Parser;
use plotpy::Plot;
use crate::benchmarktesting::benchpress;
use crate::hallucinations::{hallucinate_custom_distribution, hallucinate_dead_pixel_map, hallucinate_normal_distribution};
use crate::uvex_telescope::{DetectorSetup, UVEXConfiguration, UVEX};

pub mod uvex_telescope;
pub mod hallucinations;
pub mod benchmarktesting;
pub mod config;
pub mod detector_effects;
pub mod notebook;
pub mod parser;
pub mod star_reader;

use crate::config::UseEffect;
use crate::config::UseEffect::{On, Off};
use crate::notebook::generate_test_notebook;
use clap::{Arg, ArgAction, Command};

/*
fn main() {
    let cmd=Command::new("pacman")
        .about("package manager utility")
        .version("5.2.1")
        .subcommand_required(true)
        .arg_required_else_help(true)
        // Query subcommand
        //
        // Only a few of its arguments are implemented below.
        .subcommand(
            Command::new("query")
                .short_flag('Q')
                .long_flag("query")
                .about("Query the package database.")
                .arg(
                    Arg::new("search")
                        .short('s')
                        .long("search")
                        .help("search locally installed packages for matching strings")
                        .conflicts_with("info")
                        .action(ArgAction::Set)
                        .num_args(1..),
                ));
    //huh

    let matches = cmd.get_matches();
    let matches = match matches.subcommand() {
        Some(("example", matches)) => matches,
        _ => unreachable!("clap should ensure we don't get here"),
    };
    //let manifest_path = matches.get_one::<std::path::PathBuf>("manifest-path");
    println!("{matches:?}");
}

 */

// See also `clap_cargo::style::CLAP_STYLING`



fn main() {
    //star_reader::read_stars();


    /*
    //let config = UVEXConfiguration::default(UseEffect::On);
    //config.to_yaml("configg".to_string());
    let config = UVEXConfiguration::from_yaml("configg".to_string());
    let mut uvex = UVEX::initialize(config);
    let spectral_grid = astroimsim_spectra::visualize::STANDARD_SPECTRAL_GRID;
    let spectrum = PowerSpectrum::flat_AB(20.0,spectral_grid,"Input Spectrum");
    let sources = astroimsim_data::
    point_sources::
    FullSpectrumSourceList::
    full_spectrum_point_source_field(100, 0.01, 1.0, spectrum, &uvex.fuv_flatfield.grid);
    uvex.run((0.4,0.3), sources, 300.0);

     */








    //generate_test_notebook();




    /*

    hallucinate_dead_pixel_map(4096,
                               "/Users/mayabasu/Desktop/uvex_psf_files/dead_pixel.fits",
                               1000,
                               50,
                               400,
                               50);

     */
    //hallucinate_normal_distribution(4096,"/Users/mayabasu/Desktop/uvex_psf_files/dar_current.fits", 0.0001,0.001);

    //benchpress()

    //UVEX::generate_random_dead_pixel_map();


   // let configuration = UVEXConfiguration::default(On);
   // configuration.to_yaml("config.yaml".to_string());


    let configuration = UVEXConfiguration::from_yaml("config.yaml".to_string());

    //UVEX::generate_missing_data(configuration)
   let mut uvex = UVEX::initialize(configuration, DetectorSetup::Single);

    let spectrum = PowerSpectrum::flat_AB(20.0,STANDARD_SPECTRAL_GRID,"Input Spectrum");
    let sources = astroimsim_data::
    point_sources::
    FullSpectrumSourceList::
    full_spectrum_point_source_field(10000, 100.0, 1000.0, spectrum, &uvex.fuv_flatfield.grid);
    uvex.run((0.01,0.02), sources, (300.0,900.0),"/Users/mayabasu/Desktop/test_outputs".to_string());





   // let mut plot = Plot::new();
  //  grid.plot_outline(&mut plot,"green");
   // uvex.fuv_flatfield.grid.plot_outline(&mut plot, "red");
  //  println!("{:?}",uvex.fuv_flatfield.grid.center);
    /*
    let colors = ["red","yellow", "blue",
        "purple", "orange", "black",
        "blue", "pink", "gray"];

     */

    //for i in 0..9{
    //    uvex.detector_array.detectors[i].grid.plot_outline(&mut plot, colors[i]);
    //}



   // plot.show("lskejf");


   // println!("FINISHED INITIALIZING");
    let mut spectrum = PowerSpectrum::flat_AB(20.0,STANDARD_SPECTRAL_GRID,"Input Spectrum");
  //  uvex.run((10.0,10.0),FullSpectrumSourceList::full_spectrum_point_source_field(1000, 100.0, 1000.0, spectrum, &uvex.fuv_flatfield.grid));


    let a = STANDARD_SPECTRAL_GRID;


   // let psf_grid = uvex.fuv_psf;
   // psf_grid.gaussian_blur("/Users/mayabasu/Desktop/blurred_psf",10.0)

    /*

    let fuv_path = "/Users/mayabasu/Desktop/uvex_psf_files/FUV PSF";
    let flatfield = "/Users/mayabasu/Desktop/uvex_psf_files/FUV_flat_field_illumination.fits";
    let mut uvex = uvex_telescope::UVEX::initialize(fuv_path,flatfield);
d    uvex.compare_flatfields(600);

     */






}




