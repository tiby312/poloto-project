impl<I: Iterator + Sized> PlotIterator for I {}

pub trait PlotIterator: Iterator + Sized {
    fn buffer_iter(self) -> BufferIter<Self> {
        let ll = self.size_hint().0;
        BufferIter {
            inner: self,
            buffer: Vec::with_capacity(ll),
        }
    }
    fn twice_iter(self) -> NoBufferIter<Self>
    where
        Self: Clone,
    {
        let sec = self.clone();
        NoBufferIter {
            inner: self,
            inner2: sec,
        }
    }
}

pub trait DoubleIter: Iterator {
    type Next: Iterator<Item = Self::Item>;
    fn finish_first(self) -> Self::Next;
}

pub struct BufferIter<I: Iterator> {
    inner: I,
    buffer: Vec<I::Item>,
}

impl<I: Iterator> DoubleIter for BufferIter<I>
where
    I::Item: Copy,
{
    type Next = std::vec::IntoIter<I::Item>;
    fn finish_first(self) -> Self::Next {
        self.buffer.into_iter()
    }
}
impl<I: Iterator> Iterator for BufferIter<I>
where
    I::Item: Copy,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(a) = self.inner.next() {
            self.buffer.push(a);
            Some(a)
        } else {
            None
        }
    }
}

pub struct NoBufferIter<I: Iterator> {
    inner: I,
    inner2: I,
}

impl<I: Iterator> DoubleIter for NoBufferIter<I>
where
    I::Item: Copy,
{
    type Next = I;
    fn finish_first(self) -> Self::Next {
        self.inner2
    }
}

impl<I: Iterator> Iterator for NoBufferIter<I>
where
    I::Item: Copy,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
