use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use ndarray::{Array, Array2};
use std::fmt::Debug;
use rand::{SeedableRng};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

#[derive(PartialEq)]
#[derive(Debug)]
enum Type {
	Int,
	Float,
	Bool,
	Char,
	Str,
}

#[derive(Debug)]
pub struct DataFrame {
    columns: HashMap<usize, String>,
    data: Vec<Vec<String>>,
	shape: (usize, usize),
}

impl DataFrame {
    pub fn new() -> DataFrame {
        DataFrame {
			columns: HashMap::new(),
            data: Vec::new(),
			shape: (0, 0),
        }
    }

	pub fn from_vec(vec: Vec<Vec<String>>, header: &Vec<String>) -> DataFrame {
		let mut columns = HashMap::new();

		let mut index: usize = 0;
		for column in header {
			columns.insert(index, column.to_string());
			index += 1;
		}

		let shape = (vec.len() as usize, vec[0].len() as usize);

		DataFrame {
			columns: columns,
			data: vec,
			shape: shape,
		}
	}

//-------------------------------------------------------------------------
	
	//You have to sort the keys with value = index.
	fn get_header(&self) -> Vec<String> {
		let mut header = Vec::new();
		for index in 0..self.columns.len() {
			header.push(self.columns.get(&index).unwrap().to_string());
		}

		header
		
	}

//-------------------
//-------------------
//-----------------Fix Up this

	//Return Data
	fn get_data(&self) -> &Vec<Vec<String>> { &self.data }
	
	pub fn shape(&self) -> (usize, usize) { self.shape }

	//Return DataFrame from csv.
    pub fn csv_to_dataframe(&mut self, path: &str) {
        let file = File::open(path).expect("file not found");

        let mut bufread = BufReader::new(file);    
        let mut line = String::new();
		
		//Read header line.
		match bufread.read_line(&mut line) {
			Ok(_) => {()}, //Nothing to do
			Err(error) => println!("error: {}", error),
		}
		
		//Set the dataframe.columns with header and index.
		{
			let header = match line.lines().next() {
				Some(result) => result.to_string(),
				None => panic!("Nothing to read"),
			};

			let header = split_comma(&header);
			let mut index: usize = 0;
			for column in header {
				self.columns.insert(index, column);
				index += 1;
			}
		}
		
        let mut rows_string = Vec::new();
        
		//Insert the rows in vec as string each other.
		for line in bufread.lines() {
            let row = match line {
                Ok(values_string) => values_string,
                Err(error) => {
                    panic!("Some problem occur: {:?}", error)
                }
            };
            rows_string.push(row);
        }
		

		//Split the value from row_string, and set the dataframe.data
		self.shape = (rows_string.len() as usize, self.columns.len() as usize);
        for row_string in rows_string.iter() {
			let row_value = split_comma(row_string);
            self.data.push(row_value);
        }

    }	


// I think, I'll do not use this code.
/*	pub fn load_dataset_as_i32(&self, columns: &[&str]) -> Array2<i32> {
		let columns_index = find_column_index(&self.columns, columns);
		
		let row_num = self.shape.0;
				
		//Start type checking and push the value in 1-dimension vector.
		let mut tmp = Vec::new();
		let type_check = infer_type(&self.data[0][0]);
		for row in 0..row_num {
			for &column in &columns_index {
				let type_infer = infer_type(&self.data[row as usize][column]);
				if type_infer != Type::Int {
					panic!("Can't parse all the data.");
				}
				tmp.push(&self.data[row as usize][column]);
			}
		}
		//End type checking.
		
		//Make dataset of i32.
		let arr_shape = (row_num as usize, columns.len() as usize);
		let dataset = match type_check {
			Type::Int => {
				let arr =
					Array::from_shape_vec(arr_shape,
					tmp.iter()
					.map(|value| value.parse::<i32>().unwrap())
					.collect())
					.unwrap();
				arr
				}
			_ => { panic!("Something is wrong in make ndarray") },
		};
		dataset
	}
*/

