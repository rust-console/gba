use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::ops::{Deref, DerefMut};
use core::ptr;
use core::sync::atomic::{compiler_fence, Ordering};
use super::*;

#[inline(never)]
fn already_locked() -> ! {
    panic!("This lock has already been locked by another thread.")
}

/// A mutex that prevents code from running in both an IRQ and normal code at
/// the same time.
///
/// Note that this does not support blocking like a typical mutex, and instead
/// mainly exists for memory safety reasons.
pub struct RawMutex(Static<bool>);
impl RawMutex {
    /// Creates a new lock.
    pub const fn new() -> Self {
        RawMutex(Static::new(false))
    }

    /// Locks the mutex and returns whether a lock was successfully acquired.
    fn raw_lock(&self) -> bool {
        if self.0.replace(true) {
            // value was already true, opps.
            false
        } else {
            // prevent any weird reordering, and continue
            compiler_fence(Ordering::Acquire);
            true
        }
    }

    /// Unlocks the mutex.
    fn raw_unlock(&self) {
        compiler_fence(Ordering::Release);
        if !self.0.replace(false) { already_locked() }
    }

    /// Returns a guard for this lock, or panics if there is another lock active.
    pub fn lock(&self) -> RawMutexGuard<'_> {
        self.try_lock().unwrap_or_else(|| already_locked())
    }

    /// Returns a guard for this lock, or `None` if there is another lock active.
    pub fn try_lock(&self) -> Option<RawMutexGuard<'_>> {
        if self.raw_lock() {
            Some(RawMutexGuard(self))
        } else {
            None
        }
    }
}
unsafe impl Send for RawMutex {}
unsafe impl Sync for RawMutex {}

/// A guard representing an active lock on an [`RawMutex`].
pub struct RawMutexGuard<'a>(&'a RawMutex);
impl <'a> Drop for RawMutexGuard<'a> {
    fn drop(&mut self) {
        self.0.raw_unlock();
    }
}

/// A mutex that protects an object from being accessed in both an IRQ and
/// normal code at once.
///
/// Note that this does not support blocking like a typical mutex, and instead
/// mainly exists for memory safety reasons.
pub struct Mutex<T> {
    raw: RawMutex,
    data: UnsafeCell<T>,
}
impl <T> Mutex<T> {
    /// Creates a new lock containing a given value.
    pub const fn new(t: T) -> Self {
        Mutex {
            raw: RawMutex::new(),
            data: UnsafeCell::new(t),
        }
    }

    /// Returns a guard for this lock, or panics if there is another lock active.
    pub fn lock(&self) -> MutexGuard<'_, T> {
        self.try_lock().unwrap_or_else(|| already_locked())
    }

    /// Returns a guard for this lock or `None` if there is another lock active.
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        if self.raw.raw_lock() {
            Some(MutexGuard {
                underlying: self,
                ptr: self.data.get(),
            })
        } else {
            None
        }
    }
}
unsafe impl <T> Send for Mutex<T> {}
unsafe impl <T> Sync for Mutex<T> {}

/// A guard representing an active lock on an [`Mutex`].
pub struct MutexGuard<'a, T> {
    underlying: &'a Mutex<T>,
    ptr: *mut T,
}
impl <'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.underlying.raw.raw_unlock();
    }
}
impl <'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}
impl <'a, T> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}

enum Void { }

/// A helper type that ensures a particular value is only initialized once.
pub struct InitOnce<T> {
    state: Static<u8>,
    value: UnsafeCell<MaybeUninit<T>>,
}
impl <T> InitOnce<T> {
    /// Creates a new uninitialized object.
    pub const fn new() -> Self {
        InitOnce {
            state: Static::new(0),
            value: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    /// Gets the contents of this state, or initializes it if it has not already
    /// been initialized.
    ///
    /// The initializer function is guaranteed to only be called once.
    ///
    /// Take care when sharing an `InitOnce` object between an IRQ and normal
    /// code. If this function is called in an IRQ when it is already currently
    /// being initialized by user code, this function will panic.
    pub fn get(&self, initializer: impl FnOnce() -> T) -> &T {
        match self.try_get(|| -> Result<T, Void> { Ok(initializer()) }) {
            Ok(v) => v,
            _ => unimplemented!(),
        }
    }

    /// Gets the contents of this state, or initializes it if it has not already
    /// been initialized.
    ///
    /// The initializer function is guaranteed to only be called once if it
    /// returns `Ok`. If it returns `Err`, it will be called again in the
    /// future until an attempt at initialization succeeds.
    ///
    /// Take care when sharing an `InitOnce` object between an IRQ and normal
    /// code. If this function is called in an IRQ when it is already currently
    /// being initialized by user code, this function will panic.
    pub fn try_get<E>(&self, initializer: impl FnOnce() -> Result<T, E>) -> Result<&T, E> {
        unsafe {
            if self.state.read() != 2 {
                // Locks the initializer
                if self.state.replace(1) != 0 {
                    panic!("Attempt to initialize `InitOnce` that is already in initialization.");
                }

                // Initialize the actual value.
                let init = match initializer() {
                    Ok(v) => v,
                    Err(e) => {
                        assert_eq!(self.state.replace(0), 1);
                        return Err(e);
                    }
                };
                ptr::write_volatile((*self.value.get()).as_mut_ptr(), init);
                assert_eq!(self.state.replace(2), 1);
            }
            Ok(&*(*self.value.get()).as_mut_ptr())
        }
    }
}
impl <T> Drop for InitOnce<T> {
    fn drop(&mut self) {
        if self.state.read() == 2 {
            // drop the value inside the `MaybeUninit`
            unsafe { ptr::read((*self.value.get()).as_ptr()); }
        }
    }
}
unsafe impl <T: Send> Send for InitOnce<T> {}
unsafe impl <T: Sync> Sync for InitOnce<T> {}
