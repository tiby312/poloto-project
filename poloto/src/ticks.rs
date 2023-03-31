//!
//! Tools to create tick distributions.
//!

use super::*;

///
/// Min/max bounds for all plots
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

use tick_fmt::*;
pub mod tick_fmt {
    use super::*;
    pub struct DefaultTickFmt;

    impl<N: Display> TickFmt<N> for DefaultTickFmt {
        fn write_tick(&self, a: &mut dyn std::fmt::Write, val: &N) -> std::fmt::Result {
            write!(a, "{}", val)
        }
        fn write_where(&self, _: &mut dyn std::fmt::Write) -> std::fmt::Result {
            Ok(())
        }
    }

    ///
    /// Formatter for a tick distribution
    ///
    pub trait TickFmt<Num> {
        fn write_tick(&self, a: &mut dyn std::fmt::Write, val: &Num) -> std::fmt::Result;
        fn write_where(&self, _: &mut dyn std::fmt::Write) -> std::fmt::Result {
            Ok(())
        }
    }

    pub struct WithWhereFmt<D, F> {
        ticks: D,
        func: F,
    }

    impl<N, D: TickFmt<N>, F, J: fmt::Display> TickFmt<N> for WithWhereFmt<D, F>
    where
        F: Fn() -> J,
    {
        fn write_tick(&self, a: &mut dyn std::fmt::Write, val: &N) -> std::fmt::Result {
            self.ticks.write_tick(a, val)
        }
        fn write_where(&self, w: &mut dyn std::fmt::Write) -> std::fmt::Result {
            let j = (self.func)();
            write!(w, "{}", j)
        }
    }

    pub struct WithTickFmt<D, F> {
        ticks: D,
        func: F,
    }
    impl<N, D: TickFmt<N>, F, K: fmt::Display> TickFmt<N> for WithTickFmt<D, F>
    where
        F: Fn(&N) -> K,
    {
        fn write_tick(&self, a: &mut dyn std::fmt::Write, val: &N) -> std::fmt::Result {
            let j = (self.func)(val);
            write!(a, "{}", j)
        }
        fn write_where(&self, w: &mut dyn std::fmt::Write) -> std::fmt::Result {
            self.ticks.write_where(w)
        }
    }

    pub struct WithData<K, E> {
        ticks: K,
        pub data: E,
    }

    impl<N, K: TickFmt<N>, E> TickFmt<N> for WithData<K, E> {
        fn write_tick(&self, a: &mut dyn std::fmt::Write, val: &N) -> std::fmt::Result {
            self.ticks.write_tick(a, val)
        }
        fn write_where(&self, w: &mut dyn std::fmt::Write) -> std::fmt::Result {
            self.ticks.write_where(w)
        }
    }

    impl<X: PlotNum, I: IntoIterator<Item = X>, Fmt: TickFmt<X>> TickDistribution<I, Fmt> {
        pub fn with_tick_fmt<F: Fn(&X) -> J, J: fmt::Display>(
            self,
            func: F,
        ) -> TickDistribution<I, WithTickFmt<Fmt, F>> {
            TickDistribution {
                iter: self.iter,
                fmt: WithTickFmt {
                    ticks: self.fmt,
                    func,
                },
                res: self.res,
            }
        }

        pub fn with_where_fmt<F: Fn() -> J, J: fmt::Display>(
            self,
            func: F,
        ) -> TickDistribution<I, WithWhereFmt<Fmt, F>> {
            TickDistribution {
                iter: self.iter,
                fmt: WithWhereFmt {
                    ticks: self.fmt,
                    func,
                },
                res: self.res,
            }
        }

        pub fn with_data<E>(self, data: E) -> TickDistribution<I, WithData<Fmt, E>> {
            TickDistribution {
                iter: self.iter,
                fmt: WithData {
                    ticks: self.fmt,
                    data,
                },
                res: self.res,
            }
        }

        pub fn with_fmt<J: TickFmt<I::Item>>(self, other: J) -> TickDistribution<I, J> {
            TickDistribution {
                iter: self.iter,
                fmt: other,
                res: self.res,
            }
        }
    }
}

impl<X: PlotNum, I: IntoIterator<Item = X>, Fmt: TickFmt<X>> TickDistribution<I, Fmt> {
    pub fn from_parts(it: I, fmt: Fmt, res: TickRes) -> Self {
        TickDistribution { iter: it, fmt, res }
    }
    pub fn map<K, F: FnOnce(Self) -> K>(self, func: F) -> K {
        func(self)
    }
}
///
/// Create a [`TickDistGen`] from a closure.
///
pub fn from_closure<N: PlotNum, It, F>(func: F) -> GenTickDistClosure<F>
where
    It: TickDist<Num = N>,
    F: FnOnce(&DataBound<N>, &RenderOptionsBound, IndexRequester) -> It,
{
    GenTickDistClosure { func }
}

pub struct GenTickDistClosure<F> {
    func: F,
}

impl<N: PlotNum, Res: TickDist<Num = N>, F> TickDistGen<N> for GenTickDistClosure<F>
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
pub trait TickDistGen<N> {
    type Res: TickDist<Num = N>;
    fn generate(
        self,
        data: &ticks::DataBound<N>,
        canvas: &RenderOptionsBound,
        req: IndexRequester,
    ) -> Self::Res;
}

pub fn gen_ticks<N: PlotNum, G: TickDistGen<N>>(
    gen: G,
    data: &ticks::DataBound<N>,
    opt: &RenderOptionsBound,
    req: IndexRequester,
) -> G::Res {
    gen.generate(data, opt, req)
}

pub trait TickDist {
    type Num;
    type It: IntoIterator<Item = Self::Num>;
    type Fmt: TickFmt<Self::Num>;
    fn unwrap(self) -> TickDistribution<Self::It, Self::Fmt>;
}

impl<I: IntoIterator, F: TickFmt<I::Item>> TickDist for TickDistribution<I, F> {
    type Num = I::Item;
    type It = I;
    type Fmt = F;
    fn unwrap(self) -> TickDistribution<Self::It, Self::Fmt> {
        self
    }
}

pub struct TickDistribution<I, F> {
    pub iter: I,
    pub fmt: F,
    pub res: TickRes,
}

pub fn from_iter<I: IntoIterator<Item = X>, X: PlotNum + fmt::Display>(
    it: I,
) -> TickDistribution<I, DefaultTickFmt> {
    TickDistribution::new(it)
}
impl<X: PlotNum, I: IntoIterator<Item = X>> TickDistribution<I, DefaultTickFmt>
where
    X: fmt::Display,
{
    pub fn new(it: I) -> Self {
        Self::from_parts(it, DefaultTickFmt, TickRes { dash_size: None })
    }
}

impl<X: PlotNum, I: IntoIterator<Item = X>, Fmt: TickFmt<X>> TickDistGen<X>
    for TickDistribution<I, Fmt>
{
    type Res = Self;
    fn generate(self, _: &ticks::DataBound<X>, _: &RenderOptionsBound, _: IndexRequester) -> Self {
        self
    }
}
