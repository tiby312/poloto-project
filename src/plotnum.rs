//!
//! Contains the [`PlotNum`] and [`PlotNumContext`] traits and their supporting structs.
//!
///
/// A disconnectable number. A number that can me marked as a hole to signify that there is a disconnect in plots.
/// See [`Croppable`]
///
pub trait DiscNum: PlotNum {
    /// Create a hole value.
    fn hole() -> Self;
}






pub trait PlotNumContext {
    type Num: PlotNum;

    ///
    /// Given an ideal number of intervals across the min and max values,
    /// Calculate information related to where the interval ticks should go.
    /// Guarenteed to be called before fmt_tick.
    ///
    fn compute_ticks(
        &mut self,
        ideal_num_steps: u32,
        range: [Self::Num; 2],
        dash: DashInfo,
    ) -> TickInfo<Self::Num>;

    /// If there is only one point in a graph, or no point at all,
    /// the range to display in the graph.
    fn unit_range(&mut self, offset: Option<Self::Num>) -> [Self::Num; 2];

    /// Provided a min and max range, scale the current value against max.
    fn scale(&mut self, val: Self::Num, range: [Self::Num; 2], max: f64) -> f64;

    /// Used to display a tick
    /// Before overriding this, consider using [`crate::Plotter::xinterval_fmt`] and [`crate::Plotter::yinterval_fmt`].
    fn fmt_tick(&mut self, tick: TickFmt<Self::Num>) -> std::fmt::Result {
        write!(tick.writer, "{}", tick.val)
    }

    fn fmt_name(&mut self, _info: NameInfo<Self::Num>) -> std::fmt::Result {
        Ok(())
    }

    fn ideal_num_ticks(&mut self) -> Option<u32> {
        None
    }

    fn get_markers(&mut self) -> Vec<Self::Num> {
        vec![]
    }
}

pub struct NameInfo<'a, T: PlotNum> {
    pub writer: &'a mut dyn std::fmt::Write,
    pub min: T,
    pub max: T,
    pub step: T::StepInfo,
}

///
/// A plottable number that has a default context that can be created by calling `ctx()`
///
pub trait HasDefaultCtx: PlotNum {
    type DefaultContext: PlotNumContext<Num = Self> + Default;

    fn ctx<K: std::fmt::Display>(name: K) -> WithName<K, Self::DefaultContext> {
        WithName {
            name,
            it: Self::DefaultContext::default(),
        }
    }

    fn ctx_fmt<K>(name: K) -> WithNameFunc<K, Self::DefaultContext>
    where
        K: FnMut(NameInfo<Self>) -> std::fmt::Result,
    {
        WithNameFunc {
            name: Some(name),
            it: Self::DefaultContext::default(),
        }
    }

    fn ctx_none() -> Self::DefaultContext {
        Self::DefaultContext::default()
    }
}

pub struct WithName<K: std::fmt::Display, J: PlotNumContext> {
    name: K,
    it: J,
}

impl<K: std::fmt::Display, J: PlotNumContext> PlotNumContext for WithName<K, J> {
    type Num = J::Num;

    ///
    /// Given an ideal number of intervals across the min and max values,
    /// Calculate information related to where the interval ticks should go.
    ///
    fn compute_ticks(
        &mut self,
        ideal_num_steps: u32,
        range: [Self::Num; 2],
        dash: DashInfo,
    ) -> TickInfo<Self::Num> {
        self.it.compute_ticks(ideal_num_steps, range, dash)
    }

    /// If there is only one point in a graph, or no point at all,
    /// the range to display in the graph.
    fn unit_range(&mut self, offset: Option<Self::Num>) -> [Self::Num; 2] {
        self.it.unit_range(offset)
    }

    /// Provided a min and max range, scale the current value against max.
    fn scale(&mut self, val: Self::Num, range: [Self::Num; 2], max: f64) -> f64 {
        self.it.scale(val, range, max)
    }

    /// Used to display a tick
    /// Before overriding this, consider using [`crate::Plotter::xinterval_fmt`] and [`crate::Plotter::yinterval_fmt`].
    fn fmt_tick(&mut self, tick: TickFmt<Self::Num>) -> std::fmt::Result {
        self.it.fmt_tick(tick)
    }

    fn fmt_name(&mut self, info: NameInfo<Self::Num>) -> std::fmt::Result {
        write!(info.writer, "{}", self.name)
    }

    fn ideal_num_ticks(&mut self) -> Option<u32> {
        self.it.ideal_num_ticks()
    }

