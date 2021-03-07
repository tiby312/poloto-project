
pub use file::*;
mod file{
    use std::path::Path;
    use super::*;
    ///A third option is to use the iterator only once, but instead
    ///of storing in memory, we use a file as a buffer.
    ///
    ///We dont auto implement this for iterator types since
    ///it is specilized for `[f64;2]`.
    pub struct FileBuffer<P: AsRef<Path>,I:Iterator<Item=[f64;2]>>{
        path:P,
        file:std::fs::File,
        inner:I
    }


    pub fn file_buffer<P: AsRef<Path>,I:Iterator<Item=[f64;2]>>(inner:I,path:P)->FileBuffer<P,I>{
        FileBuffer::new(
            inner,
            path
        )
    }
    impl<P: AsRef<Path>,I:Iterator<Item=[f64;2]>> FileBuffer<P,I>{
        fn new(inner:I,path:P)->Self{
            let file=std::fs::File::create(&path).unwrap();
            FileBuffer{
                path,
                file,
                inner
            }
        }
    }

    impl<P: AsRef<Path>,I:Iterator<Item=[f64;2]>> Iterator for FileBuffer<P,I>{
        type Item=[f64;2];
        fn next(&mut self)->Option<Self::Item>{
            if let Some(a)=self.inner.next(){
                use std::io::Write;
                writeln!(self.file,"{},{}",a[0],a[1]).unwrap();
                Some(a)
            }else{
                None
            }
        }
    }

    impl<P: AsRef<Path>,I: Iterator<Item=[f64;2]>> DoubleIter for FileBuffer<P,I>
    {
        type Next = Reverse;
        fn finish_first(mut self) -> Self::Next {
            use std::io::BufRead;
            use std::io::SeekFrom;
            use std::io::Seek;
            self.file.seek(SeekFrom::Start(0)).unwrap();
            self.file.sync_all().unwrap();
            let f=std::fs::File::open(self.path).unwrap();
            Reverse{
                lines:std::io::BufReader::new(f).lines()
            }
        }
    }

    pub struct Reverse{
        lines:std::io::Lines<std::io::BufReader<std::fs::File>>
    }

    impl Iterator for Reverse{
        type Item=[f64;2];
        fn next(&mut self)->Option<Self::Item>{
            
            if let Some(a)=self.lines.next(){
                match a{
                    Ok(a)=>{
                        let mut i=a.split(",");
                        let aa:f64=i.next().unwrap().parse().unwrap();
                        let bb:f64=i.next().unwrap().parse().unwrap();
                        Some([aa,bb])
                    },
                    Err(e)=>{
                        panic!("parse error {:?}",e);
                    }
                }
            }else{
                None
            }

        }
    }
}

impl<I: Iterator + Sized> PlotIterator for I {}

pub trait PlotIterator: IntoIterator + Sized {
    
    fn buffer_iter(self) -> BufferIter<Self::IntoIter> {
        let i = self.into_iter();
        let ll = i.size_hint().0;
        BufferIter {
            inner: i,
            buffer: Vec::with_capacity(ll),
        }
    }
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
