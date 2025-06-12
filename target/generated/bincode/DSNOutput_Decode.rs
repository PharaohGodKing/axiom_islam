impl < __Context > :: bincode :: Decode < __Context > for DSNOutput
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            dsn_id : :: bincode :: Decode :: decode(decoder) ?, insight : ::
            bincode :: Decode :: decode(decoder) ?, supporting_bnns : ::
            bincode :: Decode :: decode(decoder) ?,
        })
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for DSNOutput
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            dsn_id : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, insight : :: bincode :: BorrowDecode ::<
            '_, __Context >:: borrow_decode(decoder) ?, supporting_bnns : ::
            bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?,
        })
    }
}