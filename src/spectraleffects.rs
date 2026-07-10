use astroimsim_data::datafile::DataFile;
const FUV_CONTAMINATION_PATH: &'static str = "/Users/mayabasu/Desktop/uvex_psf_files/UVIM_FUV_contamination.dat";
const NUV_CONTAMINATION_PATH: &'static str = "/Users/mayabasu/Desktop/uvex_psf_files/UVIM_NUV_contamination.dat";
const FUV_RESPONSE_PATH: &'static str = "/Users/mayabasu/Desktop/uvex_psf_files/UVIM_FUV_filter_response.dat";
const NUV_RESPONSE_PATH: &'static str = "/Users/mayabasu/Desktop/uvex_psf_files/UVIM_NUV_filter_response.dat";
const NUV_QE_CURVE_PATH: &'static str = "/Users/mayabasu/Desktop/uvex_psf_files/UVIM_NUV_QE.dat";
const DICHROIC_PATH: &'static str = "/Users/mayabasu/Desktop/uvex_psf_files/UVIM_dichroic_response.dat";


pub struct Configuration{
    FrequencyResponseFiles: Vec<astroimsim_data::datafile>
}