use core::ops::{Deref, DerefMut};

pub(crate) enum Toast<T>
where
    T: Toasty,
{
    Stale(T),
    Fresh(T),
}

pub(crate) trait Toasty: Sized {
    fn detoast(self) -> Toast<Self>;
    /// Why does it always land butter-side down?
    unsafe fn drop_toast(&mut self);
}

impl<T: Toasty> Drop for Toast<T> {
    fn drop(&mut self) {
        match self {
            Toast::Stale(_) => {}
            Toast::Fresh(t) => unsafe { t.drop_toast() },
        }
    }
}

impl<T: Toasty> Deref for Toast<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Toast::Stale(t) => &t,
            Toast::Fresh(t) => &t,
        }
    }
}

impl<T: Toasty> DerefMut for Toast<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Toast::Stale(ref mut t) => t,
            Toast::Fresh(ref mut t) => t,
        }
    }
}
