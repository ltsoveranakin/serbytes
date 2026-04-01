use serbytes::prelude::*;

#[test]
fn test_struct_derive() {
    #[derive(SerBytes, Debug, Eq, PartialEq)]
    struct Basic {
        f1: String,
        f2: u8,
        f3: u32,
    }

    let b = Basic {
        f1: "hello world".into(),
        f2: 124,
        f3: 67,
    };

    let wbb = b.to_bb();

    let mut rbb: ReadByteBufferOwned = wbb.into();

    let mut rbb_ref = rbb.rbb_ref_mut();

    let deserialized = Basic::from_buf(&mut rbb_ref).expect("Error deserializing data");

    assert_eq!(b, deserialized);

    let f1_size = b.f1.approx_size();
    let f2_size = b.f2.approx_size();
    let f3_size = b.f3.approx_size();

    let total_size = f1_size + f2_size + f3_size;

    assert_eq!(total_size, b.approx_size());
}

#[test]
fn test_enum_derive() {
    #[derive(SerBytes, Debug, Eq, PartialEq)]
    enum Basic {
        V1,
        V2 { f1: String, f2: u8, f3: u32 },
        V3(u16, String, i8),
    }

    let b1 = Basic::V1;
    let b2 = Basic::V2 {
        f1: "hello world".into(),
        f2: 124,
        f3: 67,
    };
    let b3 = Basic::V3(7482, "hello world".to_string(), 23);

    let mut wbb = WriteByteBufferOwned::new();

    b1.to_buf(&mut wbb);
    b2.to_buf(&mut wbb);
    b3.to_buf(&mut wbb);

    let mut rbb: ReadByteBufferOwned = wbb.into();

    let mut rbb_ref = rbb.rbb_ref_mut();

    let b1_deserialized = Basic::from_buf(&mut rbb_ref).expect("Error deserializing b1");
    let b2_deserialized = Basic::from_buf(&mut rbb_ref).expect("Error deserializing b1");
    let b3_deserialized = Basic::from_buf(&mut rbb_ref).expect("Error deserializing b1");

    assert_eq!(b1, b1_deserialized);
    assert_eq!(b2, b2_deserialized);
    assert_eq!(b3, b3_deserialized);

    let b1_size = b1.approx_size();
    let b2_size = b2.approx_size();
    let b3_size = b3.approx_size();

    let total_b1_size = match b1 {
        Basic::V1 => 0,

        _ => unreachable!(),
    };

    let total_b2_size = match b2 {
        Basic::V2 { f1, f2, f3 } => {
            u8::size_hint() + f1.approx_size() + f2.approx_size() + f3.approx_size()
        }

        _ => unreachable!(),
    };

    let total_b3_size = match b3 {
        Basic::V3(f1, f2, f3) => f1.approx_size() + f2.approx_size() + f3.approx_size(),

        _ => unreachable!(),
    };

    assert_eq!(b1_size, total_b1_size);
    assert_eq!(b2_size, total_b2_size);
    assert_eq!(b3_size, total_b3_size);
}

#[test]
fn test_generic_struct() {
    #[derive(SerBytes, Debug, Eq, PartialEq)]
    struct GenStruct<T> {
        name: String,
        data: T,
        end: i32,
    }

    type NumberedGen = GenStruct<u32>;
    type StringedGen = GenStruct<String>;

    let numbered = NumberedGen {
        name: "numbered name".to_string(),
        data: 183,
        end: -857,
    };

    let stringed = StringedGen {
        name: "stringed name".to_string(),
        data: "the data :3".to_string(),
        end: -857,
    };

    let numbered_2 = NumberedGen::from_buf(
        &mut ReadByteBufferOwned::from_vec(numbered.to_bb().into_vec()).rbb_ref_mut(),
    )
    .expect("Deserialize numbered generic");
    let stringed_2 = StringedGen::from_buf(
        &mut ReadByteBufferOwned::from_vec(stringed.to_bb().into_vec()).rbb_ref_mut(),
    )
    .expect("Deserialize stringed generic");

    assert_eq!(numbered, numbered_2);

    assert_eq!(stringed, stringed_2);
}

#[test]
fn test_generic_enum() {
    #[derive(SerBytes, Debug, Eq, PartialEq)]
    enum GenEnum<T, B> {
        Empty,
        Some(T),
        Other(B),
    }

    type NumberedGen = GenEnum<u32, i8>;
    type StringedGen = GenEnum<String, u8>;

    let numbered_empty = NumberedGen::Empty;
    let numbered_first = NumberedGen::Some(3829);
    let numbered_second = NumberedGen::Other(23);

    let stringed_empty = StringedGen::Empty;
    let stringed_first = StringedGen::Some("String1".to_string());
    let stringed_second = StringedGen::Other(72);

    let mut numbered_buf = WriteByteBufferOwned::new();

    numbered_empty.to_buf(&mut numbered_buf);
    numbered_first.to_buf(&mut numbered_buf);
    numbered_second.to_buf(&mut numbered_buf);

    let mut numbered_rbb = ReadByteBufferOwned::from_vec(numbered_buf.into_vec());

    let deserialized_numbered_empty = NumberedGen::from_buf(&mut numbered_rbb.rbb_ref_mut())
        .expect("Deserialize empty numbered generic");
    let deserialized_numbered_first = NumberedGen::from_buf(&mut numbered_rbb.rbb_ref_mut())
        .expect("Deserialize first numbered generic");
    let deserialized_numbered_second = NumberedGen::from_buf(&mut numbered_rbb.rbb_ref_mut())
        .expect("Deserialize second numbered generic");

    let mut stringed_buf = WriteByteBufferOwned::new();

    stringed_empty.to_buf(&mut stringed_buf);
    stringed_first.to_buf(&mut stringed_buf);
    stringed_second.to_buf(&mut stringed_buf);

    let mut stringed_rbb = ReadByteBufferOwned::from_vec(stringed_buf.into_vec());

    let deserialized_stringed_empty = StringedGen::from_buf(&mut stringed_rbb.rbb_ref_mut())
        .expect("Deserialize empty stringed generic");
    let deserialized_stringed_first = StringedGen::from_buf(&mut stringed_rbb.rbb_ref_mut())
        .expect("Deserialize first stringed generic");
    let deserialized_stringed_second = StringedGen::from_buf(&mut stringed_rbb.rbb_ref_mut())
        .expect("Deserialize second stringed generic");

    assert_eq!(numbered_empty, deserialized_numbered_empty);
    assert_eq!(numbered_first, deserialized_numbered_first);
    assert_eq!(numbered_second, deserialized_numbered_second);

    assert_eq!(stringed_empty, deserialized_stringed_empty);
    assert_eq!(stringed_first, deserialized_stringed_first);
    assert_eq!(stringed_second, deserialized_stringed_second);
}
