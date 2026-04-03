use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, WithParent, WriteByteBufferOwned};
use crate::ser_trait::SerBytes;

pub trait CurrentVersion {
    type Output;
    fn get_data_from_buf(&self, buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self::Output>;

    fn current_version() -> Self;
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct VersioningWrapper<D, V> {
    data: D,
    version: V,
}

impl<D, V> SerBytes for VersioningWrapper<D, V>
where
    D: SerBytes,
    V: SerBytes + CurrentVersion<Output = D>,
{
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        let mut inner = || {
            let version = V::from_buf(buf)?;
            let data = version.get_data_from_buf(buf)?;

            Ok(Self { data, version })
        };

        inner().with_parent("VersioningWrapper")
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        buf.reserve(self.version.approx_size() + self.data.approx_size());

        self.version.to_buf(buf);
        self.data.to_buf(buf);
    }
}

// impl<D, V> Deref for VersioningWrapper<D, V> {
//     type Target = D;
//
//     fn deref(&self) -> &Self::Target {
//         &self.data
//     }
// }
//
// impl<D, V> DerefMut for VersioningWrapper<D, V> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.data
//     }
// }

impl<D, V> From<D> for VersioningWrapper<D, V>
where
    V: CurrentVersion,
{
    fn from(value: D) -> Self {
        Self::new(value)
    }
}

impl<D, V> VersioningWrapper<D, V>
where
    V: CurrentVersion,
{
    pub fn new(data: D) -> Self {
        Self {
            data,
            version: V::current_version(),
        }
    }

    pub fn into_inner(self) -> D {
        self.data
    }

    pub fn inner(&self) -> &D {
        &self.data
    }

    pub fn inner_mut(&mut self) -> &mut D {
        &mut self.data
    }
}
