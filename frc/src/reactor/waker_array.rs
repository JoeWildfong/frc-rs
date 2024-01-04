use std::task::Waker;

pub(crate) struct WakerArray<const T: usize> {
    wakers: [Option<Waker>; T],
}

impl<const T: usize> WakerArray<T> {
    pub fn register(&mut self, waker: Waker, i: usize) -> Result<(), Waker> {
        let slot = &mut self.wakers[i];
        match slot {
            Some(_) => Err(waker),
            None => {
                *slot = Some(waker);
                Ok(())
            }
        }
    }

    pub fn replace(&mut self, waker: Waker, i: usize) -> Option<Waker> {
        self.wakers[i].replace(waker)
    }

    pub fn wake(&mut self, i: usize) -> bool {
        self.wakers[i].take().map(Waker::wake).is_some()
    }

    pub fn new() -> Self {
        Self {
            wakers: std::array::from_fn(|_| None),
        }
    }
}

impl<const T: usize> Default for WakerArray<T> {
    fn default() -> Self {
        Self::new()
    }
}
