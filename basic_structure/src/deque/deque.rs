#[derive(Debug)]
pub struct Deque<T> {
    data: Vec<T>,
    cap: usize,
}

impl<T> Deque<T> {
    pub fn new(cap: usize) -> Self {
        Self {
            data: Vec::with_capacity(cap),
            cap,
        }
    }

    pub fn add_rear(&mut self, val: T) -> Result<(), String> {
        if self.data.len() == self.cap {
            return Err("No space available".into());
        }
        self.data.insert(0, val);
        Ok(())
    }

    pub fn add_front(&mut self, val: T) -> Result<(), String> {
        if self.data.len() == self.cap {
            return Err("No space available".into());
        }
        self.data.push(val);
        Ok(())
    }

    pub fn remove_front(&mut self) -> Option<T> {
        if self.size() > 0 {
            // if Self::size(&self) > 0 {
            return self.data.pop();
        }
        return None;
    }

    pub fn remove_rear(&mut self) -> Option<T> {
        if self.size() > 0 {
            // 完全限定语法
            // if Self::size(&self) > 0 {
            return Some(self.data.remove(0));
        }
        return None;
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

pub fn pal_check(text: &str) -> bool {
    let mut deque = Deque::new(text.len());
    for char in text.chars() {
        deque.add_rear(char).unwrap();
    }

    while deque.size() > 1 {
        let left = deque.remove_rear().unwrap();
        let right = deque.remove_front().unwrap();

        if left != right {
            return false;
        }
    }
    true
}
