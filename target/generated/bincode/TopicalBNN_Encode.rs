impl :: bincode :: Encode for TopicalBNN
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.id, encoder) ?; :: bincode ::
        Encode :: encode(&self.topic, encoder) ?; :: bincode :: Encode ::
        encode(&self.neurons, encoder) ?; :: bincode :: Encode ::
        encode(&self.config, encoder) ?; :: bincode :: Encode ::
        encode(&self.topic_keywords, encoder) ?; core :: result :: Result ::
        Ok(())
    }
}