use std::fs::File;
use std::io::{BufReader, BufRead};

pub struct DataFrame {
    header: Vec<String>,
    data: Vec<Vec<String>>,
}

impl DataFrame {
    pub fn new() -> DataFrame {
        DataFrame {
            header: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn csv_to_dataframe(&mut self, path: &str) {
        let file = File::open(path).expect("file not found");
        let mut bufread = BufReader::new(file);    
        let mut line = String::new();
		
		match bufread.read_line(&mut line) {
			Ok(_) => {}, //Nothing to do
			Err(error) => println!("error: {}", error),
		}

		let header = match line.lines().next() {
			Some(result) => result.to_string(),
			None => panic!("Nothing to read"),
		};

		self.header = split_comma(&header);
		

        let mut rows_string = Vec::new();
        
		for line in bufread.lines() {
            let row = match line {
                Ok(values_string) => values_string,
                Err(error) => {
                    panic!("Some problem occur: {:?}", error)
                }
            };
            rows_string.push(row);
        }
        
        for row_string in rows_string.iter() {
            println!("low_string: {}", row_string);
			let row_value = split_comma(row_string);
            self.data.push(row_value);
        }
    }
}

//let dataset_x = df_x.load_dataset();
//return ndarray. default is all. you can enter load_dataset("some key1", "some key3").
//let dataset_y = df_y.load_dataset(); //return ndarray
    


fn split_comma(string: &str) -> Vec<String>{
    let mut slice_vec = Vec::new();
	let string_len = string.len();
	let mut prev_index: usize = 0;
	let mut check_quote = 0;
	let string_byte = string.as_bytes();

	for index in 0..string_len {
		let b = string_byte[index] as char;
		if b != '"' {
			if b == ',' {
				if check_quote == 0 {
					let value = string[prev_index..index].to_string();
					slice_vec.push(value);
					prev_index = (index) + 1;
				}
			}
		}
		else {
			check_quote = (check_quote + 1) % 2;
		}
	}
	let value = string[prev_index..((string_len - 1) as usize)].to_string(); 
	slice_vec.push(value);
	println!("{}", slice_vec.len());
	slice_vec
}

