#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Block(pub(super) uint32x4_t);

impl Block {
    #[inline]
    pub fn is_empty(self) -> bool {
        let mut out: [u32; 4] = [0; 4];
        unsafe { vst1q_u32(out.as_mut_ptr(), vceqzq_u32(self.0)) };
        out.into_iter().all(|x| x == u32::MAX)
    }

    #[inline]
    pub fn andnot(self, other: Self) -> Self {
        Self(unsafe { vbicq_u32(self.0, other.0) })
    }
}

impl Not for Block {
    type Output = Block;
    #[inline]
    fn not(self) -> Self::Output {
        unsafe { Self(vmvnq_u32(self.0)) }
    }
}

impl BitAnd for Block {
    type Output = Block;
    #[inline]
    fn bitand(self, other: Self) -> Self::Output {
        unsafe { Self(vandq_u32(self.0, other.0)) }
    }
}

impl BitAndAssign for Block {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        unsafe {
            self.0 = vandq_u32(self.0, other.0);
        }
    }
}

impl BitOr for Block {
    type Output = Block;
    #[inline]
    fn bitor(self, other: Self) -> Self::Output {
        unsafe { Self(vorrq_u32(self.0, other.0)) }
    }
}

impl BitOrAssign for Block {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        unsafe { self.0 = vorrq_u32(self.0, other.0) }
    }
}

impl BitXor for Block {
    type Output = Block;
    #[inline]
    fn bitxor(self, other: Self) -> Self::Output {
        unsafe { Self(veorq_u32(self.0, other.0)) }
    }
}

impl BitXorAssign for Block {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        unsafe {
            self.0 = veorq_u32(self.0, other.0);
        }
    }
}

impl PartialEq for Block {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        let mut out: [u32; 4] = [0; 4];
        unsafe {
            let eq = vceqq_u32(self.0, other.0);
            vst1q_u32(out.as_mut_ptr(), eq)
        };
        out.into_iter().all(|x| x == u32::MAX)
    }
}
