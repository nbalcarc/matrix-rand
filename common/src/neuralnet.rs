

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


    /// Forward propagation with the given input
    pub fn forward(&mut self, input: &[f32]) -> Vec<f32> {

        // first hidden layer
        let mut layer0 = Vec::with_capacity(self.sizes.1);
        self.multiplier.multiply(&input, &self.weights0, (1, self.sizes.0, self.sizes.1), &mut layer0); //matrix mul
        for i in 0..layer0.len() { //apply bias
            layer0[i] += self.bias0[i];
        }
        
        // second hidden layer
        let mut layer1 = Vec::with_capacity(self.sizes.2);
        self.multiplier.multiply(&layer0, &self.weights1, (1, self.sizes.1, self.sizes.2), &mut layer1); //matrix mul
        for i in 0..layer1.len() { //apply bias
            layer1[i] += self.bias1[i];
        }

        // output layer
        let mut output = Vec::with_capacity(self.sizes.3);
        self.multiplier.multiply(&layer1, &self.weights2, (1, self.sizes.2, self.sizes.3), &mut output); //matrix mul
        for i in 0..output.len() { //apply bias
            output[i] += self.bias2[i];
        }

        output
    }


    /// Backward propagation with the given information
    pub fn backward(&mut self) {

    }
}


fn test_func(a: &[f32], b: &[f32], sizes: (usize, usize, usize), c: &mut [f32], d: i32) {

}

fn test1() {
    //let x = NeuralNetwork::new(test_func);
    //let y = test;
}




