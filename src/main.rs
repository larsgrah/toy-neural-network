extern crate rand;
extern crate ngalgebra;

fn main() {
    println!("Hello, world!");
}

struct NeuralNetwork {
    weights: Vec<f64>,
}

impl NeuralNetwork {
    pub fn new() -> NeuralNetwork{
        let mut network = NeuralNetwork {weights: Vec::new()};
        network.weights.push(rand::random::<f64>());
        network.weights.push(rand::random::<f64>());
        network.weights.push(rand::random::<f64>());
        network
    }

    pub fn train(&self, training_set_inputs: &[f64], training_set_outputs: &[f64]) {
        let output = &self.think(&training_set_inputs);

        let error = training_set_outputs - output;

        let adjustment = dot()
    }

    fn think(&self, inputs: &[f64]) -> f64 { 
        sigmoid(dot(&inputs, &self.weights))
    }
}

fn sigmoid(x: i64) -> f64 {
    1f64 / (1f64 + f64::powf(std::f64::consts::E, -x as f64))
}

fn derived_sigmoid(x: i64) -> i64 {
    x * (1 - x)
}

