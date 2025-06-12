impl :: bincode :: Encode for SensoryInput
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        match self
        {
            Self ::Text(field_0)
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (0u32), encoder) ?
                ; :: bincode :: Encode :: encode(field_0, encoder) ?; core ::
                result :: Result :: Ok(())
            }, Self ::Vision { format, resolution, data }
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (1u32), encoder) ?
                ; :: bincode :: Encode :: encode(format, encoder) ?; ::
                bincode :: Encode :: encode(resolution, encoder) ?; :: bincode
                :: Encode :: encode(data, encoder) ?; core :: result :: Result
                :: Ok(())
            }, Self ::Audio { format, sample_rate, data }
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (2u32), encoder) ?
                ; :: bincode :: Encode :: encode(format, encoder) ?; ::
                bincode :: Encode :: encode(sample_rate, encoder) ?; ::
                bincode :: Encode :: encode(data, encoder) ?; core :: result
                :: Result :: Ok(())
            }, Self ::Telemetry { source_device, data }
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (3u32), encoder) ?
                ; :: bincode :: Encode :: encode(source_device, encoder) ?; ::
                bincode :: Encode :: encode(data, encoder) ?; core :: result
                :: Result :: Ok(())
            },
        }
    }
}