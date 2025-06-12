impl < __Context > :: bincode :: Decode < __Context > for CorePhilosophy
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            name : :: bincode :: Decode :: decode(decoder) ?, definition : ::
            bincode :: Decode :: decode(decoder) ?, framework_integration : ::
            bincode :: Decode :: decode(decoder) ?, hint_for_axiom : ::
            bincode :: Decode :: decode(decoder) ?,
        })
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for CorePhilosophy
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            name : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, definition : :: bincode :: BorrowDecode
            ::< '_, __Context >:: borrow_decode(decoder) ?,
            framework_integration : :: bincode :: BorrowDecode ::< '_,
            __Context >:: borrow_decode(decoder) ?, hint_for_axiom : ::
            bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?,
        })
    }
}