impl :: bincode :: Encode for CorePhilosophy
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.name, encoder) ?; :: bincode ::
        Encode :: encode(&self.definition, encoder) ?; :: bincode :: Encode ::
        encode(&self.framework_integration, encoder) ?; :: bincode :: Encode
        :: encode(&self.hint_for_axiom, encoder) ?; core :: result :: Result
        :: Ok(())
    }
}