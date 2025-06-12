impl < __Context > :: bincode :: Decode < __Context > for SensoryInput
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        let variant_index = < u32 as :: bincode :: Decode ::< __D :: Context
        >>:: decode(decoder) ?; match variant_index
        {
            0u32 =>core :: result :: Result ::
            Ok(Self ::Text
            {
                0 : :: bincode :: Decode ::< __D :: Context >::
                decode(decoder) ?,
            }), 1u32 =>core :: result :: Result ::
            Ok(Self ::Vision
            {
                format : :: bincode :: Decode ::< __D :: Context >::
                decode(decoder) ?, resolution : :: bincode :: Decode ::< __D
                :: Context >:: decode(decoder) ?, data : :: bincode :: Decode
                ::< __D :: Context >:: decode(decoder) ?,
            }), 2u32 =>core :: result :: Result ::
            Ok(Self ::Audio
            {
                format : :: bincode :: Decode ::< __D :: Context >::
                decode(decoder) ?, sample_rate : :: bincode :: Decode ::< __D
                :: Context >:: decode(decoder) ?, data : :: bincode :: Decode
                ::< __D :: Context >:: decode(decoder) ?,
            }), 3u32 =>core :: result :: Result ::
            Ok(Self ::Telemetry
            {
                source_device : :: bincode :: Decode ::< __D :: Context >::
                decode(decoder) ?, data : :: bincode :: Decode ::< __D ::
                Context >:: decode(decoder) ?,
            }), variant =>core :: result :: Result ::
            Err(:: bincode :: error :: DecodeError :: UnexpectedVariant
            {
                found : variant, type_name : "SensoryInput", allowed : &::
                bincode :: error :: AllowedEnumVariants :: Range
                { min: 0, max: 3 }
            })
        }
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for SensoryInput
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        let variant_index = < u32 as :: bincode :: Decode ::< __D :: Context
        >>:: decode(decoder) ?; match variant_index
        {
            0u32 =>core :: result :: Result ::
            Ok(Self ::Text
            {
                0 : :: bincode :: BorrowDecode ::< __D :: Context >::
                borrow_decode(decoder) ?,
            }), 1u32 =>core :: result :: Result ::
            Ok(Self ::Vision
            {
                format : :: bincode :: BorrowDecode ::< __D :: Context >::
                borrow_decode(decoder) ?, resolution : :: bincode ::
                BorrowDecode ::< __D :: Context >:: borrow_decode(decoder) ?,
                data : :: bincode :: BorrowDecode ::< __D :: Context >::
                borrow_decode(decoder) ?,
            }), 2u32 =>core :: result :: Result ::
            Ok(Self ::Audio
            {
                format : :: bincode :: BorrowDecode ::< __D :: Context >::
                borrow_decode(decoder) ?, sample_rate : :: bincode ::
                BorrowDecode ::< __D :: Context >:: borrow_decode(decoder) ?,
                data : :: bincode :: BorrowDecode ::< __D :: Context >::
                borrow_decode(decoder) ?,
            }), 3u32 =>core :: result :: Result ::
            Ok(Self ::Telemetry
            {
                source_device : :: bincode :: BorrowDecode ::< __D :: Context
                >:: borrow_decode(decoder) ?, data : :: bincode ::
                BorrowDecode ::< __D :: Context >:: borrow_decode(decoder) ?,
            }), variant =>core :: result :: Result ::
            Err(:: bincode :: error :: DecodeError :: UnexpectedVariant
            {
                found : variant, type_name : "SensoryInput", allowed : &::
                bincode :: error :: AllowedEnumVariants :: Range
                { min: 0, max: 3 }
            })
        }
    }
}