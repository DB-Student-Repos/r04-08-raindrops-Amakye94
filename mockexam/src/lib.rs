use std::mem::{self, MaybeUninit};

pub struct CircularBuffer<T> {
    write_ptr: usize,              // % (2 * capacity)
    read_ptr: usize,               // % (2 * capacity)
    buffer: Box<[MaybeUninit<T>]>, // len() == capacity
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut data: Vec<MaybeUninit<T>> = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            data.push(MaybeUninit::uninit());
        }

        Self {
            write_ptr: 0,
            read_ptr: 0,
            buffer: data.into_boxed_slice(),
        }
    }

    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }

    fn ptr_max(&self) -> usize {
        2 * self.capacity()
    }

    pub fn len(&self) -> usize {
        (self.write_ptr - self.read_ptr) % self.ptr_max()
    }

    pub fn full(&self) -> bool {
        self.len() == self.capacity()
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.full() {
            return Err(Error::FullBuffer);
        }

        self.buffer[self.write_ptr % self.capacity()] = MaybeUninit::new(element);
        self.write_ptr = (self.write_ptr + 1) % self.ptr_max();

        Ok(())
    }

    unsafe fn read_at(&mut self, read_ptr: usize) -> T {
        let mut buffer_data = self.buffer.get_unchecked_mut(read_ptr % self.capacity());
        let mut data = MaybeUninit::uninit();
        mem::swap(&mut data, &mut buffer_data);

        data.assume_init()
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.write_ptr == self.read_ptr {
            return Err(Error::EmptyBuffer);
        }

        let data = unsafe { self.read_at(self.read_ptr) };
        self.read_ptr = (self.read_ptr + 1) % self.ptr_max();

        Ok(data)
    }

    pub fn clear(&mut self) {
        while let Ok(_data) = self.read() {
            // Drop the data stored in the buffer
        }
    }

    pub fn overwrite(&mut self, element: T) {
        if self.full() {
            // Drop the data stored in the buffer
            let _data = self.read();
        }

        self.write(element).unwrap();
    }
}