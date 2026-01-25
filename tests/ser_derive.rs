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

    let mut rbb = wbb.into();

    let deserialized = Basic::from_buf(&mut rbb).expect("Error deserializing data");

    assert_eq!(b, deserialized);

    let f1_size = b.f1.approx_size();
    let f2_size = b.f2.approx_size();
    let f3_size = b.f3.approx_size();

    let total_size = f1_size + f2_size + f3_size;

    assert_eq!(total_size, b.approx_size());
}

// fn approx_size(&self) -> usize {
//     match self {
//         Self::V2 {
//             f1,
//             f2,
//             f3,
//         } =>   {1u8.approx_size () + f1.approx_size () + f2.approx_size () + f3. approx_size ()} # ( approx_size_body_tokens   ),   +
//     }
// }

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

    let mut wbb = WriteByteBuffer::new();

    b1.to_buf(&mut wbb);
    b2.to_buf(&mut wbb);
    b3.to_buf(&mut wbb);

    let mut rbb = wbb.into();

    let b1_deserialized = Basic::from_buf(&mut rbb).expect("Error deserializing b1");
    let b2_deserialized = Basic::from_buf(&mut rbb).expect("Error deserializing b1");
    let b3_deserialized = Basic::from_buf(&mut rbb).expect("Error deserializing b1");

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
