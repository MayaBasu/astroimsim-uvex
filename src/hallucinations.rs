extern crate fitrs;
use fitrs::{Fits, Hdu};

pub fn hallucinate_dark_current(){

    // Make example dummy data array
    let shape = [20, 20];
    let data = (0..shape[0])
        .map(|i| (0..shape[1]).map(move |j| i + j))
        .flatten()
        .collect();
    let mut primary_hdu = Hdu::new(&[20, 20], data);
    // Insert values in header
    primary_hdu.insert("KEYSTR", "My string");
    primary_hdu.insert("KEYSTR2", "Whatever value you want to save in this FITS files. Continued (long) strings are supported, if you happen to care.");
    primary_hdu.insert("KEYFLOAT", 3.14);
    primary_hdu.insert("KEYINT", 42);

    // Save file
    Fits::create("new_file.fits", primary_hdu).expect("Failed to create");

}

