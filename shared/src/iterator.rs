use std::mem::MaybeUninit;

/// Implements additional methods for iterators. This could also use `itertools`, but what's the
/// fun in that?  
pub trait ToArray<T>: Iterator {
    /// Collects `N` items from the iterator into an array. Panics when there are too few remaining
    /// items in the iterator.
    fn collect_array<const N: usize>(&mut self) -> [T; N];
}

/// Blanket implementation for all iterators.
impl<I, T> ToArray<T> for I
where
    I: Iterator<Item = T>,
{
    fn collect_array<const N: usize>(&mut self) -> [T; N] {
        // SAFETY: the `MaybeUninit`s that are written into the array do not have to be initialized.
        let mut array: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };
        for item in &mut array {
            item.write(self.next().expect("expected item"));
        }
        // SAFETY: all elements in the array have been initialized above.
        unsafe { array.as_ptr().cast::<[T; N]>().read() }
    }
}
