impl < __Context > :: bincode :: Decode < __Context > for SpikeEvent
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            presynaptic_neuron_id : :: bincode :: Decode :: decode(decoder) ?,
            timestamp : :: bincode :: Decode :: decode(decoder) ?, value : ::
            bincode :: Decode :: decode(decoder) ?,
        })
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for SpikeEvent
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            presynaptic_neuron_id : :: bincode :: BorrowDecode ::< '_,
            __Context >:: borrow_decode(decoder) ?, timestamp : :: bincode ::
            BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?, value
            : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?,
        })
    }
}