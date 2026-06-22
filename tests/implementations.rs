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

#[test]
fn test_versioning() {
    #[derive(SerBytes, Debug, Clone)]
    struct UnversionedRawData1 {
        f1: u32,
        s: String,
    }

    #[derive(SerBytes, Debug, Eq, PartialEq)]
    struct UnversionedRawData2 {
        s1: String,
        s2: String,
        f1: u64,
    }

    #[derive(SerBytes)]
    enum DataVersionTransformerOld {
        V1,
    }

    impl CurrentVersion for DataVersionTransformerOld {
        type Output = UnversionedRawData1;

        fn get_data_from_buf(&self, buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self::Output> {
            match *self {
                Self::V1 => UnversionedRawData1::from_buf(buf),
            }
        }

        fn current_version() -> Self {
            Self::V1
        }
    }

    #[derive(SerBytes)]
    enum DataVersionTransformer {
        V1,
        V2,
    }

    fn transform_data1_to_data2(data: UnversionedRawData1) -> UnversionedRawData2 {
        UnversionedRawData2 {
            s1: data.s,
            s2: "new string!".to_string(),
            f1: data.f1 as u64,
        }
    }

    impl CurrentVersion for DataVersionTransformer {
        type Output = UnversionedRawData2;

        fn get_data_from_buf(&self, buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self::Output> {
            match *self {
                Self::V1 => {
                    let old_data = UnversionedRawData1::from_buf(buf)?;

                    let current_data = transform_data1_to_data2(old_data);

                    Ok(current_data)
                }

                Self::V2 => from_buf(buf),
            }
        }

        fn current_version() -> Self {
            Self::V2
        }
    }

    type VersionedData1 = VersioningWrapper<UnversionedRawData1, DataVersionTransformerOld>;
    type VersionedData2 = VersioningWrapper<UnversionedRawData2, DataVersionTransformer>;

    let data1 = UnversionedRawData1 {
        f1: 78,
        s: "aabbcdefg".to_string(),
    };

    let data1_versioned = VersionedData1::new(data1.clone());

    let wbb = data1_versioned.to_bb();

    // println!("{:?}", wbb.buf());

    let mut rbb = ReadByteBufferOwned::from_vec(wbb.into_vec());

    let data2_deserialized = VersionedData2::from_buf(&mut rbb.rbb_ref_mut())
        .expect("Read versioned data from buffer")
        .inner;

    let cmp_data2 = transform_data1_to_data2(data1);

    assert_eq!(
        data2_deserialized, cmp_data2,
        "comparing deserialized data2 to original data",
    );
}

#[test]
fn test_block() {
    let mut wbb = WriteByteBufferOwned::new();
    let block_data = "Hello".to_string();

    let block = SizedBlock::new(block_data);

    block.to_buf(&mut wbb);

    let mut rbb = ReadByteBufferOwned::from_vec(wbb.into_vec());
    let mut rbuf = rbb.rbb_ref_mut();

    let deser_block = SizedBlock::from_buf(&mut rbuf).unwrap();

    assert_eq!(block, deser_block);

    /// A type that writes more bytes than it reads
    #[derive(Debug)]
    struct BadType;

    impl SerBytes for BadType {
        fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
        where
            Self: Sized,
        {
            buf.read_bytes(10)?;

            Ok(Self)
        }

        fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
            buf.write_bytes(&[0; 5]);
        }
    }

    let mut wbb = WriteByteBufferOwned::new();
    let bt_block = SizedBlock::new(BadType);
    bt_block.to_buf(&mut wbb);

    let mut rbb = ReadByteBufferOwned::from_vec(wbb.into_vec());
    let mut rbuf = rbb.rbb_ref_mut();

    let e = SizedBlock::<BadType>::from_buf(&mut rbuf).unwrap_err();

    assert!(matches!(e.specific_error, SpecificError::Bytes { .. }));
}

#[test]
fn test_block_result() {
    #[derive(SerBytes, Eq, PartialEq)]
    struct Data1 {
        first: u32,
        block: SizedBlock<BBReadResult<u16>>,
        end: String,
    }

    #[derive(SerBytes, Eq, PartialEq)]
    struct Data2 {
        first: u32,
        block: SizedBlock<BBReadResult<u32>>,
        end: String,
    }

    let d1 = Data1 {
        first: 124,
        block: Ok(8387).into(),
        end: "End data block".into(),
    };

    let mut wbb = WriteByteBufferOwned::new();

    d1.to_buf(&mut wbb);

    let mut rbb = ReadByteBufferOwned::from_vec(wbb);

    let d2 = Data2::from_buf(&mut rbb.rbb_ref_mut()).expect("Deserialize data 2");

    assert_eq!(d1.first, d2.first);

    d2.block.inner.expect_err("Should error with invalid size");

    assert_eq!(d1.end, d2.end);
}

#[test]
fn dyn_compat() {
    let s = String::from("hello dyn");

    let s_dyn: &dyn SerBytes = &s;

    let mut wbb = WriteByteBufferOwned::new();

    s_dyn.to_buf(&mut wbb);

    let mut rbb = ReadByteBufferOwned::from_vec(wbb);
    let mut rbuf = rbb.rbb_ref_mut();

    let s_deser = String::from_buf(&mut rbuf).expect("Clean read of dyn str");

    assert_eq!(s, s_deser);
}
