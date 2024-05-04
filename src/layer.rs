use crate::neuron::Neuron;

enum LayerType
{
    DENSE ,
    INPUT ,
    OUTPUT,
}
struct Layer
{
    neurons:    Vec<Neuron>,
    layer_type: LayerType  ,
}