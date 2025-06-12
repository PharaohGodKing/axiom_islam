impl :: bincode :: Encode for KeyProject
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.name, encoder) ?; :: bincode ::
        Encode :: encode(&self.nature, encoder) ?; :: bincode :: Encode ::
        encode(&self.core_purpose, encoder) ?; :: bincode :: Encode ::
        encode(&self.mission_alignment, encoder) ?; :: bincode :: Encode ::
        encode(&self.operational_link, encoder) ?; core :: result :: Result ::
        Ok(())
    }
}