    fn get_markers(&mut self) -> Vec<Self::Num> {
        self.it.get_markers()
    }
}

pub struct TickFmt<'a, T: PlotNum> {
    pub writer: &'a mut dyn std::fmt::Write,
    pub val: T,
    pub step: FmtFull,
    pub info: T::StepInfo,
    pub bounds: [T; 2],
}

pub struct WithNameFunc<K: FnOnce(NameInfo<J::Num>) -> std::fmt::Result, J: PlotNumContext> {
    name: Option<K>,
    it: J,
}

impl<K: FnOnce(NameInfo<J::Num>) -> std::fmt::Result, J: PlotNumContext> PlotNumContext
    for WithNameFunc<K, J>
{
    type Num = J::Num;

    ///
    /// Given an ideal number of intervals across the min and max values,
    /// Calculate information related to where the interval ticks should go.
    ///
    fn compute_ticks(
        &mut self,
        ideal_num_steps: u32,
        range: [Self::Num; 2],
        dash: DashInfo,
    ) -> TickInfo<Self::Num> {
        self.it.compute_ticks(ideal_num_steps, range, dash)
    }

    /// If there is only one point in a graph, or no point at all,
    /// the range to display in the graph.
    fn unit_range(&mut self, offset: Option<Self::Num>) -> [Self::Num; 2] {
        self.it.unit_range(offset)
    }

    /// Provided a min and max range, scale the current value against max.
    fn scale(&mut self, val: Self::Num, range: [Self::Num; 2], max: f64) -> f64 {
        self.it.scale(val, range, max)
    }

    /// Used to display a tick
    /// Before overriding this, consider using [`crate::Plotter::xinterval_fmt`] and [`crate::Plotter::yinterval_fmt`].
    fn fmt_tick(&mut self, tick: TickFmt<Self::Num>) -> std::fmt::Result {
        self.it.fmt_tick(tick)
    }

    fn fmt_name(&mut self, info: NameInfo<Self::Num>) -> std::fmt::Result {
        (self.name.take().unwrap())(info)
    }

    fn ideal_num_ticks(&mut self) -> Option<u32> {
        self.it.ideal_num_ticks()
    }

    fn get_markers(&mut self) -> Vec<Self::Num> {
        self.it.get_markers()
    }
}

///
/// A plottable number. In order to be able to plot a number, we need information on how
/// to display it as well as the interval ticks.
///
pub trait PlotNum: PartialOrd + Copy + std::fmt::Display {
    type StepInfo: Copy;

    /// Is this a hole value to inject discontinuty?
    fn is_hole(&self) -> bool {
        false
    }


    /// Provided a min and max range, scale the current value against max.
    fn scale(val: Self, range: [Self; 2], max: f64) -> f64;

    ///
    /// Given an ideal number of intervals across the min and max values,
    /// Calculate information related to where the interval ticks should go.
    /// Guarenteed to be called before fmt_tick.
    ///
    fn compute_ticks(
        ideal_num_steps: u32,
        range: [Self; 2],
        dash: DashInfo,
    ) -> TickInfo<Self>;

    /// If there is only one point in a graph, or no point at all,
    /// the range to display in the graph.
    fn unit_range(offset: Option<Self>) -> [Self; 2];


    /// Default way to display the number
    fn val_fmt(&mut self,writer:&mut dyn std::fmt::Write,_tick: FmtFull,_extra:&mut Self::StepInfo) -> std::fmt::Result {
        write!(writer,"{}",self)
    } 
}

pub struct DashInfo {
    //The ideal dash size in the drawing area
    pub ideal_dash_size: f64,

    //The total drawing area
    pub max: f64,
}

pub enum FmtFull {
    Full,
    Tick,
}

///
/// One interval tick
///
#[derive(Debug, Clone, Copy)]
pub struct Tick<I> {
    pub position: I,
    /// If [`TickInfo::display_relative`] is `None`, then this has the same value as [`Tick::position`]
    pub value: I,
}

///
/// Information on the properties of all the interval ticks for one dimension.
///
#[derive(Debug, Clone)]
pub struct TickInfo<I: PlotNum> {
    pub unit_data: I::StepInfo,
    /// List of the position of each tick to be displayed.
    /// This must have a length of as least 2.
    pub ticks: Vec<Tick<I>>,

    /// The number of dashes between two ticks must be a multiple of this number.
    //pub dash_multiple: u32,
    pub dash_size: Option<f64>,

    /// If we want to display the tick values relatively, this will
    /// have the base start to start with.
    pub display_relative: Option<I>,
}
