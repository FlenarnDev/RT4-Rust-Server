pub struct NoTimeout;

impl NoTimeout {
    pub(crate) const DEFAULT: NoTimeout = NoTimeout::new();
    
    #[inline]
    pub const fn new() -> NoTimeout {
        NoTimeout
    }
}