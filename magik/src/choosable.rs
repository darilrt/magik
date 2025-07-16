pub trait Choosable<T> {
    fn choose(&self, if_true: T, if_false: T) -> T;

    fn choose_with<F>(&self, if_true: F, if_false: F) -> T
    where
        F: FnOnce() -> T;
}

impl<T> Choosable<T> for bool {
    fn choose(&self, if_true: T, if_false: T) -> T {
        if *self { if_true } else { if_false }
    }

    fn choose_with<F>(&self, if_true: F, if_false: F) -> T
    where
        F: FnOnce() -> T,
    {
        if *self { if_true() } else { if_false() }
    }
}
