impl < __Context > :: bincode :: Decode < __Context > for TopicalBNN
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            id : :: bincode :: Decode :: decode(decoder) ?, topic : :: bincode
            :: Decode :: decode(decoder) ?, neurons : :: bincode :: Decode ::
            decode(decoder) ?, config : :: bincode :: Decode ::
            decode(decoder) ?, topic_keywords : :: bincode :: Decode ::
            decode(decoder) ?,
        })
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for TopicalBNN
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            id : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, topic : :: bincode :: BorrowDecode ::<
            '_, __Context >:: borrow_decode(decoder) ?, neurons : :: bincode
            :: BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?,
            config : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, topic_keywords : :: bincode ::
            BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?,
        })
    }
}