	pub fn load_dataset_as_f64(&self, columns: &[&str]) -> Array2<f64> {
		let columns_index = find_column_index(&self.columns, columns);
		
		let row_num = self.shape.0;
				
		//Start type checking.
		let mut tmp = Vec::new();
		let type_check = match infer_type(&self.data[0][0]) {
			Type::Int => { Type::Float },
			Type::Float => { Type::Float },
			_ => { panic!("Can't parse F64") },

		};
		
		//Start type checking and push the value in 1-dimension vector.
		for row in 0..row_num {
			for &column in &columns_index {
				let type_infer = infer_type(&self.data[row as usize][column]);
				match type_infer {
					Type::Float => { tmp.push(&self.data[row as usize][column]) },
					Type::Int => { tmp.push(&self.data[row as usize][column]) },
					_ => { panic!("Can't parse all the data."); }
				}
			}
		}
		//End type checking.
		
		//Make dataset of f64.
		let arr_shape = (row_num as usize, columns.len() as usize);
		let dataset = match type_check {
			Type::Float => {
				let arr =
					Array::from_shape_vec(arr_shape,
					tmp.iter()
					.map(|value| value.parse::<f64>().unwrap())
					.collect())
					.unwrap();
				
				arr
				}
			_ => { panic!("Something is wrong in make ndarray") },
		};
		dataset
		}
	}
    
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
	let value = string[prev_index..((string_len) as usize)].to_string(); 
	slice_vec.push(value);
	slice_vec
}

fn infer_type(value: &str) -> Type
{
	let try_parse_int = value.parse::<i32>();
	let try_parse_float = value.parse::<f32>();
	let try_parse_bool = value.parse::<bool>();
	let try_parse_char = value.parse::<char>();
	
	match try_parse_int {
		Ok(_) => { return Type::Int; }
		_ => (),
	}

	match try_parse_float {
		Ok(_) => { return Type::Float; }
		_ => (),
	}

	match try_parse_bool {
		Ok(_) => { return Type::Bool; }
		_ => (),
	}

	match try_parse_char {
		Ok(_) => { return Type::Char; }
		_ => { return Type::Str; }
	}
}

fn find_column_index(columns: &HashMap<usize, String>, target: &[&str]) -> Vec<usize> {
	let mut columns_index = Vec::new();
	let mut isfind = 0 as usize;
	for column in target {
		isfind = 0;
		for (index, value) in columns {
			if value == column {
				columns_index.push(*index);
				isfind = 1;
				break;
			}
		}
		if isfind == 0 {
		panic!("The dataframe doesn't have the key: {}", column); 
		}
	}
	columns_index
}

pub fn train_test_split(x: &DataFrame, y: &DataFrame, test_size: f32, random_state: u64) -> (DataFrame, DataFrame, DataFrame, DataFrame) { 
	let row_num = x.shape.0;
	let mut test_indexes: Vec<usize>= vec![0; row_num];
	for i in 0..row_num {
		test_indexes[i] += i;
	}
	
	let split_num = (row_num as f32 * test_size).round() as usize;
	
	let mut rng = StdRng::seed_from_u64(random_state);
	
	test_indexes.shuffle(&mut rng);
	
	test_indexes.drain(split_num..);

	test_indexes.sort();
	test_indexes.reverse();
	

	let mut train_x_data = x.data.clone();
	let mut train_y_data = y.data.clone();
	let mut test_x_data = Vec::new();
	let mut test_y_data = Vec::new();


	for index in test_indexes {
		test_x_data.push(train_x_data.remove(index));
		test_y_data.push(train_y_data.remove(index));
	}

	let x_header = x.get_header();
	println!("x_header: {:?}", x_header);
	let y_header = y.get_header();
	println!("y_header: {:?}", y_header);

	( DataFrame::from_vec(train_x_data, &x_header),
	  DataFrame::from_vec(test_x_data, &x_header),
	  DataFrame::from_vec(train_y_data, &y_header),
	  DataFrame::from_vec(test_y_data, &y_header),
	  )
}


