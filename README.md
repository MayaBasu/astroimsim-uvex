# A UVEX Image Simulator in Rust

The following provides a walk through of how to set up and run the simulator.





# Configuring the Details

We first make a configuration file for the instrument which contains paths to all the data files to be used. We do this by first generating a default configuration file and then editing it as needed.

```rust
fn main() {
    //Generate a default configuration file
    let configuration = UVEXConfiguration::default();
    //write this configuration file to config.yaml
    configuration.to_yaml("config.yaml");
}
```


Viewing the config.yaml file which hs just been created, we get the following:


```yaml


fuv_contamination:
  - On
  - ~/Desktop/uvex/spectral_response/UVIM_FUV_contamination.dat
nuv_contamination:
  - On
  - ~/Desktop/uvex/spectral_response/UVIM_NUV_contamination.dat
fuv_response:
  - On
  - ~/Desktop/uvex/spectral_response/UVIM_FUV_filter_response.dat
nuv_response:
  - On
  - ~/Desktop/uvex/spectral_response/UVIM_NUV_filter_response.dat
nuv_qe_response:
  - On
  - ~/Desktop/uvex/spectral_response/UVIM_NUV_QE.dat
dichroic:
  - On
  - ~/Desktop/uvex/spectral_response/UVIM_dichroic_response.dat
mirror_response:
  - On
  - ~/Desktop/uvex/spectral_response/mirror_reflectivity.dat
fuv_psf:
  - On
  - ~/Desktop/uvex/FUV_PSF
  - ~Desktop/uvex/FUV_PSF_BLURRED
nuv_psf:
  - On
  - ~/Desktop/uvex/NUV_PSF
  - ~/Desktop/uvex/NUV_PSF_BLURRED
fuv_flatfield_illumination:
  - On
  - ~/Desktop/uvex/vinietting/FUV_vignetting_model_4096.fits
nuv_flatfield_illumination:
  - On
  - ~/Desktop/uvex/vinietting/NUV_vignetting_model_4096.fits
fuv_read_noise:
  - On
  - ~/Desktop/uvex/detector_effects/fuv_read_noise
nuv_read_noise:
  - On
  - ~/Desktop/uvex/detector_effects/nuv_read_noise
fuv_dark_current:
  - On
  - ~/Desktop/uvex/detector_effects/fuv_dark_current
nuv_dark_current:
  - On
  - ~/Desktop/uvex/detector_effects/nuv_dark_current
fuv_dead_pixels:
  - On
  - ~/Desktop/uvex/detector_effects/fuv_dead_pixels
nuv_dead_pixels:
  - On
  - ~/Desktop/uvex/detector_effects/nuv_dead_pixels
x_gap: 0.0
y_gap: 0.0
gaussian_blur_std:
  - On
  - 20.0



```
The default paths are for macOS, and assume that the uvex data is stored in a folder on your Desktop called "uvex", with the following file tree:

```text

uvex/
├── FUV_PSF/
│   ├── UVEX_FUV_PSF_1um_F001.fits
│   ├── UVEX_FUV_PSF_1um_F002.fits
│   └── etc...
├── NUV_PSF/
│   ├── UVEX_NUV_PSF_1um_F001.fits
│   ├── UVEX_NUV_PSF_1um_F002.fits
│   └── etc...
├── FUV_PSF_BLURRED
├── NUV_PSF_BLURRED
├── spectral_response/
│   ├── mirror_reflectivity.dat
│   ├── UVIM_dichroic_response.dat
│   ├── UVIM_FUV_contamination.dat
│   ├── UVIM_FUV_filter_response.dat
│   ├── UVIM_NUV_contamination.dat
│   ├── UVIM_NUV_filter_response.dat
│   └── UVIM_NUV_QE.dat
├── vinietting/
│   ├── FUV_vignetting_model_4096.fits
│   └── NUV_vignetting_model_4096.fits
└── detector_effects/
    ├── fuv_read_noise
    ├── nuv_read_noise
    ├── fuv_dark_current
    ├── nuv_dark_current
    ├── fuv_dead_pixels
    └── nuv_dead_pixels

```


If you do not want to have your data stored in this structure or have different file names, simply change each path in the configuration file to point appropriatly. FUV_PSF_BLURRED and NUV_PSF_BLURRED are empty directories which will be filled with psf files after the original psf files (in FUV_PSF and NUV_PSF) have been convolved with a gaussian.

The directories in detector_effects (fuv_read_noise, nuv_dark current, etc) as also empty. When there is experimental data for these effects, there will be 9 files in each of these directories, one for each detector plane. However, at the moment, we will be populating them with synthetic data.



```rust

fn main() {
    //initialize the uvex instrument with the details in "configuration/details.yaml"
    let uvex = uvex::initialize_uvex("configuration/uvex");
}

```


# Sources

We can create single point source by specifying a position in the view area (values for x and y between 0 and 1) as well as a spectrum and an overall brightness factor which is between 0 and 1.
 ```rust

fn main() {
    let spectrum: [f64;spectral_resolution] = [0.1,0.13,/* Insert your spectrum here */0.2];
    let source_x = 0.2;
    let source_y = 0.3;
    let luminosity = 0.6;
    let point_source = point_source::new(source_x,source_y, spectrum,luminosity);
}

```
The UVEX instrument takes a source_list, which is a group of point_sources. We can either add custom point_sources to an empty source_list, or we can initialize a random source list to look at.


```rust
fn main() {
    
    //make three point sources: point_source1, point_source2, and point_source3
    //make a source list containing the first two sources
    let mut curated_source_list = source_list::new_from(vec![point_source1,point_source2]);
    //we can also add sources to an existing list
    curated_source_list.add_source(point_source3);
    
    //alternatively, we can generate a random star field of sources which all share a spectrum but which 
    
    
    let random_source_list = source_list::new_random_point_source_field(number_of_point_sources, //how many points
                                                                  min_brightness, //minimum brightness of the random range
                                                                  max_brightness, //maximum brightness of the random range
                                                                  min_x,//minimum x position of the random range
                                                                  max_x,//maximum x position of the random range
                                                                  min_y,//minimum y position of the random range
                                                                  max_y,//maximum x position of the random range
                                                                  spectrum); //shared spectrum
    
}


```


Once you have a source_list you are ready to feed it into the uvex_instrument and get images.

