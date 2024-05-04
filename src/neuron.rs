use crate::buffer_manager::Buffer;

pub struct Neuron
{
    pub weights      :        Vec<f32>,
    pub activation_fn:        fn( &Vec<f32> , Vec<u8> ) -> u8,
    pub pos_in_layer :        usize,
    pub write_mask   :        Vec<bool>
}

impl Neuron 
{

    pub fn new( 
                weights      :  Vec<f32>                    , 
                activation_fn:  fn(&Vec<f32>, Vec<u8>) -> u8, // user can define a custom activation function
                pos_in_layer :  usize                       ,
                write_mask   :  Vec<bool>                   , // TRUE is write, FALSE if not to write, decided by layer
               )             -> Neuron
    {
        Neuron
        {
            weights        ,
            activation_fn  ,
            pos_in_layer   ,
            write_mask     , 
        }
    }

    pub fn invoke(&self, buffer: &mut Buffer)
    {
        let out_data = (self.activation_fn)(&self.weights, self.read_buffer(buffer));
        self.write_buffer(buffer, out_data)

    }

    fn read_buffer(&self, buffer: &Buffer) -> Vec<u8>
    {
        return buffer.get(self.pos_in_layer).unwrap().clone();
    }

    fn write_buffer(&self, buffer: &mut Buffer, write_data: u8)
    {
        let counter = 0;
        for data_vec in buffer.iter_mut()
        {
            if  *self.write_mask.get(counter).unwrap()  // Mask set by layer, if layer says write, do so, decided AOT (should be safe, so just unwrap)
            {
                data_vec.push(write_data);
            }
            
        } 
    }
}

#[cfg(test)]
mod tests
{

}