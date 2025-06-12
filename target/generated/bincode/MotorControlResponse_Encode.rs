impl :: bincode :: Encode for MotorControlResponse
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.status, encoder) ?; :: bincode ::
        Encode :: encode(&self.feedback, encoder) ?; core :: result :: Result
        :: Ok(())
    }
}