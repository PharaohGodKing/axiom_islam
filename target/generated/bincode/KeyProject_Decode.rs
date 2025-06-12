impl < __Context > :: bincode :: Decode < __Context > for KeyProject
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            name : :: bincode :: Decode :: decode(decoder) ?, nature : ::
            bincode :: Decode :: decode(decoder) ?, core_purpose : :: bincode
            :: Decode :: decode(decoder) ?, mission_alignment : :: bincode ::
            Decode :: decode(decoder) ?, operational_link : :: bincode ::
            Decode :: decode(decoder) ?,
        })
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for KeyProject
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            name : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, nature : :: bincode :: BorrowDecode ::<
            '_, __Context >:: borrow_decode(decoder) ?, core_purpose : ::
            bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, mission_alignment : :: bincode ::
            BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?,
            operational_link : :: bincode :: BorrowDecode ::< '_, __Context
            >:: borrow_decode(decoder) ?,
        })
    }
}