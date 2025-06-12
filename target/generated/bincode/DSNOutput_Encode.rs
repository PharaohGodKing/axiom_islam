impl :: bincode :: Encode for DSNOutput
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.dsn_id, encoder) ?; :: bincode ::
        Encode :: encode(&self.insight, encoder) ?; :: bincode :: Encode ::
        encode(&self.supporting_bnns, encoder) ?; core :: result :: Result ::
        Ok(())
    }
}