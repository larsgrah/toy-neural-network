extern crate rand;

#[macro_use(array)]
extern crate ndarray;
use ndarray::Array1;
use ndarray::Array2;

fn main() {
    let mut neural_network = NeuralNetwork::new();
    
    let inputs = array![[0.,0.,1.],
                        [1.,1.,1.],
                        [1.,0.,1.],
                        [0.,1.,1.]];

    let outputs = array![0., 1., 1., 0.];

    let unknown = array![1., 0., 0.];

    neural_network.train(&inputs, &outputs, 10000);

    print!("{}", neural_network.think_1_dim(&unknown));


}

struct NeuralNetwork {
    weights: Array1<f64>,
}

impl NeuralNetwork {
    pub fn new() -> NeuralNetwork {
        let mut network = NeuralNetwork {weights: array![rand::random::<f64>(), rand::random::<f64>(), rand::random::<f64>()]};
        network
    }

    pub fn train(&mut self, training_set_inputs: &Array2<f64>, training_set_outputs: &Array1<f64>, iter: i32) {
        for i in (1..iter) {
            let output = &self.think(&training_set_inputs);

            let error = training_set_outputs - output;

            let adjustment = training_set_inputs.t().dot(&(error * &derived_sigmoid(output)));

            self.weights = &self.weights + &adjustment;
        }
        
    }

    pub fn think(&self,inputs: &Array2<f64>) -> Array1<f64> { 
        sigmoid(&inputs.dot(&self.weights))
    }

    pub fn think_1_dim(&self, inputs: &Array1<f64>) -> Array1<f64> { 
        sigmoid(&array![inputs.dot(&self.weights)])
    }
}

fn sigmoid(x: &Array1<f64>) -> Array1<f64> {
    matrix_pow_e(x).map(|y| y + 1.).map(|val| 1. / val)
}

fn derived_sigmoid(matrix: &Array1<f64>) -> Array1<f64> {
    matrix.map(|x| x * (1. - x))
}

fn matrix_pow_e(matrix: &Array1<f64>) -> Array1<f64> {
    matrix.map(|x| std::f64::consts::E.powf(-x))
}
