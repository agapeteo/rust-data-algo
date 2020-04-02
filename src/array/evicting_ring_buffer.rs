pub struct EvictingRingBuffer<'a, T> {
    buff: Vec<&'a T>,
    write_idx: usize,
    read_idx: usize,
    size: usize,
    capacity: usize,
}

#[allow(dead_code)]
impl<'a, T> EvictingRingBuffer<'a, T> {
    pub fn new(capacity: usize) -> Self {
        let buff: Vec<&'a T> = Vec::with_capacity(capacity);
        EvictingRingBuffer { buff, write_idx: 0, read_idx: 0, size: 0, capacity }
    }

    pub fn push(&mut self, value: &'a T) {
        if let Some(existing) = self.buff.get_mut(self.write_idx) {
            *existing = value;
        } else {
            self.buff.insert(self.write_idx, value);
        }

        self.write_idx = self.next_idx(self.write_idx);

        if self.size == self.capacity {
            self.read_idx = self.next_idx(self.read_idx);
        } else {
            self.size += 1;
        }
        // self.write_idx = self.next_idx(self.write_idx);
        // println!("write_idx: {}, read_idx: {}", self.write_idx, self.read_idx);
    }

    pub fn take(&mut self) -> Option<&&'a T> {
        if self.size == 0 {
            return None;
        }
        let result = self.buff.get(self.read_idx);
        self.read_idx = self.next_idx(self.read_idx);
        self.size -= 1;

        result
    }

    fn next_idx(&self, idx: usize) -> usize {
        (idx + 1) % self.capacity
    }

    // fn print_values(&self) {
    //     let mut buf_size = self.size;
    //     let mut cur_idx = self.read_idx;
    //
    //     while buf_size > 0 {
    //         println!("{:?}", self.buff[cur_idx]);
    //         cur_idx = self.next_idx(cur_idx);
    //         buf_size -= 1;
    //     }
    // }

    // fn iter(&self) -> BufferIter<'a, 'k, T> {
    //     // let buff_elements: &'k [&'a T] = &'k self.buff;
    //     // let buff_elements: &'k[&'a T] = self.buff.as_slice();
    //     let buff_elements: &'k Vec<&'a T> = &self.buff;
    //     BufferIter { cur_idx: self.read_idx, items_left: self.size, buff_elements }
    // }
}

// // TODO! ?? how properly implement it?
// pub struct BufferIter<'a, 'k, T> {
//     cur_idx: usize,
//     items_left: usize,
//     buff_elements: &'k Vec<&'a T>,
// }
//
// impl<'a,'k: 'a, T> Iterator for BufferIter<'a,'k, T> {
//     type Item = &'a T;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.items_left == 0 {
//             return None;
//         }
//         let result = self.buff_elements.get(self.cur_idx);
//         self.items_left -= 1;
//         self.cur_idx = (self.cur_idx + 1) % self.buff_elements.len();
//
//         result.copied()
//     }
// }

#[test]
fn test() {
    let mut buf: EvictingRingBuffer<i32> = EvictingRingBuffer::new(3);

    assert_eq!(buf.take(), None);

    buf.push(&1);
    buf.push(&2);
    buf.push(&3);
    buf.push(&4);
    buf.push(&5);

    assert_eq!(buf.take(), Some(&&3));
    assert_eq!(buf.take(), Some(&&4));
    assert_eq!(buf.take(), Some(&&5));
    assert_eq!(buf.take(), None);


    // TODO ??? why next has compilation errors?
    // let mut actual: Vec<Option<&&i32>> = Vec::new();
    // actual.push(buf.take());
    // actual.push(buf.take());
}