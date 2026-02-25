use serbytes::prelude::*;

#[test]
fn test_may_not_exist() {
    const CUSTOM_I32: i32 = 4578;

    #[derive(Debug, Eq, PartialEq)]
    struct CustomDataProvider;

    impl MayNotExistDataProvider<i32> for CustomDataProvider {
        fn get_data() -> i32 {
            CUSTOM_I32
        }
    }

    #[derive(SerBytes, Debug, Eq, PartialEq)]
    struct FieldsMayNotExist {
        f1: u32,
        f2: MayNotExistOrDefault<u32>,
        f3: MayNotExistOrElse<i32, CustomDataProvider>,
    }

    let mut buf = WriteByteBufferOwned::new();

    let initial_value = 10u32;

    initial_value.to_buf(&mut buf);

    let mut rbb = ReadByteBufferOwned::from_vec(buf.into_vec());

    let fields_defaulted =
        FieldsMayNotExist::from_buf(&mut rbb.rbb_ref_mut()).expect("Read data from bytebuffer");

    assert_eq!(
        fields_defaulted,
        FieldsMayNotExist {
            f1: initial_value,
            f2: u32::default().into(),
            f3: CUSTOM_I32.into(),
        }
    )
}
