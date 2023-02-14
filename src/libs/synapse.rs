static mut _GLOBAL_ID: usize = 0;

pub struct Node {
    value: f64,
    id: usize,
}

impl Node {
    pub fn new() -> Self {
        Node { 
            value: 0.0, 
            id: get_new_id() 
        }
    }

    pub fn update(&mut self, i: &f64) {
        self.value = *i;
    }
}

/// A neuron, which takes the Vector of weighted input values and returns one output value.
/// 
/// Input values should be between -1 and +1, and input weights should be between -4 and +4.
///
/// ## Creating a neuron object.
/// Construct a neuron using the following syntax:
/// ```
/// let neuron_0 = Neuron::new()
/// ```
/// The neuron will automatically be given a unique id that can be accessed via the `.id()` getter to differentiate it from other neurons.
///
/// ## Updating the values in the neuron.
/// As long as the update method is called in a game loop via a method like this:
///
/// ```
/// // An example simple game loop.
/// fn main() {
///     loop {
///         update();
///     }
/// }
/// // The main update function, where all objects are updated.
/// fn update() {
///     in_0.update();
///     n_0.update();
///     n_1.update();
/// }
/// ```
pub struct Neuron {
    value: f64,
    id: usize,
    in_vals: Vec<f64>,
    in_weights: Vec<f64>,
    in_ids: Vec<usize>,
}

impl Neuron {
    pub fn new() -> Self {
        Neuron {
            value: 0.0,
            id: get_new_id(),
            in_vals: vec![],
            in_weights: vec![],
            in_ids: vec![],
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn add_input(&mut self, input: &f64, weight: &f64, id: &usize) {
        self.in_ids.push(*id);
        self.in_vals.push(*input);
        self.in_weights.push(*weight);
    }

    ///Runs every tick; computes the hyperbolic tangent of the dot product of the input values and input weights, returning a f64 between -1 and +1.
    pub fn update(&mut self) {
        self.value = dot(&self.in_vals, &self.in_weights).tanh();
    }

    pub fn ping(&self) {
        let a: &str = "Pinged neuron with id ";
        let b: &str = &self.id.to_string();
        let c: &str = ", a value of ";
        let d: &str = &self.value.to_string();
        let e: &str = ", input values of ";
        let f: &str = &vec_to_str(&self.in_vals);
        let g: &str = ", and input weights of ";
        let h: &str = &vec_to_str(&self.in_vals);
        let i = [a, b, c, d, e, f, g, h].join("");
        print!("{}", i);
        println!(".")
    }
}

fn get_new_id() -> usize {
    unsafe {
        _GLOBAL_ID += 1;
        _GLOBAL_ID - 1
    }
}

/// Calculate the dot product of two vectors.
pub fn dot(a: &[f64], b: &[f64]) -> f64 {
    assert_eq!(a.len(), b.len());
    let mut product: f64 = 0.0;
    for i in 0..a.len() {
        product += a[i] * b[i];
    }
    product
}

fn vec_to_str(v: &Vec<f64>) -> String {
    let mut strings: Vec<String> = vec![];
    for i in 0..v.len() {
        strings.push(v[i].to_string());
    }
    let a = "{";
    let b = "}";
    let c = if strings.is_empty() {"".to_owned()} else {strings.join(", ")};
    [a, &c, b].join("")
}