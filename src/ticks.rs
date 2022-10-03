//!
//! Tools to create tick distributions.
//!
use super::*;

///
/// Tick relevant information of [`Data`]
///
#[derive(Debug, Clone, Copy)]
pub struct DataBound<X> {
    pub min: X,
    pub max: X,
}

///
/// Tick relevant information of [`RenderOptions`]
///
#[derive(Debug, Clone)]
pub struct RenderOptionsBound {
    pub ideal_num_steps: u32,
    pub ideal_dash_size: f64,
    pub max: f64,
    pub axis: Axis,
}

// pub struct Mapper<X> {
//     inner: X,
// }

// impl<X> Mapper<X> {
//     pub fn map<N: PlotNum, It2, Fmt2, F>(self, func: F) -> Wrappy<Mapper<X>, F>
//     where
//         It2: IntoIterator<Item = N>,
//         Fmt2: TickFmt<N>,
//         F: FnOnce(&DataBound<N>, &RenderOptionsBound) -> TickGen<It2, Fmt2>,
//         X: TickFormat<N>,
//     {
//         Wrappy { inner: self, func }
//     }
// }

pub fn from_closure<N: PlotNum, It, Fmt, F>(func: F) -> Wrappy<F>
where
    It: IntoIterator<Item = N>,
    Fmt: TickFmt<N>,
    F: FnOnce(&DataBound<N>, &RenderOptionsBound, IndexRequester) -> TickGen<It, Fmt>,
{
    Wrappy { func }
}

// impl<N: PlotNum, X: TickFormat<N>> TickFormat<N> for Mapper<X> {
//     type It = X::It;
//     type Fmt = X::Fmt;
//     fn generate(
//         self,
//         data: &ticks::DataBound<N>,
//         canvas: &RenderOptionsBound,
//     ) -> TickGen<Self::It, Self::Fmt> {
//         self.inner.generate(data, canvas)
//     }
// }

pub struct Wrappy<F> {
    func: F,
}

impl<N: PlotNum, It2: IntoIterator<Item = N>, Fmt2: TickFmt<N>, F> TickFormat<N> for Wrappy<F>
where
    F: FnOnce(&DataBound<N>, &RenderOptionsBound, IndexRequester) -> TickGen<It2, Fmt2>,
    It2::Item: PlotNum,
{
    type It = It2;
    type Fmt = Fmt2;
    fn generate(
        self,
        data: &ticks::DataBound<N>,
        canvas: &RenderOptionsBound,
        req: IndexRequester,
    ) -> TickGen<It2, Fmt2> {
        (self.func)(data, canvas, req)
    }
}

pub struct TickBuilder<I, F> {
    it: I,
    fmt: F,
}
impl<I: IntoIterator> TickBuilder<I, DefaultTickFmt> {
    pub fn new(it: I) -> Self {
        TickBuilder {
            it,
            fmt: DefaultTickFmt,
        }
    }
}
impl<I: IntoIterator, F: TickFmt<I::Item>> TickBuilder<I, F> {
    pub fn build(self) -> TickGen<I, F> {
        TickGen {
            it: self.it,
            fmt: self.fmt,
            res: TickRes { dash_size: None },
        }
    }
}

impl<I: IntoIterator, K: TickFmt<I::Item>> TickBuilder<I, K> {
    pub fn with_fmt<J: TickFmt<I::Item>>(self, other: J) -> TickBuilder<I, J> {
        TickBuilder {
            it: self.it,
            fmt: other,
        }
    }
    pub fn with_ticks<F: FnMut(&mut dyn fmt::Write, &I::Item) -> fmt::Result>(
        self,
        func: F,
    ) -> TickBuilder<I, WithTicky<K, F>> {
        TickBuilder {
            it: self.it,
            fmt: self.fmt.with_ticks(func),
        }
    }

    pub fn with_fmt_data<E>(self, data: E) -> TickBuilder<I, WithExtra<K, E>> {
        TickBuilder {
            it: self.it,
            fmt: WithExtra {
                inner: self.fmt,
                data,
            },
        }
    }

    pub fn with_where<F: FnMut(&mut dyn fmt::Write) -> fmt::Result>(
        self,
        func: F,
    ) -> TickBuilder<I, WithWhere<K, F>> {
        TickBuilder {
            it: self.it,
            fmt: self.fmt.with_where(func),
        }
    }
}

pub struct WithExtra<K, E> {
    inner: K,
    pub data: E,
}

impl<N, K: TickFmt<N>, E> TickFmt<N> for WithExtra<K, E> {
    fn write_tick(&mut self, a: &mut dyn std::fmt::Write, val: &N) -> std::fmt::Result {
        self.inner.write_tick(a, val)
    }
    fn write_where(&mut self, w: &mut dyn std::fmt::Write) -> std::fmt::Result {
        self.inner.write_where(w)
    }
}

