impl :: bincode :: Encode for LIFNeuron
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.id, encoder) ?; :: bincode ::
        Encode :: encode(&self.membrane_potential, encoder) ?; :: bincode ::
        Encode :: encode(&self.threshold, encoder) ?; :: bincode :: Encode ::
        encode(&self.leak_rate, encoder) ?; :: bincode :: Encode ::
        encode(&self.reset_potential, encoder) ?; :: bincode :: Encode ::
        encode(&self.refractory_countdown, encoder) ?; :: bincode :: Encode ::
        encode(&self.refractory_cycles, encoder) ?; :: bincode :: Encode ::
        encode(&self.connections, encoder) ?; :: bincode :: Encode ::
        encode(&self.last_spike_time, encoder) ?; :: bincode :: Encode ::
        encode(&self.learning_rate, encoder) ?; core :: result :: Result ::
        Ok(())
    }
}