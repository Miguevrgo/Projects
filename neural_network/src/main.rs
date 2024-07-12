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

fn transpose<T>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    
}

fn visualize_data(data: &[Vec<f64>], num_samples: usize) {
    for i in 0..num_samples {
        let label = data[i][0] as usize;
        println!("Label: {}", label);

        for y in 0..28 {
            for x in 0..28 {
                let pixel = data[i][1 + y * 28 + x];
                print!("{:3} ", (pixel) as u8);
            }
            println!();
        }
        println!();
    }
}

fn main() {
    let file_path = "data/mnist_test.csv";
    match read_csv(file_path) {
        Ok(data) => {
            
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
