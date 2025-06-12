impl :: bincode :: Encode for DSN
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.id, encoder) ?; :: bincode ::
        Encode :: encode(&self.topical_bnns, encoder) ?; :: bincode :: Encode
        :: encode(&self.config, encoder) ?; core :: result :: Result :: Ok(())
    }
}