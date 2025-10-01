use serbytes::prelude::*;

#[test]
fn test_enum_derive() {
    #[derive(SerBytes, Debug, Eq, PartialEq)]
    struct Basic {
        f1: String,
        f2: u8,
        f4: u32,
    }

    let b = Basic {
        f1: "hello".into(),
        f2: 124,
        f4: 67,
    };

    let mut bb = b.to_bb();

    let deserialized = Basic::from_buf(&mut bb).expect("Error deserializing data");

    assert_eq!(b, deserialized);
}
