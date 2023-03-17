#[derive(Debug, PartialEq)]
pub struct Stack<T> {
    data: Vec<T>,
    top: usize,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Self {
            data: Vec::new(),
            top: 0,
        }
    }

    pub fn push(&mut self, val: T) {
        self.data.push(val);
        self.top += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            None
        } else {
            self.top -= 1;

            self.data.pop()
        }
    }
    pub fn peek(&self) -> Option<&T> {
        if self.top == 0 {
            None
        } else {
            self.data.get(self.top - 1)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    pub fn size(&self) -> usize {
        self.top
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_stack() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(
            stack,
            Stack {
                data: vec![1, 2, 3],
                top: 3
            }
        )
    }
}
