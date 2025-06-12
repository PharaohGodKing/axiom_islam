impl :: bincode :: Encode for SpikeEvent
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.presynaptic_neuron_id, encoder)
        ?; :: bincode :: Encode :: encode(&self.timestamp, encoder) ?; ::
        bincode :: Encode :: encode(&self.value, encoder) ?; core :: result ::
        Result :: Ok(())
    }
}