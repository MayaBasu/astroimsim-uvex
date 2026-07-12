use std::time::Instant;
pub fn benchpress(){
    let pixels = 4096*3;
    let mut detector_data = Vec::new();
    let mut effect = Vec::new();
    for row in 0..pixels{
        let mut detector_row = Vec::new();
        let mut effect_row = Vec::new();
        for column in 0..pixels{
            detector_row.push((row+column)as f64);
            effect_row.push(1.0/(row as f64 + 3.0));

        }
        detector_data.push(detector_row);
        effect.push(effect_row);
    }
    println!("sum is {:?}", detector_data.iter().flatten().sum::<f64>());
    let start = Instant::now();

    for row in 0..pixels{
        for column in 0..pixels{
           detector_data[row][column] = effect[row][column] * detector_data[row][column];
        }

    }
    println!("sum is {:?}, duration is {:?}",
             detector_data.iter().flatten().sum::<f64>(), start.elapsed().as_millis());


}