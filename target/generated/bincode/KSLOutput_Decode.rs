impl < __Context > :: bincode :: Decode < __Context > for KSLOutput
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            ksl_id : :: bincode :: Decode :: decode(decoder) ?, ksl_name : ::
            bincode :: Decode :: decode(decoder) ?, conclusion : :: bincode ::
            Decode :: decode(decoder) ?, contributing_insights : :: bincode ::
            Decode :: decode(decoder) ?,
        })
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for KSLOutput
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            ksl_id : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, ksl_name : :: bincode :: BorrowDecode
            ::< '_, __Context >:: borrow_decode(decoder) ?, conclusion : ::
            bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, contributing_insights : :: bincode ::
            BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?,
        })
    }
}