pub struct DefaultTickFmt;

impl<N: Display> TickFmt<N> for DefaultTickFmt {
    fn write_tick(&mut self, a: &mut dyn std::fmt::Write, val: &N) -> std::fmt::Result {
        write!(a, "{}", val)
    }
    fn write_where(&mut self, _: &mut dyn std::fmt::Write) -> std::fmt::Result {
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Axis {
    X,
    Y,
}

///
/// Useful for numbering footnotes. If one axis uses the number one as a footnote,
/// The second access should use the number two as a footnote.
///
pub struct IndexRequester<'a> {
    counter: &'a mut usize,
}
impl<'a> IndexRequester<'a> {
    #[inline(always)]
    pub fn new(counter: &'a mut usize) -> Self {
        IndexRequester { counter }
    }
    #[inline(always)]
    pub fn request(self) -> usize {
        let val = *self.counter;
        *self.counter += 1;
        val
    }
}

pub trait TickFmt<N> {
    fn write_tick(&mut self, a: &mut dyn std::fmt::Write, val: &N) -> std::fmt::Result;
    fn write_where(&mut self, _: &mut dyn std::fmt::Write) -> std::fmt::Result {
        Ok(())
    }
    fn with_ticks<F>(self, func: F) -> WithTicky<Self, F>
    where
        F: FnMut(&mut dyn std::fmt::Write, &N) -> fmt::Result,
        Self: Sized,
    {
        WithTicky { ticks: self, func }
    }
    fn with_where<F>(self, func: F) -> WithWhere<Self, F>
    where
        F: FnMut(&mut dyn std::fmt::Write) -> fmt::Result,
        Self: Sized,
    {
        WithWhere { ticks: self, func }
    }
}

pub struct WithWhere<D, F> {
    ticks: D,
    func: F,
}

impl<N, D: TickFmt<N>, F> TickFmt<N> for WithWhere<D, F>
where
    F: FnMut(&mut dyn std::fmt::Write) -> fmt::Result,
{
    fn write_tick(&mut self, a: &mut dyn std::fmt::Write, val: &N) -> std::fmt::Result {
        self.ticks.write_tick(a, val)
    }
    fn write_where(&mut self, w: &mut dyn std::fmt::Write) -> std::fmt::Result {
        (self.func)(w)
    }
}

pub struct WithTicky<D, F> {
    ticks: D,
    func: F,
}
impl<N, D: TickFmt<N>, F> TickFmt<N> for WithTicky<D, F>
where
    F: FnMut(&mut dyn std::fmt::Write, &N) -> fmt::Result,
{
    fn write_tick(&mut self, a: &mut dyn std::fmt::Write, val: &N) -> std::fmt::Result {
        (self.func)(a, val)
    }
    fn write_where(&mut self, w: &mut dyn std::fmt::Write) -> std::fmt::Result {
        self.ticks.write_where(w)
    }
}

pub struct TickRes {
    pub dash_size: Option<f64>,
}

///
/// Formatter for a tick.
///
pub trait TickFormat<Num> {
    type It: IntoIterator<Item = Num>;
    type Fmt: TickFmt<Num>;
    fn generate(
        self,
        data: &ticks::DataBound<Num>,
        canvas: &RenderOptionsBound,
        req: IndexRequester,
    ) -> TickGen<Self::It, Self::Fmt>;
}

pub struct TickGen<I, F> {
    pub it: I,
    pub fmt: F,
    pub res: TickRes,
}

impl<X: PlotNum, I: IntoIterator<Item = X>, Fmt: TickFmt<X>> TickGen<I, Fmt> {
    pub fn new(it: I, fmt: Fmt, res: TickRes) -> Self {
        TickGen { it, fmt, res }
    }
}
impl<X: PlotNum, I: IntoIterator<Item = X>, Fmt: TickFmt<X>> TickFormat<X> for TickGen<I, Fmt> {
    type It = I;
    type Fmt = Fmt;
    fn generate(
        self,
        _: &ticks::DataBound<X>,
        _: &RenderOptionsBound,
        _: IndexRequester,
    ) -> TickGen<Self::It, Self::Fmt> {
        self
    }
}

// pub struct WithFmt<T, F> {
//     ticks: T,
//     fmt: F,
// }
// impl<N: PlotNum, T: TickFormat<N>, F: TickFmt<N>> TickFormat<N> for WithFmt<T, F> {
//     type It = T::It;
//     type Fmt = F;
//     fn generate(
//         self,
//         data: &ticks::DataBound<N>,
//         canvas: &RenderOptionsBound,
//         req: IndexRequester,
//     ) -> TickGen<Self::It, Self::Fmt> {
//         let TickGen { it, res, .. } = self.ticks.generate(data, canvas, req);
//         TickGen {
//             it,
//             fmt: self.fmt,
//             res,
//         }
//     }
// }
