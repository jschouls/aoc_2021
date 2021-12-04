use std::fs::File;
use std::io::{self, BufRead, Error};

fn main() -> Result<(), Error> {
    let num = count_number_depth_increases("data/input.txt")?;

    println!("Number: {}", num);
    Ok(())
}

fn count_number_depth_increases(p: &str) -> Result<u32, Error> {
    println!("Reading file:{} and count number of incements depths", p);
    let file = File::open(p)?;
    let lines = io::BufReader::new(file).lines();

    // Previous is in first iteration not available.
    let mut last_depth = 0i32;
    let mut num_depth_increased = 0u32;
    for line in lines {
        let new_depth_measure = line.unwrap().parse::<i32>().unwrap();

        // if new_depth - last depth is > 0, then depth has increased.
        let is_increased = (new_depth_measure - last_depth) > 0;

        println!(
            "{} ({})",
            new_depth_measure,
            if is_increased == true {
                num_depth_increased += 1;
                "increased"
            } else {
                "decreased"
            }
        );
        last_depth = new_depth_measure;
    }

    // Because the first one does not count. (No previous depth result)
    num_depth_increased = num_depth_increased - 1;

    Ok(num_depth_increased)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_increment() {
        // Test data from example
        let num = crate::count_number_depth_increases("data/test_data.txt");
        assert_eq!(num.unwrap(), 7);
    }
}
