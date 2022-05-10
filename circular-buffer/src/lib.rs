pub struct CircularBuffer<T> {
    buff: Vec<Option<T>>,
    index_r: usize,
    index_w: usize,
    length: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

pub fn next(index: usize, len: usize) -> usize {
    (index + 1) % len
}
pub fn prev(index: usize, len: usize) -> usize {
    if index == 0 {
        len - 1
    } else {
        index - 1
    }
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut arr = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            arr.push(None);
        }

        Self {
            buff: arr,
            index_r: 0,
            index_w: 0,
            length: capacity,
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.index_r == self.index_w && self.buff[self.index_w].is_some() {
            Err(Error::FullBuffer)
        } else {
            self.buff[self.index_w] = Some(_element);
            self.index_w = next(self.index_w, self.length);
            Ok(())
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        //println!("r:{} w:{} len:{}  ",self.index_r,self.index_w,self.length);
        if (self.index_r == self.index_w) && self.buff[self.index_r].is_none() {
            Err(Error::EmptyBuffer)
        } else {
            let tmp = self.buff[self.index_r].take();
            self.index_r = next(self.index_r, self.length);
            Ok(tmp.unwrap())
        }
    }

    pub fn clear(&mut self) {
        for x in self.buff.iter_mut() {
            x.take();
        }
        self.index_r = 0;
        self.index_w = 0;
    }

    pub fn overwrite(&mut self, _element: T) {
        if self.index_r == self.index_w && self.buff[self.index_w].is_some() {
            self.buff[self.index_r].replace(_element);
            self.index_w = next(self.index_w, self.length);
            self.index_r=self.index_w;
        } else {
            self.buff[self.index_w] = Some(_element);
            self.index_w = next(self.index_w, self.length);
        }     
    }
}
