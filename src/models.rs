use ndarray::{Axis, Array2};
use ndarray;
//Implement
//
/*let mut logreg = LogisticRegression::new(); //Object for Logistic Regression parameter's information.
    logreg.feed(x_train, y_train); //Feed x and y. Then set the matrix size.
    logreg.train(weight, learning_rate, iter); //Train the algorithm with parameters.
    logreg.info(); //print weight, learning_rate, iter ...)
    
    let pred_y_test = logreg.predict(x_test); //Predict something with the object's parameters.
    let pred_y_train = logreg.predict(x_train); //Same with upper line.

    let scores = scoring(pred_y_test, y_test); //Set precision, recall, f1score.
    
    let train_error_rate = compute_error_rate(pred_y_train, y_train); //Compute error.
    let test_error_rate = compute_error_rate(pred_y_test, y_test); //Same with upper line.
*/

#[derive(Debug)]
pub struct LogisticRegression{
	bias: Option<Array2<f64>>,
	weight: Option<Array2<f64>>,
	learning_rate: Option<f64>,
	iter: Option<usize>,
	costs: Option<Vec<f64>>,
}

impl LogisticRegression {
	pub fn new() -> LogisticRegression{
		LogisticRegression {
			bias: None,
			weight: None,
			learning_rate: None,
			iter: None,
			costs: None,
		}
	}


	pub fn train(&mut self, x: &Array2<f64>, y: &Array2<f64>, mut w: Array2<f64>, mut bias: Array2<f64>, learning_rate: f64, iter: usize) {
		let m = x.shape()[0];
		let mut costs = Vec::<f64>::new();
	
		for _ in 0..iter {
			let z = x.dot(&w) + &bias;
			println!("z : {:?}", z);
			let y_hat = sigmoid_f64(&z);
			let cost = compute_cost(y, &y_hat);
			println!("cost: {:?}", cost);
			if !cost.is_nan() {
				costs.push(cost);
			}

			println!("y_hat: {}",y_hat);
			let dz = y_hat - y;
			let dw = x.t().dot(&dz) / (m as f64);
			let db = dz.sum_axis(Axis(0)) / (m as f64);

			//now you have to do ndarray compute.
			println!("w: {:?}", w);
			w = w - dw * learning_rate;
			println!("bias: {:?}", bias);
			bias = bias - db * learning_rate;
		}
		self.bias = Some(bias);
		self.weight = Some(w);
		self.learning_rate = Some(learning_rate);
		self.iter = Some(iter);
		self.costs = Some(costs);
	}

	pub fn info(&self) {
		println!("{:#?}", self);
	}
}

fn sigmoid_f64(target: &Array2<f64>) -> Array2<f64>{
	let result = target.mapv(|target| (1.0 / (1.0 + (-target).exp())));
	result
}

fn compute_cost(y: &Array2<f64>, a: &Array2<f64>) -> f64{
	let ln_a = a.mapv(|a| (a.ln()));
	let tmp = 1.0 - a;
	println!("tmp: {:?}", tmp);
	let ln_1_minus_a = tmp.mapv(|tmp| (tmp.ln()));

	(-((ln_a * y) + (ln_1_minus_a * (1.0 - y)))).sum()
}
