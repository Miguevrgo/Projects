use std::error::Error;

fn read_csv(file_path: &str) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut data = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let row = record.iter().map(|x| x.parse::<f64>().unwrap()).collect::<Vec<f64>>();
        data.push(row);
    }
    Ok(data)
}

// for x in 0..28 {
//     let pixel = data[i][1 + y * 28 + x];
//     if pixel == 0.0 {
//         print!("    ");
//     } else {
//         print!("\x1b[48;2;{};{};{}m   \x1b[0m", (pixel * 255.0) as u8, (pixel * 255.0) as u8, (pixel * 255.0) as u8);
//     }
// }
// println!();

fn visualize_data(data: &[Vec<f64>], num_samples: usize) {
    for i in 0..num_samples {
        let label = data[i][0] as usize;
        println!("Label: {}", label);

        for y in 0..28 {
            for x in 0..28 {
                let pixel = data[i][1 + y * 28 + x];
                print!("{:3} ", (pixel * 255.0) as u8);
            }
            println!();
        }
        println!();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "data/mnist_test.csv";
    let data = read_csv(file_path)?;

    println!("Mostrando los primeros 5 ejemplos del conjunto de datos:");
    visualize_data(&data, 5);

    Ok(())
}
