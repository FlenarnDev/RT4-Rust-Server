pub struct Verification {
    pub(crate) verification: i32
}

impl Verification {
    #[inline]
    pub fn new(verification: i32) -> Verification {
        Verification {
            verification
        }
    }
}