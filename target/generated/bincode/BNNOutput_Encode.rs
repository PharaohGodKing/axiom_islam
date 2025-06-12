impl :: bincode :: Encode for BNNOutput
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.bnn_id, encoder) ?; :: bincode ::
        Encode :: encode(&self.anomaly_ratio, encoder) ?; :: bincode :: Encode
        :: encode(&self.pattern_ratio, encoder) ?; core :: result :: Result ::
        Ok(())
    }
}