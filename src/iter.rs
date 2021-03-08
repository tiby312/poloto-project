pub use file::*;
mod file {
    use super::*;
    use std::path::Path;
    ///[`PlotIterator`] provides two ways to create a [`DoubleIterator`].
    ///A third option is to use the iterator only once, but instead
    ///of storing in memory, we use a file as a buffer.
    ///
    ///This way we only use the iterator once, and also don't need to store all the results
    ///in memory.
    ///
    ///We dont auto implement this for iterator types since
    ///it is specilized for `[f64;2]`.
    pub struct FileBuffer<P: AsRef<Path>, I: Iterator<Item = [f64; 2]>> {
        path: P,
        file: std::io::BufWriter<std::fs::File>,
        inner: I,
    }

    /// Create a [`FileBuffer`]
    pub fn file_buffer<P: AsRef<Path>, I: Iterator<Item = [f64; 2]>>(
        inner: I,
        path: P,
    ) -> FileBuffer<P, I> {
        FileBuffer::new(inner, path)
    }

    impl<P: AsRef<Path>, I: Iterator<Item = [f64; 2]>> FileBuffer<P, I> {
        /// Constructor
        fn new(inner: I, path: P) -> Self {
            let file = std::fs::File::create(&path).unwrap();
            FileBuffer {
                path,
                file: std::io::BufWriter::new(file),
                inner,
            }
        }
    }

    impl<P: AsRef<Path>, I: Iterator<Item = [f64; 2]>> Iterator for FileBuffer<P, I> {
        type Item = [f64; 2];
        fn next(&mut self) -> Option<Self::Item> {
            if let Some(a) = self.inner.next() {
                use std::io::Write;
                writeln!(self.file, "{},{}", a[0], a[1]).unwrap();
                Some(a)
            } else {
                None
            }
        }
    }

    impl<P: AsRef<Path>, I: Iterator<Item = [f64; 2]>> DoubleIterator for FileBuffer<P, I> {
        type Next = FileBufferRead;
        fn finish_first(mut self) -> Self::Next {
            use std::io::BufRead;
            use std::io::Seek;
            use std::io::SeekFrom;
            use std::io::Write;
            self.file.flush().unwrap();
            self.file.seek(SeekFrom::Start(0)).unwrap();
            let f = std::fs::File::open(self.path).unwrap();
            FileBufferRead {
                lines: std::io::BufReader::new(f).lines(),
            }
        }
    }

    /// Iterate over the plots that were stored to a file.
    pub struct FileBufferRead {
        lines: std::io::Lines<std::io::BufReader<std::fs::File>>,
    }

    impl Iterator for FileBufferRead {
        type Item = [f64; 2];
        fn next(&mut self) -> Option<Self::Item> {
            if let Some(a) = self.lines.next() {
                match a {
                    Ok(a) => {
                        let mut i = a.split(",");
                        let aa: f64 = i.next().unwrap().parse().unwrap();
                        let bb: f64 = i.next().unwrap().parse().unwrap();
                        Some([aa, bb])
                    }
                    Err(e) => {
                        panic!("parse error {:?}", e);
                    }
                }
            } else {
                None
            }
        }
    }
}

impl<I: Iterator + Sized> PlotIterator for I {}

///Trait that is implemented for all iterators through a blanket impl.
pub trait PlotIterator: IntoIterator + Sized {
    ///Create a [`DoubleIterator`] that uses an iterator just once,
    ///and stores the plots in a Vec for the second iteration.
    fn buffer_iter(self) -> BufferIter<Self::IntoIter> {
        let i = self.into_iter();
        let ll = i.size_hint().0;
        BufferIter {
            inner: i,
            buffer: Vec::with_capacity(ll),
        }
    }

    ///Create a [`DoubleIterator`] that uses an iterator twice
    ///by cloning it once.
    fn twice_iter(self) -> NoBufferIter<Self::IntoIter>
    where
        Self::IntoIter: Clone,
    {
        let i = self.into_iter();
        let sec = i.clone();
        NoBufferIter {
            inner: i,
            inner2: sec,
        }
    }
}

///The trait that plot functions accept.
///All plots must be iterated through twice.
///Once to find the right scale to fit all the plots in the graph.
///And a second time to scale all the plots by the scale we found
///on the first iteration.
///A [`DoubleIterator`] is itself an iterator
///that represents its first iteration. Once that is done,
///the user can call [`finish_first`](DoubleIterator::finish_first) to
///produce the second iterator.
pub trait DoubleIterator: Iterator {
    type Next: Iterator<Item = Self::Item>;
    fn finish_first(self) -> Self::Next;
}

/// Created by [`PlotIterator::buffer_iter`]
pub struct BufferIter<I: Iterator> {
    inner: I,
    buffer: Vec<I::Item>,
}

impl<I: Iterator> DoubleIterator for BufferIter<I>
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

/// Created by [`PlotIterator::buffer_iter`]
pub struct NoBufferIter<I: Iterator> {
    inner: I,
    inner2: I,
}

impl<I: Iterator> DoubleIterator for NoBufferIter<I>
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
