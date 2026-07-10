use std::time::Instant;
use rand::rand_core::utils::next_word_via_fill;
use crate::datafile_reader;
use crate::point_sources::{Bands, PointSource, SourceList, Spectrum};


pub fn apply_dichroic(source_list: SourceList) -> SourceList{

    let start = Instant::now();

    /*
    let FUV_contamination  = "/Users/mayabasu/Desktop/uvex_psf_files/UVIM_FUV_contamination.dat";
    let FUV_response = "/Users/mayabasu/Desktop/uvex_psf_files/UVIM_FUV_filter_response.dat";
    let NUV_respnse = "/Users/mayabasu/Desktop/uvex_psf_files/UVIM_NUV_filter_response.dat";
    let NUV_contamination = "/Users/mayabasu/Desktop/uvex_psf_files/UVIM_NUV_contamination.dat";
    let QE_curve = "/Users/mayabasu/Desktop/uvex_psf_files/UVIM_NUV_QE.dat";
    let Dichroic = "/Users/mayabasu/Desktop/uvex_psf_files/UVIM_dichroic_response.dat";


    let FUV_contamination = datafile_reader::read_data_file(FUV_contamination,890,2,1.0,false);
    let FUV_response = datafile_reader::read_data_file(FUV_response,1001,2,1.0,false);
    let NUV_respnse =datafile_reader::read_data_file(NUV_respnse,931,2,1.0,false);
    let NUV_contamination =datafile_reader::read_data_file(NUV_contamination,890,2,1.0,false);
    let QE_curve =datafile_reader::read_data_file(QE_curve,1001,2,1000.0,false);
    let Dichroic = datafile_reader::read_data_file(Dichroic,1960,3,1000.0,true);

     */

    let fake_contamination_curve1 = PointSource::fake_spectrum();
    let fake_contamination_curve2 = PointSource::fake_spectrum();

    let mut algamated_sources = Vec::with_capacity(source_list.sources.len());

    for point_source in source_list.sources{
        let (luminosity, spectrum,u) = match  point_source.spectrum{
            Spectrum::Full(luminosity, spectrum,u) => {(luminosity, spectrum,u)}
            Spectrum::Bands(_,_) => {panic!("This shoudl get the full thing :(")}
        };
        let fuv_spectrum = spectrum.clone();
        let nuv_spectrum = spectrum;

        let fuv_band: f64 = fuv_spectrum.iter().zip(fake_contamination_curve1.iter()).map(|(x, y)| x * y).sum();
        let nuv_band: f64 = nuv_spectrum.iter().zip(fake_contamination_curve2.iter()).map(|(x, y)| x * y).sum();

        let fuv_band = fuv_band*luminosity;
        let nuv_band = nuv_band*luminosity;

        // println!("Initial luminosity of {:?} has {:?} in the fuv band and {:?} in the nuv band",luminosity,fuv_band,nuv_band);

        let new_source = PointSource::new(
            point_source.point.clone(),
            Spectrum::Bands(vec![Bands::NUV(nuv_band),Bands::FUV(fuv_band)],u));
        algamated_sources.push(new_source);

    }
    println!("Executed spectrum transformation in {:?}", start.elapsed().as_secs());
    SourceList::new_from(algamated_sources)






}

