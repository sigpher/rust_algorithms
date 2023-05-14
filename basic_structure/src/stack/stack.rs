#[derive(Debug)]
pub struct Stack<T> {
    data: Vec<T>,
    top: usize,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            data: Vec::new(),
            top: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
        self.top += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.top > 0 {
            self.top -= 1;
        }
        self.data.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        if self.top == 0 {
            return None;
        }
        return self.data.get(self.top - 1);
    }
    pub fn is_empty(&self) -> bool {
        self.top == 0
    }
    pub fn size(&self) -> usize {
        self.top
    }
}
