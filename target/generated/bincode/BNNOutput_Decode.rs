impl < __Context > :: bincode :: Decode < __Context > for BNNOutput
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            bnn_id : :: bincode :: Decode :: decode(decoder) ?, anomaly_ratio
            : :: bincode :: Decode :: decode(decoder) ?, pattern_ratio : ::
            bincode :: Decode :: decode(decoder) ?,
        })
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for BNNOutput
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            bnn_id : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, anomaly_ratio : :: bincode ::
            BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?,
            pattern_ratio : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?,
        })
    }
}