#[derive(Debug)]
pub struct Queue<T> {
    data: Vec<T>,
    cap: usize,
}

impl<T> Queue<T> {
    pub fn new(size: usize) -> Self {
        Self {
            data: Vec::with_capacity(size),
            cap: size,
        }
    }
    pub fn enqueue(&mut self, val: T) -> Result<(), String> {
        if self.data.len() == self.cap {
            return Err("No space available".into());
        }
        self.data.insert(0, val);
        Ok(())
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if Self::size(&self) == 0 {
            return None;
        }
        self.data.pop()
    }

    pub fn is_empty(&self) -> bool {
        Self::size(&self) == 0
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

// pub fn hot_potato(names: Vec<&str>, num: usize) -> &str {
//     let mut queue: Queue<&str> = Queue::new(names.len());
//     for name in names {
//         queue.enqueue(name).unwrap();
//     }
//     // println!("{:?}", queue);

//     while queue.size() > 1 {
//         for _ in 0..num {
//             let name = queue.dequeue().unwrap();
//             queue.enqueue(name).unwrap();
//         }
//         // queue.dequeue().unwrap();
//     }
//     queue.dequeue().unwrap()
// }

// hot_potato.rs
pub fn hot_potato(names: Vec<&str>, num: usize) -> &str {
    // 初始化队列 、 名字入队
    let mut q = Queue::new(names.len());
    for name in names {
        let _rm = q.enqueue(name);
    }
    while q.size() > 1 {
        // 出入栈名字，相当于传递山芋
        for _i in 0..num {
            let name = q.dequeue().unwrap();
            let _rm = q.enqueue(name);
        }
        // 出入栈达到num次，删除一人
        let _rm = q.dequeue();
    }

    q.dequeue().unwrap()
}
