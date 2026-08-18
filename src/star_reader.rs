use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_stars(){
    let path = "/Users/mayabasu/Desktop/uvex_fov_plane_gaiadr2_dust_260811.ecsv";
    let file = File::open(path).unwrap();
    let delineator = " "
;    let reader = BufReader::new(file);

    let mut data: Vec<Vec<f64>> = Vec::new();
    for line in reader.lines(){

        let mut parsable = true;
        let line = line.expect("Failed to read line");

        let line = line.trim()
            .split(&delineator)
            .map(|a|
                {//println!("{:?}",a.trim());
                    a.trim().parse::<f64>()})
            .map(|result|
                match result {
                    Ok(value) => {//println!("{:?}",value);
                        value},
                    Err(v) => {//println!("COULDN:T PARSE {:?}",v);
                        parsable = false; 0.0},
                }).collect();
        // println!("{:?}",line);
        if parsable{
            data.push(line)
        }
        // println!("{:?} ",parsable);


    }
    for i in 0..30{
        println!("{:?}", data[i])
    }
    for i in 0..100{
        let count_1 = data.iter().filter(|v|v[3] ==i as f64).count();
        println!("count {i}: {:?}",count_1);

    }





}