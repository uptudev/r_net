use libs::synapse::Neuron;

mod libs;

fn main() {
    let n_0 = Neuron::new();
    let n_1 = Neuron::new();
    n_0.ping();
    n_1.ping();
}
