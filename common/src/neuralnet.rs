use rand::prelude::*;

// Define a trait for matrix multiplication
pub trait MatrixMultiplier {
    fn multiply(&self, a: &[f32], b: &[f32], sizes: (usize, usize, usize)) -> Vec<f32>;
}

// Implement the trait for the default matrix multiplication
struct DefaultMatrixMultiplier;

impl MatrixMultiplier for DefaultMatrixMultiplier {
    fn multiply(&self, a: &[f32], b: &[f32], sizes: (usize, usize, usize)) -> Vec<f32> {
        let (rows_a, cols_a, cols_b) = sizes;
        let mut result = vec![0.0; rows_a * cols_b];

        for i in 0..rows_a {
            for j in 0..cols_b {
                for k in 0..cols_a {
                    result[i * cols_b + j] += a[i * cols_a + k] * b[k * cols_b + j];
                }
            }
        }

        result
    }
}

pub struct NeuralNetwork<M>
where
    M: MatrixMultiplier,
{
    input_size: usize,
    hidden_size: usize,
    output_size: usize,
    learning_rate: f32,
    weights_input_hidden: Vec<f32>,
    weights_hidden_output: Vec<f32>,
    bias_hidden: Vec<f32>, // Bias for the hidden layer
    bias_output: Vec<f32>, // Bias for the output layer
    multiplier: M,
}

impl<M> NeuralNetwork<M>
where
    M: MatrixMultiplier,
{
    // Constructor to initialize the neural network
    pub fn new(
        input_size: usize,
        hidden_size: usize,
        output_size: usize,
        learning_rate: f32,
        multiplier: M,
    ) -> Self {
        let mut rng = thread_rng();

        // Initialize weights with random values
        let weights_input_hidden = (0..input_size * hidden_size)
            .map(|_| rng.gen_range(-0.5..0.5))
            .collect();
        let weights_hidden_output = (0..hidden_size * output_size)
            .map(|_| rng.gen_range(-0.5..0.5))
            .collect();

        // Initialize biases with random values
        let bias_hidden = (0..hidden_size).map(|_| rng.gen_range(-0.5..0.5)).collect();
        let bias_output = (0..output_size).map(|_| rng.gen_range(-0.5..0.5)).collect();

        NeuralNetwork {
            input_size,
            hidden_size,
            output_size,
            learning_rate,
            weights_input_hidden,
            weights_hidden_output,
            bias_hidden,
            bias_output,
            multiplier,
        }
    }


    // Sigmoid activation function
    pub fn sigmoid(&self, x: f32) -> f32 {
        1.0 / (1.0 + (-x).exp())
    }

    // Forward propagation to compute predictions
    pub fn forward(&self, inputs: &[f32]) -> Vec<f32> {
        // Calculate hidden layer activations (including bias)
        let mut hidden_activations = self.multiplier.multiply(
            inputs,
            &self.weights_input_hidden,
            (1,
            self.input_size,
            self.hidden_size),
        );
        for i in 0..self.hidden_size {
            hidden_activations[i] += self.bias_hidden[i];
        }
        let hidden_activations = hidden_activations.iter().map(|&x| self.sigmoid(x)).collect::<Vec<f32>>();

        // Calculate output layer activations (including bias)
        let mut output_activations = self.multiplier.multiply(
            &hidden_activations,
            &self.weights_hidden_output,
            (1,
            self.hidden_size,
            self.output_size),
        );
        for i in 0..self.output_size {
            output_activations[i] += self.bias_output[i];
        }
        output_activations.iter().map(|&x| self.sigmoid(x)).collect::<Vec<f32>>()
    }

    // Backpropagation to update weights and biases based on loss
    pub fn backpropagate(&mut self, inputs: &[f32], targets: &[f32]) {
        // Forward pass
        let mut hidden_activations = self.multiplier.multiply(
            inputs,
            &self.weights_input_hidden,
            (1,
            self.input_size,
            self.hidden_size),
        );
        for i in 0..self.hidden_size {
            hidden_activations[i] += self.bias_hidden[i];
        }
        let hidden_activations = hidden_activations.iter().map(|&x| self.sigmoid(x)).collect::<Vec<f32>>();

        let mut output_activations = self.multiplier.multiply(
            &hidden_activations,
            &self.weights_hidden_output,
            (1,
            self.hidden_size,
            self.output_size),
        );
        for i in 0..self.output_size {
            output_activations[i] += self.bias_output[i];
        }
        let output_activations = output_activations.iter().map(|&x| self.sigmoid(x)).collect::<Vec<f32>>();

        // Compute output layer error
        let mut output_errors = vec![0.0; self.output_size];
        for i in 0..self.output_size {
            output_errors[i] = output_activations[i] - targets[i];
        }
        
        let mut output_deltas = vec![0.0; self.output_size];
        for i in 0..self.output_size {
            output_deltas[i] = output_errors[i] * output_activations[i] * (1.0 - output_activations[i]);
        }

        // Compute hidden layer error
        let mut hidden_errors = vec![0.0; self.hidden_size];
        for i in 0..self.hidden_size {
            let mut error = 0.0;
            for j in 0..self.output_size {
                error += self.weights_hidden_output[i * self.output_size + j] * output_deltas[j];
            }
            hidden_errors[i] = error * hidden_activations[i] * (1.0 - hidden_activations[i]);
        }

        // Update weights using gradient descent
        for i in 0..self.hidden_size {
            for j in 0..self.output_size {
                self.weights_hidden_output[i * self.output_size + j] -=
                    self.learning_rate * output_deltas[j] * hidden_activations[i];

                // Update biases for the output layer
                self.bias_output[j] -= self.learning_rate * output_deltas[0] * 0.1;
            }
        }

        println!("output errors: {:?}", output_deltas);

        for i in 0..self.input_size {
            for j in 0..self.hidden_size {
                self.weights_input_hidden[i * self.hidden_size + j] -=
                    self.learning_rate * hidden_errors[j] * inputs[i];
            }
            // Update biases for the hidden layer
            self.bias_hidden[i] -= self.learning_rate * hidden_errors[i];
        }
    }


    // Train the neural network using backpropagation
    pub fn train(&mut self, inputs: &[Vec<f32>], targets: &[Vec<f32>], epochs: usize) {
        assert_eq!(inputs.len(), targets.len(), "Input and target dimensions mismatch");

        for _ in 0..epochs {
            for i in 0..inputs.len() {
                self.backpropagate(&inputs[i], &targets[i]);
            }
        }
    }
}

fn main() {
    // Example training data (input and target)
    let inputs = vec![
        vec![0.1, 0.2],
        vec![0.3, 0.4],
        vec![0.5, 0.6],
        vec![0.7, 0.8],
    ];
    let targets = vec![
        vec![0.5],
        vec![0.8],
        vec![0.2],
        vec![0.6],
    ];

    // Initialize neural network with default matrix multiplication
    let default_multiplier = DefaultMatrixMultiplier;
    let mut neural_network = NeuralNetwork::new(2, 4, 1, 0.1, default_multiplier);

    // Train the neural network
    neural_network.train(&inputs, &targets, 100);

    // Test the trained neural network
    let test_input = vec![0.9, 1.0];
    let predicted_output = neural_network.forward(&test_input);

    println!("Predicted Output: {:?}", predicted_output);
}

