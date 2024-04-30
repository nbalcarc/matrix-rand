use num_traits::Float;



pub trait Multiplier {
    fn multiply(&mut self, a: &[f32], b: &[f32], sizes: (usize, usize, usize), c: &mut [f32]) -> ();
}


/// A two-hidden-layer neural network with customizable layer sizes.
pub struct NeuralNetwork<T: Multiplier> {
    sizes: (usize, usize, usize, usize),
    multiplier: T,

    // hidden
    weights0: Vec<f32>,
    bias0: Vec<f32>,

    // hidden
    weights1: Vec<f32>,
    bias1: Vec<f32>,

    // output
    weights2: Vec<f32>,
    bias2: Vec<f32>,
}
impl<T: Multiplier> NeuralNetwork<T> {
    pub fn new(multiplier: T, sizes: (usize, usize, usize, usize)) -> Self {
        NeuralNetwork {
            sizes,
            multiplier,
            
            weights0: (0..sizes.0*sizes.1).into_iter().map(|_| rand::random()).collect(),
            bias0: (0..sizes.1).into_iter().map(|_| rand::random()).collect(),

            weights1: (0..sizes.1*sizes.2).into_iter().map(|_| rand::random()).collect(),
            bias1: (0..sizes.2).into_iter().map(|_| rand::random()).collect(),

            weights2: (0..sizes.2*sizes.3).into_iter().map(|_| rand::random()).collect(),
            bias2: (0..sizes.3).into_iter().map(|_| rand::random()).collect(),
        }
    }


    /// Transpose a matrix
    pub fn transpose(&self, input: &[f32], sizes: (usize, usize)) -> Vec<f32> {
        let mut ret = vec![0.0; input.len()];

        for n in 0..sizes.0 { //input tall, return wide
            for m in 0..sizes.1 { //input wide, return tall
                ret[n + (m * sizes.0)] = input[m + (n * sizes.1)]; //place into correct new index
            }
        }

        ret
    }


    /// Forward propagation with the given input
    pub fn forward(&mut self, input: &[f32]) -> Vec<f32> {

        // first hidden layer
        let mut layer0 = Vec::with_capacity(self.sizes.1);
        self.multiplier.multiply(&input, &self.weights0, (1, self.sizes.0, self.sizes.1), &mut layer0); //matrix mul
        for i in 0..layer0.len() { //apply bias + nonlinearity
            layer0[i] = (layer0[i] + self.bias0[i]).tanh();
        }
        
        // second hidden layer
        let mut layer1 = Vec::with_capacity(self.sizes.2);
        self.multiplier.multiply(&layer0, &self.weights1, (1, self.sizes.1, self.sizes.2), &mut layer1); //matrix mul
        for i in 0..layer1.len() { //apply bias + nonlinearity
            layer1[i] = (layer1[i] + self.bias1[i]).tanh();
        }

        // output layer
        let mut output = Vec::with_capacity(self.sizes.3);
        self.multiplier.multiply(&layer1, &self.weights2, (1, self.sizes.2, self.sizes.3), &mut output); //matrix mul
        for i in 0..output.len() { //apply bias + nonlinearity
            output[i] = (output[i] + self.bias2[i]).tanh();
        }

        output
    }


    /// Backward propagation with the given information
    pub fn backward(&mut self, input: &[f32], expected: &[f32]) -> Vec<f32> {

        // initialize vectors
        let mut grads_w0 = vec![0.0; self.weights0.len()];
        let mut grads_w1 = vec![0.0; self.weights1.len()];
        let mut grads_w2 = vec![0.0; self.weights2.len()];

        let mut grads_b0 = vec![0.0; self.bias0.len()];
        let mut grads_b1 = vec![0.0; self.bias1.len()];
        let mut grads_b2 = vec![0.0; self.bias2.len()];

        // forward first hidden layer
        let mut layer0_z = Vec::with_capacity(self.sizes.1);
        let mut layer0 = Vec::with_capacity(self.sizes.1);
        self.multiplier.multiply(&input, &self.weights0, (1, self.sizes.0, self.sizes.1), &mut layer0); //matrix mul
        for i in 0..layer0.len() { //apply bias + nonlinearity
            layer0_z[i] = layer0[i] + self.bias0[i];
            layer0[i] = (layer0[i] + self.bias0[i]).tanh();
        }
        
        // forward second hidden layer
        let mut layer1_z = Vec::with_capacity(self.sizes.2);
        let mut layer1 = Vec::with_capacity(self.sizes.2);
        self.multiplier.multiply(&layer0, &self.weights1, (1, self.sizes.1, self.sizes.2), &mut layer1); //matrix mul
        for i in 0..layer1.len() { //apply bias + nonlinearity
            layer1_z[i] = layer1[i] + self.bias1[i];
            layer1[i] = (layer1[i] + self.bias1[i]).tanh();
        }

        // forward output layer
        let mut output_z = Vec::with_capacity(self.sizes.3);
        let mut output = Vec::with_capacity(self.sizes.3);
        self.multiplier.multiply(&layer1, &self.weights2, (1, self.sizes.2, self.sizes.3), &mut output); //matrix mul
        for i in 0..output.len() { //apply bias + nonlinearity
            output_z[i] = output[i] + self.bias2[i];
            output[i] = (output[i] + self.bias2[i]).tanh();
        }

        // delta output layer
        let mut delta2 = vec![0.0; self.sizes.3];
        for i in 0..delta2.len() {
            delta2[i] = (output[i] - expected[i]) * (1.0 - output_z[i].tanh().powi(2)); //take difference and mul by invert tanh of z
        }

        // delta second hidden layer
        let mut delta1 = vec![0.0; self.sizes.2];
        let weights2_transposed = self.transpose(&self.weights2, (self.sizes.2, self.sizes.3));
        self.multiplier.multiply(&weights2_transposed, &delta2, (self.sizes.3, self.sizes.2, 1), &mut delta1);
        for i in 0..delta1.len() {
            delta1[i] *= layer1_z[i];
        }

        // delta first hidden layer
        let mut delta0 = vec![0.0; self.sizes.1];
        let weights1_transposed = self.transpose(&self.weights1, (self.sizes.1, self.sizes.2));
        self.multiplier.multiply(&weights1_transposed, &delta1, (self.sizes.2, self.sizes.1, 1), &mut delta0);
        for i in 0..delta0.len() {
            delta0[i] *= layer0_z[i];
        }

        // calculate gradients


        todo!()
    }


    pub fn train(&mut self, batch: &[Vec<f32>], eta: f32) {
        
    }
}


fn test_func(a: &[f32], b: &[f32], sizes: (usize, usize, usize), c: &mut [f32], d: i32) {

}

fn test1() {
    //let x = NeuralNetwork::new(test_func);
    //let y = test;
}




