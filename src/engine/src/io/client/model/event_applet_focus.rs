pub struct EventAppletFocus {
    pub(crate) focus: bool,
}

impl EventAppletFocus {
    #[inline]
    pub fn new(focus: bool) -> EventAppletFocus {
        EventAppletFocus {
            focus
        }
    }
}