//!
//! Tools to create tick distributions.
//!
use super::*;

///
/// Building block to make ticks.
///
/// Created once the min and max bounds of all the plots has been computed.
/// Contains in it all the information typically needed to make a [`TickFormat`].
///
/// Used by [`ticks::from_default`]
///
#[derive(Debug, Clone)]
pub struct Bound<'a, X> {
    pub data: &'a ticks::DataBound<X>,
    pub canvas: &'a RenderOptionsBound,
}

///
/// Tick relevant information of [`Data`]
///
#[derive(Debug, Clone)]
pub struct DataBound<X> {
    pub min: X,
    pub max: X,
}

// ///
// /// Construct x and y tick data to be fed into a tick generator.
// ///
// pub fn bounds<'a, X, Y, P, K: Renderable>(
//     data: &'a Data<X, Y, P>,
//     render: &'a K,
// ) -> (Bound<'a, X>, Bound<'a, Y>) {
//     let (dx, dy) = data.bounds();
//     let (cx, cy) = render.bounds();
//     (
//         Bound {
//             data: dx,
//             canvas: cx,
//         },
//         Bound {
//             data: dy,
//             canvas: cy,
//         },
//     )
// }

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

///
/// Create a [`TickFormat`] from a step iterator.
///
///
pub fn from_iter<X: PlotNum + Display, I: Iterator<Item = X>>(ticks: I) -> TickIterFmt<I> {
    TickIterFmt { ticks }
}

pub struct DefaultTickFmt;

impl<N: Display> TickFmt<N> for DefaultTickFmt {
    fn write_tick(&mut self, a: &mut dyn std::fmt::Write, val: &N) -> std::fmt::Result {
        write!(a, "{}", val)
    }
    fn write_where(
        &mut self,
        _: &mut dyn std::fmt::Write,
        _req: IndexRequester,
    ) -> std::fmt::Result {
        Ok(())
    }
}

///
/// Used by [`ticks::from_iter`]
///
pub struct TickIterFmt<I: Iterator> {
    ticks: I,
}
impl<I: Iterator> TickFormat for TickIterFmt<I>
where
    I::Item: PlotNum + Display,
{
    type Num = I::Item;
    type It = I;
    type Fmt = DefaultTickFmt;
    fn generate(
        self,
        data: &ticks::DataBound<Self::Num>,
        canvas: &RenderOptionsBound,
    ) -> (TickRes, Self::It, Self::Fmt) {
        (TickRes { dash_size: None }, self.ticks, DefaultTickFmt)
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
    pub fn request(&mut self) -> usize {
        let val = *self.counter;
        *self.counter += 1;
        val
    }
}

pub trait TickFmt<N> {
    fn write_tick(&mut self, a: &mut dyn std::fmt::Write, val: &N) -> std::fmt::Result;
    fn write_where(
        &mut self,
        _: &mut dyn std::fmt::Write,
        _req: IndexRequester,
    ) -> std::fmt::Result {
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
        F: FnMut(&mut dyn std::fmt::Write, IndexRequester) -> fmt::Result,
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
    F: FnMut(&mut dyn std::fmt::Write, IndexRequester) -> fmt::Result,
{
    fn write_tick(&mut self, a: &mut dyn std::fmt::Write, val: &N) -> std::fmt::Result {
        self.ticks.write_tick(a, val)
    }
    fn write_where(
        &mut self,
        w: &mut dyn std::fmt::Write,
        req: IndexRequester,
    ) -> std::fmt::Result {
        (self.func)(w, req)
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
    fn write_where(
        &mut self,
        w: &mut dyn std::fmt::Write,
        req: IndexRequester,
    ) -> std::fmt::Result {
        self.ticks.write_where(w, req)
    }
}

pub struct TickRes {
    pub dash_size: Option<f64>,
}

///
/// Formatter for a tick.
///
pub trait TickFormat {
    type Num: PlotNum;
    type It: IntoIterator<Item = Self::Num>;
    type Fmt: TickFmt<Self::Num>;
    fn generate(
        self,
        data: &ticks::DataBound<Self::Num>,
        canvas: &RenderOptionsBound,
    ) -> (TickRes, Self::It, Self::Fmt);

    fn with_fmt<F: TickFmt<Self::Num>>(self, fmt: F) -> WithFmt<Self, F>
    where
        Self: Sized,
    {
        WithFmt { ticks: self, fmt }
    }
}

pub struct WithFmt<T, F> {
    ticks: T,
    fmt: F,
}
impl<T: TickFormat, F: TickFmt<T::Num>> TickFormat for WithFmt<T, F> {
    type Num = T::Num;
    type It = T::It;
    type Fmt = F;
    fn generate(
        self,
        data: &ticks::DataBound<Self::Num>,
        canvas: &RenderOptionsBound,
    ) -> (TickRes, Self::It, Self::Fmt) {
        let (a, b, c) = self.ticks.generate(data, canvas);
        (a, b, self.fmt)
    }
}

// pub trait TickFormatExt: TickFormat {
//     fn with_tick_fmt<F>(self, func: F) -> TickFmt<Self, F>
//     where
//         Self: Sized,
//         F: Fn(&mut dyn std::fmt::Write, &Self::Num) -> std::fmt::Result,
//     {
//         TickFmt { inner: self, func }
//     }

//     fn with_where_fmt<F>(self, func: F) -> WhereFmt<Self, F>
//     where
//         Self: Sized,
//         F: Fn(&mut dyn std::fmt::Write) -> std::fmt::Result,
//     {
//         WhereFmt { inner: self, func }
//     }
// }

// impl<T: TickFormat> TickFormatExt for T {}

// ///
// /// Used by [`TickFormatExt::with_where_fmt`]
// ///
// pub struct WhereFmt<T, F> {
//     inner: T,
//     func: F,
// }
// impl<T: TickFormat, F: Fn(&mut dyn std::fmt::Write) -> std::fmt::Result> TickFormat
//     for WhereFmt<T, F>
// {
//     type Num = T::Num;
//     fn write_tick(&mut self, a: &mut dyn std::fmt::Write, val: &Self::Num) -> std::fmt::Result {
//         self.inner.write_tick(a, val)
//     }
//     fn write_where(
//         &mut self,
//         a: &mut dyn std::fmt::Write,
//         _req: IndexRequester,
//     ) -> std::fmt::Result {
//         (self.func)(a)
//     }
//     fn dash_size(&self) -> Option<f64> {
//         self.inner.dash_size()
//     }
//     fn next_tick(&mut self) -> Option<Self::Num> {
//         self.inner.next_tick()
//     }
// }

// ///
// /// Used by [`TickFormatExt::with_tick_fmt`]
// ///
// pub struct TickFmt<T, F> {
//     inner: T,
//     func: F,
// }
// impl<T: TickFormat, F: Fn(&mut dyn std::fmt::Write, &T::Num) -> std::fmt::Result> TickFormat
//     for TickFmt<T, F>
// {
//     type Num = T::Num;
//     fn write_tick(&mut self, a: &mut dyn std::fmt::Write, val: &Self::Num) -> std::fmt::Result {
//         (self.func)(a, val)
//     }
//     fn write_where(
//         &mut self,
//         a: &mut dyn std::fmt::Write,
//         ind: IndexRequester,
//     ) -> std::fmt::Result {
//         self.inner.write_where(a, ind)
//     }
//     fn dash_size(&self) -> Option<f64> {
//         self.inner.dash_size()
//     }
//     fn next_tick(&mut self) -> Option<Self::Num> {
//         self.inner.next_tick()
//     }
// }
