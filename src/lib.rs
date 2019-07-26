use std::marker::PhantomData;
use std::{mem, ptr, borrow::{Cow, ToOwned}};
use zerocopy::{LayoutVerified, AsBytes, ByteSlice, FromBytes};

fn aligned_to(bytes: &[u8], align: usize) -> bool {
    (bytes as *const _ as *const () as usize) % align == 0
}

pub fn into_cow<T>(bytes: &[u8]) -> Option<Cow<'_, [T]>>
where T: FromBytes + Clone,
{
    assert_ne!(mem::size_of::<T>(), 0);

    if bytes.len() % mem::size_of::<T>() != 0 {
        return None;
    }

    if !aligned_to(bytes, mem::align_of::<T>()) {
        let len = bytes.len();
        let elem_size = mem::size_of::<T>();
        debug_assert_ne!(elem_size, 0);
        debug_assert_eq!(len % elem_size, 0);
        let elems = len / elem_size;

        let mut buf = Vec::with_capacity(elems);

        unsafe {
            let src = bytes.as_ptr();
            let dst = buf.as_mut_ptr() as *mut u8;
            ptr::copy_nonoverlapping(src, dst, len);
            buf.set_len(elems);
        }

        return Some(Cow::Owned(buf))
    }

    let verified = LayoutVerified::new_slice(bytes).unwrap().into_slice();
    Some(Cow::Borrowed(verified))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
