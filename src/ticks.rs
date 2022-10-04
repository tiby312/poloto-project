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

pub struct DefaultTickFmt<'a, N> {
    _inner: std::marker::PhantomData<&'a N>,
}
impl<'a, N: Display> DefaultTickFmt<'a, N> {
    pub fn new() -> DefaultTickFmt<'a, N> {
        DefaultTickFmt {
            _inner: std::marker::PhantomData,
        }
    }
}
impl<'a, N: Display> TickFmt for DefaultTickFmt<'a, N> {
    type Num = N;
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

pub trait TickFmt {
    type Num;
    fn write_tick(&mut self, a: &mut dyn std::fmt::Write, val: &Self::Num) -> std::fmt::Result;
    fn write_where(&mut self, _: &mut dyn std::fmt::Write) -> std::fmt::Result {
        Ok(())
    }

    fn with_ticks<F: FnMut(&mut dyn fmt::Write, &Self::Num) -> fmt::Result>(
        self,
        func: F,
    ) -> WithTicky<Self, F>
    where
        Self: Sized,
    {
        WithTicky { ticks: self, func }
    }

    fn with_where<F: FnMut(&mut dyn fmt::Write) -> fmt::Result>(self, func: F) -> WithWhere<Self, F>
    where
        Self: Sized,
    {
        WithWhere { ticks: self, func }
    }
}

pub struct WithWhere<D, F> {
    ticks: D,
    func: F,
}

impl<D: TickFmt, F> TickFmt for WithWhere<D, F>
where
    F: FnMut(&mut dyn std::fmt::Write) -> fmt::Result,
{
    type Num = D::Num;
    fn write_tick(&mut self, a: &mut dyn std::fmt::Write, val: &Self::Num) -> std::fmt::Result {
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
impl<D: TickFmt, F> TickFmt for WithTicky<D, F>
where
    F: FnMut(&mut dyn std::fmt::Write, &D::Num) -> fmt::Result,
{
    type Num = D::Num;
    fn write_tick(&mut self, a: &mut dyn std::fmt::Write, val: &Self::Num) -> std::fmt::Result {
        (self.func)(a, val)
    }
    fn write_where(&mut self, w: &mut dyn std::fmt::Write) -> std::fmt::Result {
        self.ticks.write_where(w)
    }
}

pub fn from_closure<N: PlotNum, It, F>(func: F) -> ClosureTickFormat<F>
where
    It: TickDist<Num = N>,
    F: FnOnce(&DataBound<N>, &RenderOptionsBound, IndexRequester) -> It,
{
    ClosureTickFormat { func }
}

pub struct ClosureTickFormat<F> {
    func: F,
}

impl<N: PlotNum, Res: TickDist<Num = N>, F> GenTickDist<N> for ClosureTickFormat<F>
where
    F: FnOnce(&DataBound<N>, &RenderOptionsBound, IndexRequester) -> Res,
    N: PlotNum,
{
    type Res = Res;
    fn generate(
        self,
        data: &ticks::DataBound<N>,
        canvas: &RenderOptionsBound,
        req: IndexRequester,
    ) -> Res {
        (self.func)(data, canvas, req)
    }
}

pub struct TickRes {
    pub dash_size: Option<f64>,
}

///
/// Formatter for a tick.
///
pub trait GenTickDist<N> {
    type Res: TickDist<Num = N>;
    fn generate(
        self,
        data: &ticks::DataBound<N>,
        canvas: &RenderOptionsBound,
        req: IndexRequester,
    ) -> Self::Res;
}

pub trait TickDist {
    type Num;
    type It: IntoIterator<Item = Self::Num>;
    type Fmt: TickFmt<Num = Self::Num>;
    fn unwrap(self) -> TickDistRes<Self::It, Self::Fmt>;
}
impl<I: IntoIterator, F: TickFmt<Num = I::Item>> TickDist for TickDistRes<I, F> {
    type Num = I::Item;
    type It = I;
    type Fmt = F;
    fn unwrap(self) -> TickDistRes<Self::It, Self::Fmt> {
        self
    }
}

pub struct TickDistRes<I, F> {
    pub it: I,
    pub fmt: F,
    pub res: TickRes,
}

impl<'a, X: PlotNum, I: IntoIterator<Item = X>> TickDistRes<I, DefaultTickFmt<'a, X>>
where
    X: fmt::Display,
{
    pub fn new(it: I) -> Self {
        Self::from_parts(it, DefaultTickFmt::new(), TickRes { dash_size: None })
    }
}
impl<X: PlotNum, I: IntoIterator<Item = X>, Fmt: TickFmt<Num = X>> TickDistRes<I, Fmt> {
    pub fn from_parts(it: I, fmt: Fmt, res: TickRes) -> Self {
        TickDistRes { it, fmt, res }
    }

    pub fn with_fmt<J: TickFmt<Num = I::Item>>(self, other: J) -> TickDistRes<I, J> {
        TickDistRes {
            it: self.it,
            fmt: other,
            res: self.res,
        }
    }
}
impl<X: PlotNum, I: IntoIterator<Item = X>, Fmt: TickFmt<Num = X>> GenTickDist<X>
    for TickDistRes<I, Fmt>
{
    type Res = Self;
    fn generate(self, _: &ticks::DataBound<X>, _: &RenderOptionsBound, _: IndexRequester) -> Self {
        self
    }
}
