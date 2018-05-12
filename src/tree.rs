use std::marker::PhantomData;

pub trait Down {
    fn down(&mut self, idx: usize) -> Option<*mut Self>;
}

pub struct TreeCursor<'n, N: 'n + Down> {
    root: PhantomData<&'n mut N>,
    stack: Vec<(*mut N, usize)>,
}

impl<'n, N: 'n + Down> TreeCursor<'n, N> {
    pub fn new(root: &'n mut N) -> Self {
        let root_ptr: *mut N = root;
        TreeCursor { root: PhantomData, stack: vec![(root_ptr, 0)] }
    }

    pub fn get(&self) -> &'n N {
        let here: *const N = self.stack.last().unwrap().0;
        (unsafe { here.as_ref() }).unwrap()
    }

    pub fn get_mut(&mut self) -> &'n mut N {
        let here = self.stack.last().unwrap().0;
        (unsafe { here.as_mut() }).unwrap()
    }

    pub fn down(&mut self) -> bool {
        let idx = self.stack.last().unwrap().1;
        let new_ptr = match self.get_mut().down(idx) {
            Some(x) => x,
            None => return false,
        };

        self.stack.last_mut().unwrap().1 += 1;
        self.stack.push((new_ptr, 0));
        true
    }

    pub fn up(&mut self) -> bool {
        if self.stack.len() == 1 {
            self.stack[0].1 = 0;
            false
        } else {
            self.stack.pop().unwrap();
            true
        }
    }
}
