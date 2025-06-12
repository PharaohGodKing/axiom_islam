impl < __Context > :: bincode :: Decode < __Context > for LIFNeuron
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            id : :: bincode :: Decode :: decode(decoder) ?, membrane_potential
            : :: bincode :: Decode :: decode(decoder) ?, threshold : ::
            bincode :: Decode :: decode(decoder) ?, leak_rate : :: bincode ::
            Decode :: decode(decoder) ?, reset_potential : :: bincode ::
            Decode :: decode(decoder) ?, refractory_countdown : :: bincode ::
            Decode :: decode(decoder) ?, refractory_cycles : :: bincode ::
            Decode :: decode(decoder) ?, connections : :: bincode :: Decode ::
            decode(decoder) ?, last_spike_time : :: bincode :: Decode ::
            decode(decoder) ?, learning_rate : :: bincode :: Decode ::
            decode(decoder) ?,
        })
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for LIFNeuron
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            id : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, membrane_potential : :: bincode ::
            BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?,
            threshold : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, leak_rate : :: bincode :: BorrowDecode
            ::< '_, __Context >:: borrow_decode(decoder) ?, reset_potential :
            :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, refractory_countdown : :: bincode ::
            BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?,
            refractory_cycles : :: bincode :: BorrowDecode ::< '_, __Context
            >:: borrow_decode(decoder) ?, connections : :: bincode ::
            BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?,
            last_spike_time : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, learning_rate : :: bincode ::
            BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?,
        })
    }
}