impl :: bincode :: Encode for KSLOutput
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.ksl_id, encoder) ?; :: bincode ::
        Encode :: encode(&self.ksl_name, encoder) ?; :: bincode :: Encode ::
        encode(&self.conclusion, encoder) ?; :: bincode :: Encode ::
        encode(&self.contributing_insights, encoder) ?; core :: result ::
        Result :: Ok(())
    }
}