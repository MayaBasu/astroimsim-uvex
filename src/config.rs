use std::io::Write;
use std::fs;
use serde::{Deserialize, Serialize};
use crate::config::UseEffect::On;

#[derive(Serialize,Deserialize,Clone)]
pub enum UseEffect{
    On,
    Off
}
#[derive(Serialize,Deserialize,Clone)]

pub struct UVEXConfiguration {
    pub fuv_contamination: (UseEffect,String),
    pub nuv_contamination: (UseEffect,String),

    pub fuv_response: (UseEffect,String),
    pub nuv_response: (UseEffect,String),

    pub nuv_qe_response: (UseEffect,String), //TO BE REMOVED SOON

    pub dichroic: (UseEffect,String),
    pub mirror_response: (UseEffect,String),

    pub fuv_psf: (UseEffect,String, String),
    pub nuv_psf: (UseEffect,String,String),

    pub fuv_flatfield_illumination: (UseEffect,String),
    pub nuv_flatfield_illumination: (UseEffect,String),

    pub fuv_read_noise:  (UseEffect,String),
    pub nuv_read_noise: (UseEffect,String),

    pub fuv_dark_current: (UseEffect,String),
    pub nuv_dark_current: (UseEffect,String),

    pub fuv_dead_pixels: (UseEffect,String),
    pub nuv_dead_pixels: (UseEffect,String),

    pub x_gap: f64,
    pub y_gap: f64,

    pub gaussian_blur_std: (UseEffect,f64), //in oversampled pixels
    
    pub read_noise: (UseEffect, f64,f64), //in electrons
    pub dark_current: (UseEffect,f64,f64), //in electrons
    pub dead_pixels: (UseEffect, usize,usize,usize,usize)
    /*
    Number of scatterd pixels
    number of dead rows
    number of dead columns
    number of dead rectangles
     */
    




}


impl UVEXConfiguration{

    pub fn default(default:UseEffect)-> UVEXConfiguration{
        UVEXConfiguration{
            fuv_contamination: (default.clone(), "/Users/mayabasu/Desktop/uvex/spectral_response/UVIM_FUV_contamination.dat".to_string()),
            nuv_contamination: (default.clone(),"/Users/mayabasu/Desktop/uvex/spectral_response/UVIM_NUV_contamination.dat".to_string()),

            fuv_response: (default.clone(),"/Users/mayabasu/Desktop/uvex/spectral_response/UVIM_FUV_filter_response.dat".to_string()),
            nuv_response: (default.clone(),"/Users/mayabasu/Desktop/uvex/spectral_response/UVIM_NUV_filter_response.dat".to_string()),

            nuv_qe_response: (default.clone(),"/Users/mayabasu/Desktop/uvex/spectral_response/UVIM_NUV_QE.dat".to_string()),

            dichroic: (default.clone(),"/Users/mayabasu/Desktop/uvex/spectral_response/UVIM_dichroic_response.dat".to_string()),
            mirror_response: (default.clone(),"/Users/mayabasu/Desktop/uvex/spectral_response/mirror_reflectivity.dat".to_string()),

            fuv_psf: (default.clone(),"/Users/mayabasu/Desktop/uvex/FUV_PSF".to_string(),"~/Desktop/uvex/FUV_PSF_BLURRED".to_string()),
            nuv_psf: (default.clone(),"/Users/mayabasu/Desktop/uvex/NUV_PSF".to_string(),"~/Desktop/uvex/NUV_PSF_BLURRED".to_string()),

            fuv_flatfield_illumination: (default.clone(),"/Users/mayabasu/Desktop/uvex/vinietting/FUV_vignetting_model_4096.fits".to_string()),
            nuv_flatfield_illumination: (default.clone(),"/Users/mayabasu/Desktop/uvex/vinietting/NUV_vignetting_model_4096.fits".to_string()),

            fuv_read_noise:  (default.clone(),"/Users/mayabasu/Desktop/uvex/detector_effects/fuv_read_noise".to_string()),
            nuv_read_noise: (default.clone(),"/Users/mayabasu/Desktop/uvex/detector_effects/nuv_read_noise".to_string()),

            fuv_dark_current: (default.clone(),"/Users/mayabasu/Desktop/uvex/detector_effects/fuv_dark_current".to_string()),
            nuv_dark_current: (default.clone(),"/Users/mayabasu/Desktop/uvex/detector_effects/nuv_dark_current".to_string()),

            fuv_dead_pixels:(default.clone(), "/Users/mayabasu/Desktop/uvex/detector_effects/fuv_dead_pixels".to_string()),
            nuv_dead_pixels: (default.clone(),"/Users/mayabasu/Desktop/uvex/detector_effects/nuv_dead_pixels".to_string()),

            x_gap: 0.0,
            y_gap: 0.0,

            gaussian_blur_std: (default.clone(),20.0),
            
            read_noise: (default.clone(),2.0,1.0),
            dark_current: (default.clone(),0.01,0.005),
            dead_pixels: (default.clone(), 100,10,10,5)


        }
    }


    pub fn to_yaml(&self, path:&'static str){
        println!("Writing configuration to {:?}", path);
        let serialized_self = serde_yaml::to_string(&self).expect("Failed to YAMLify the object");
        let mut file = fs::File::create(path).expect("Couldn't create the config file");
        write!(file, "{}", serialized_self).expect("Failed to write YAML to config file");

    }

    pub fn from_yaml(path:&'static str)-> UVEXConfiguration{
        let config: String = fs::read_to_string(path).expect("couldn't read from config file");
        let config: UVEXConfiguration = serde_yaml::from_str(config.as_str()).expect("invalid details data");
        config

    }


}