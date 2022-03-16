use super::*;

pub fn data_dyn<F: Flop>() -> DataDyn<F> {
    DataDyn::new()
}

pub struct DataDyn<F: Flop> {
    bound_counter: usize,
    plot_counter: usize,
    flop: Vec<F>,
}
impl<F: Flop> Default for DataDyn<F> {
    fn default() -> Self {
        Self::new()
    }
}
impl<F: Flop> DataDyn<F> {
    pub fn new() -> Self {
        DataDyn {
            bound_counter: 0,
            plot_counter: 0,
            flop: vec![],
        }
    }
    pub fn add(&mut self, a: F) -> &mut Self {
        self.flop.push(a);
        self
    }
}

impl<F: Flop> Flop for DataDyn<F> {
    type X = F::X;
    type Y = F::Y;
    fn next_bound(&mut self) -> Option<(Self::X, Self::Y)> {
        loop {
            if self.bound_counter >= self.flop.len() {
                return None;
            }
            if let Some(a) = self.flop[self.bound_counter].next_bound() {
                return Some(a);
            }
            self.bound_counter += 1;
        }
    }
    fn next_typ(&mut self) -> Option<PlotMetaType> {
        if self.plot_counter >= self.flop.len() {
            None
        } else {
            self.flop[self.plot_counter].next_typ()
        }
    }
    fn next_plot(&mut self) -> Option<PlotSesh<(Self::X, Self::Y)>> {
        let a = self.flop[self.plot_counter].next_plot();
        if let Some(PlotSesh::None) = a {
            self.plot_counter += 1;
        }
        a
    }

    fn next_name<W: fmt::Write>(&mut self, write: W) -> Option<fmt::Result> {
        self.flop[self.plot_counter].next_name(write)
    }
}


///
/// Renderer will first call next_bound() until exhausted in order to find min/max bounds.
/// 
/// Then renderer will call next_typ() to determine  if there is a plot
///     if next_typ() returned Some(), then it will then call next_name()
///       and expect there to be a name. Then it will call next_plot continuously
///         untill exausted.
/// 
pub trait Flop {
    type X: PlotNum;
    type Y: PlotNum;
    fn next_bound(&mut self) -> Option<(Self::X, Self::Y)>;
    fn next_plot(&mut self) -> Option<PlotSesh<(Self::X, Self::Y)>>;
    fn next_name<W: fmt::Write>(&mut self, w: W) -> Option<fmt::Result>;
    fn next_typ(&mut self) -> Option<PlotMetaType>;

    fn chain<B: Flop>(self, b: B) -> Chain<Self, B>
    where
        Self: Sized,
    {
        Chain {
            a: self,
            b,
            started: false,
        }
    }

    fn text<D: Display>(self, name: D) -> Chain<Self, OneP<std::iter::Empty<(Self::X, Self::Y)>, D>>
    where
        Self: Sized,
    {
        self.chain(OneP::new(PlotMetaType::Text, name, std::iter::empty()))
    }

    fn xmarker(self, val: Self::X) -> XMarker<Self>
    where
        Self: Sized,
    {
        XMarker {
            val: Some(val),
            foo: self,
            store: None,
        }
    }
    fn ymarker(self, val: Self::Y) -> YMarker<Self>
    where
        Self: Sized,
    {
        YMarker {
            val: Some(val),
            foo: self,
            store: None,
        }
    }
    fn collect(mut self) -> Data<Self::X, Self::Y, Self>
    where
        Self: Sized,
    {
        let ii = std::iter::from_fn(|| self.next_bound());

        let (boundx, boundy) = util::find_bounds(ii);

        let boundx = DataBound {
            min: boundx[0],
            max: boundx[1],
        };
        let boundy = DataBound {
            min: boundy[0],
            max: boundy[1],
        };

        Data {
            boundx,
            boundy,
            plots: self,
        }
    }
}

pub struct Flopp<A: Flop> {
    flop: A,
}
impl<A: Flop> Flopp<A> {
    pub fn new(flop: A) -> Self {
        Flopp { flop }
    }
    pub fn next_plot(&mut self) -> Option<FlopIterator<A>> {
        if let Some(typ) = self.flop.next_typ() {
            Some(FlopIterator {
                typ,
                flop: &mut self.flop,
            })
        } else {
            None
        }
    }
}

pub struct FlopIterator<'a, A: Flop> {
    typ: PlotMetaType,
    flop: &'a mut A,
}
impl<'b, A: Flop> FlopIterator<'b, A> {
    pub fn typ(&mut self) -> PlotMetaType {
        self.typ
    }
    pub fn name<F: fmt::Write>(&mut self, write: F) -> fmt::Result {
        self.flop.next_name(write).unwrap()
    }
    pub fn plots<'a>(&'a mut self) -> impl Iterator<Item = (A::X, A::Y)> + 'a {
        std::iter::from_fn(|| {
            if let Some(PlotSesh::Some(a)) = self.flop.next_plot() {
                Some(a)
            } else {
                None
            }
        })
    }
}

pub enum PlotSesh<T> {
    Some(T),
    None,
}

pub fn bars<X: PlotNum, Y: PlotNum, I: PlotIter, D: Display>(name: D, it: I) -> OneP<I, D>
where
    I::Item1: Plottable<Item = (X, Y)>,
    I::Item2: Plottable<Item = (X, Y)>,
{
    OneP::new(PlotMetaType::Plot(PlotType::Bars), name, it)
}

pub fn histogram<X: PlotNum, Y: PlotNum, I: PlotIter, D: Display>(name: D, it: I) -> OneP<I, D>
where
    I::Item1: Plottable<Item = (X, Y)>,
    I::Item2: Plottable<Item = (X, Y)>,
{
    OneP::new(PlotMetaType::Plot(PlotType::Histo), name, it)
}

pub fn scatter<X: PlotNum, Y: PlotNum, I: PlotIter, D: Display>(name: D, it: I) -> OneP<I, D>
where
    I::Item1: Plottable<Item = (X, Y)>,
    I::Item2: Plottable<Item = (X, Y)>,
{
    OneP::new(PlotMetaType::Plot(PlotType::Scatter), name, it)
}

pub fn line_fill<X: PlotNum, Y: PlotNum, I: PlotIter, D: Display>(name: D, it: I) -> OneP<I, D>
where
    I::Item1: Plottable<Item = (X, Y)>,
    I::Item2: Plottable<Item = (X, Y)>,
{
    OneP::new(PlotMetaType::Plot(PlotType::LineFill), name, it)
}

pub fn line_fill_raw<X: PlotNum, Y: PlotNum, I: PlotIter, D: Display>(name: D, it: I) -> OneP<I, D>
where
    I::Item1: Plottable<Item = (X, Y)>,
    I::Item2: Plottable<Item = (X, Y)>,
{
    OneP::new(PlotMetaType::Plot(PlotType::LineFillRaw), name, it)
}

pub fn line<X: PlotNum, Y: PlotNum, I: PlotIter, D: Display>(name: D, it: I) -> OneP<I, D>
where
    I::Item1: Plottable<Item = (X, Y)>,
    I::Item2: Plottable<Item = (X, Y)>,
{
    OneP::new(PlotMetaType::Plot(PlotType::Line), name, it)
}

pub struct XMarker<F: Flop> {
    val: Option<F::X>,
    foo: F,
    store: Option<(F::X, F::Y)>,
}
impl<F: Flop> Flop for XMarker<F> {
    type X = F::X;
    type Y = F::Y;
    fn next_bound(&mut self) -> Option<(Self::X, Self::Y)> {
        if let Some(a) = self.store.take() {
            Some(a)
        } else if let Some(val) = self.val.take() {
            if let Some((x, y)) = self.foo.next_bound() {
                if !y.is_hole() {
                    self.store = Some((x, y));
                    Some((val, y))
                } else {
                    Some((x, y))
                }
            } else {
                None
            }
        } else {
            self.foo.next_bound()
        }
    }

    fn next_plot(&mut self) -> Option<PlotSesh<(Self::X, Self::Y)>> {
        self.foo.next_plot()
    }

    fn next_name<W: fmt::Write>(&mut self, w: W) -> Option<fmt::Result> {
        self.foo.next_name(w)
    }

    fn next_typ(&mut self) -> Option<PlotMetaType> {
        self.foo.next_typ()
    }
}

pub struct YMarker<F: Flop> {
    val: Option<F::Y>,
    foo: F,
    store: Option<(F::X, F::Y)>,
}
impl<F: Flop> Flop for YMarker<F> {
    type X = F::X;
    type Y = F::Y;
    fn next_bound(&mut self) -> Option<(Self::X, Self::Y)> {
        if let Some(a) = self.store.take() {
            Some(a)
        } else if let Some(val) = self.val.take() {
            if let Some((x, y)) = self.foo.next_bound() {
                if !x.is_hole() {
                    self.store = Some((x, y));
                    Some((x, val))
                } else {
                    Some((x, y))
                }
            } else {
                None
            }
        } else {
            self.foo.next_bound()
        }
    }

    fn next_plot(&mut self) -> Option<PlotSesh<(Self::X, Self::Y)>> {
        self.foo.next_plot()
    }

    fn next_name<W: fmt::Write>(&mut self, w: W) -> Option<fmt::Result> {
        self.foo.next_name(w)
    }

    fn next_typ(&mut self) -> Option<PlotMetaType> {
        self.foo.next_typ()
    }
}

pub struct OneP<I: PlotIter, D: Display> {
    buffer1: Option<I::It1>, //todo replace two options with one enum
    buffer2: Option<I::It2>,
    plots: Option<I>,
    name: D,
    typ: PlotMetaType,
    hit_end: bool,
    started: bool,
}
impl<I: PlotIter, D: Display> OneP<I, D>
where
    I::Item1: Plottable,
    I::Item2: Plottable,
{
    fn new(typ: PlotMetaType, name: D, plots: I) -> Self {
        OneP {
            buffer1: None,
            buffer2: None,
            plots: Some(plots),
            name,
            typ,
            hit_end: false,
            started: false,
        }
    }
}
impl<X: PlotNum, Y: PlotNum, I: PlotIter, D: Display> Flop for OneP<I, D>
where
    I::Item1: Plottable<Item = (X, Y)>,
    I::Item2: Plottable<Item = (X, Y)>,
{
    type X = X;
    type Y = Y;
    fn next_bound(&mut self) -> Option<(Self::X, Self::Y)> {
        if self.buffer1.is_none() {
            self.buffer1 = Some(self.plots.as_mut().unwrap().first());
        }

        self.buffer1.as_mut().unwrap().next().map(|x| x.make_plot())
    }

    fn next_plot(&mut self) -> Option<PlotSesh<(Self::X, Self::Y)>> {
        if let Some(d) = self.buffer1.take() {
            self.buffer2 = Some(self.plots.take().unwrap().second(d));
        }

        if let Some(bb) = self.buffer2.as_mut() {
            if let Some(k) = bb.next() {
                Some(PlotSesh::Some(k.make_plot()))
            } else if !self.hit_end {
                self.hit_end = true;
                Some(PlotSesh::None)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn next_name<W: fmt::Write>(&mut self, mut writer: W) -> Option<fmt::Result> {
        Some(write!(writer, "{}", self.name))
    }

    fn next_typ(&mut self) -> Option<PlotMetaType> {
        if !self.started {
            self.started = true;
            Some(self.typ)
        } else {
            None
        }
    }
}

pub struct Chain<A, B> {
    a: A,
    b: B,
    started: bool,
}
impl<A: Flop, B: Flop<X = A::X, Y = A::Y>> Flop for Chain<A, B> {
    type X = A::X;
    type Y = A::Y;
    fn next_bound(&mut self) -> Option<(Self::X, Self::Y)> {
        if let Some(a) = self.a.next_bound() {
            Some(a)
        } else {
            self.b.next_bound()
        }
    }

    fn next_plot(&mut self) -> Option<PlotSesh<(Self::X, Self::Y)>> {
        if let Some(a) = self.a.next_plot() {
            Some(a)
        } else {
            self.b.next_plot()
        }
    }

    fn next_name<W: fmt::Write>(&mut self, mut writer: W) -> Option<fmt::Result> {
        if !self.started {
            self.a.next_name(&mut writer)
        } else {
            self.b.next_name(&mut writer)
        }
    }
    fn next_typ(&mut self) -> Option<PlotMetaType> {
        if let Some(a) = self.a.next_typ() {
            Some(a)
        } else {
            self.started = true;
            self.b.next_typ()
        }
    }
}

/*


impl<'a, X: PlotNum + 'a, Y: PlotNum + 'a> DataBuilder<'a, X, Y> {
    pub fn xmarker(&mut self, a: X) -> &mut Self {
        self.xmarkers.push(a);
        self
    }

    pub fn ymarker(&mut self, a: Y) -> &mut Self {
        self.ymarkers.push(a);
        self
    }

    ///
    /// Write some text in the legend. This doesnt increment the plot number.
    ///
    /// ```
    /// let mut plotter = poloto::data::<f64,f64>();
    /// plotter.text("This is a note");
    /// ```
    pub fn text(&mut self, name: impl Display + 'a) -> &mut Self {
        self.plots.push(Box::new(PlotStruct::new(
            std::iter::empty(),
            name,
            PlotMetaType::Text,
        )));
        self
    }

    /// Create a line from plots using a SVG path element.
    /// The path element belongs to the `.poloto[N]fill` css class.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::data();
    /// plotter.line("", &data);
    /// ```
    pub fn line<I>(&mut self, name: impl Display + 'a, plots: I) -> &mut Self
    where
        I: PlotIter + 'a,
        I::Item1: Plottable<Item = (X, Y)>,
        I::Item2: Plottable<Item = (X, Y)>,
    {
        self.plots.push(Box::new(PlotStruct::new(
            plots.map_plot(|x| x.make_plot(), |x| x.make_plot()),
            name,
            PlotMetaType::Plot(PlotType::Line),
        )));
        self
    }

    /// Create a line from plots that will be filled underneath using a SVG path element.
    /// The path element belongs to the `.poloto[N]fill` css class.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::data();
    /// plotter.line_fill("", &data);
    /// ```
    pub fn line_fill<I>(&mut self, name: impl Display + 'a, plots: I) -> &mut Self
    where
        I: PlotIter + 'a,
        I::Item1: Plottable<Item = (X, Y)>,
        I::Item2: Plottable<Item = (X, Y)>,
    {
        self.plots.push(Box::new(PlotStruct::new(
            plots.map_plot(|x| x.make_plot(), |x| x.make_plot()),
            name,
            PlotMetaType::Plot(PlotType::LineFill),
        )));
        self
    }

    /// Create a line from plots that will be filled using a SVG path element.
    /// The first and last points will be connected and then filled in.
    /// The path element belongs to the `.poloto[N]fill` css class.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::data();
    /// plotter.line_fill_raw("", &data);
    /// ```
    pub fn line_fill_raw<I>(&mut self, name: impl Display + 'a, plots: I) -> &mut Self
    where
        I: PlotIter + 'a,
        I::Item1: Plottable<Item = (X, Y)>,
        I::Item2: Plottable<Item = (X, Y)>,
    {
        self.plots.push(Box::new(PlotStruct::new(
            plots.map_plot(|x| x.make_plot(), |x| x.make_plot()),
            name,
            PlotMetaType::Plot(PlotType::LineFillRaw),
        )));
        self
    }

    /// Create a scatter plot from plots, using a SVG path with lines with zero length.
    /// Each point can be sized using the stroke width.
    /// The path belongs to the CSS classes `poloto_scatter` and `.poloto[N]stroke` css class
    /// with the latter class overriding the former.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::data();
    /// plotter.scatter("", &data);
    /// ```
    pub fn scatter<I>(&mut self, name: impl Display + 'a, plots: I) -> &mut Self
    where
        I: PlotIter + 'a,
        I::Item1: Plottable<Item = (X, Y)>,
        I::Item2: Plottable<Item = (X, Y)>,
    {
        self.plots.push(Box::new(PlotStruct::new(
            plots.map_plot(|x| x.make_plot(), |x| x.make_plot()),
            name,
            PlotMetaType::Plot(PlotType::Scatter),
        )));
        self
    }

    /// Create a histogram from plots using SVG rect elements.
    /// Each bar's left side will line up with a point.
    /// Each rect element belongs to the `.poloto[N]fill` css class.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::data();
    /// plotter.histogram("", &data);
    /// ```
    pub fn histogram<I>(&mut self, name: impl Display + 'a, plots: I) -> &mut Self
    where
        I: PlotIter + 'a,
        I::Item1: Plottable<Item = (X, Y)>,
        I::Item2: Plottable<Item = (X, Y)>,
    {
        self.plots.push(Box::new(PlotStruct::new(
            plots.map_plot(|x| x.make_plot(), |x| x.make_plot()),
            name,
            PlotMetaType::Plot(PlotType::Histo),
        )));
        self
    }

    pub fn move_into(&mut self) -> Self {
        let mut val = DataBuilder {
            plots: vec![],
            xmarkers: vec![],
            ymarkers: vec![],
        };

        std::mem::swap(&mut val, self);
        val
    }

    /*
    ///
    /// Compute min/max bounds and prepare for next stage in pipeline.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::data();
    /// plotter.line("", &data);
    /// plotter.build();
    /// ```
    ///
    pub fn build(&mut self) -> Data<X, Y, impl AllPlotFmt<Item = (X, Y)> + 'a> {
        let mut val = self.move_into();

        let (boundx, boundy) = util::find_bounds(
            val.plots.iter_mut().flat_map(|x| x.iter_first()),
            val.xmarkers.clone(),
            val.ymarkers.clone(),
        );

        let boundx = DataBound {
            min: boundx[0],
            max: boundx[1],
        };
        let boundy = DataBound {
            min: boundy[0],
            max: boundy[1],
        };

        Data {
            plots: Foo2 { plots: val.plots },
            boundx,
            boundy,
        }
    }
    */
}
*/

impl<X: PlotNum, Y: PlotNum, P: Flop<X = X, Y = Y>> Data<X, Y, P> {
    pub fn data_boundx(&self) -> &DataBound<X> {
        &self.boundx
    }
    pub fn data_boundy(&self) -> &DataBound<Y> {
        &self.boundy
    }

    pub fn boundx<'b>(&'b self, canvas: &'b Canvas) -> Bound<'b, X> {
        Bound {
            data: &self.boundx,
            canvas: canvas.boundx(),
        }
    }
    pub fn boundy<'b>(&'b self, canvas: &'b Canvas) -> Bound<'b, Y> {
        Bound {
            data: &self.boundy,
            canvas: canvas.boundy(),
        }
    }

    pub fn stage(self) -> Stager<X, Y, P, Canvas> {
        Stager {
            res: self,
            canvas: crate::canvas().build(),
        }
    }

    pub fn stage_with<K: Borrow<Canvas>>(self, canvas: K) -> Stager<X, Y, P, K> {
        Stager { res: self, canvas }
    }
}

impl<X: PlotNum, Y: PlotNum, P: Flop<X = X, Y = Y>, K: Borrow<Canvas>> Stager<X, Y, P, K> {
    ///
    /// Automatically create a tick distribution using the default
    /// tick generators tied to a [`PlotNum`].
    ///

    ///
    /// Automatically create a tick distribution using the default
    /// tick generators tied to a [`PlotNum`].
    ///
    pub fn plot(
        self,
        title: impl Display,
        xname: impl Display,
        yname: impl Display,
    ) -> Plotter<impl Disp>
    where
        X: HasDefaultTicks,
        Y: HasDefaultTicks,
    {
        let (x, xt) = ticks_from_default(self.res.boundx(self.canvas.borrow()));
        let (y, yt) = ticks_from_default(self.res.boundy(self.canvas.borrow()));

        let p = plot_fmt(title, xname, yname, xt, yt);
        self.plot_with(x, y, p)
    }

    ///
    /// Move to final stage in pipeline collecting the title/xname/yname.
    /// Unlike [`Stager::plot`] User must supply own tick distribution.
    ///
    pub fn plot_with<XI, YI, PF>(
        self,
        xtick: TickInfo<XI>,
        ytick: TickInfo<YI>,
        plot_fmt: PF,
    ) -> Plotter<impl Disp>
    where
        XI: IntoIterator<Item = X>,
        YI: IntoIterator<Item = Y>,
        PF: BaseFmt<X = X, Y = Y>,
    {
        ///
        /// Wrap tick iterators and a [`PlotFmt`] behind the [`PlotFmtAll`] trait.
        ///
        struct PlotAllStruct<XI: IntoIterator, YI: IntoIterator, PF: BaseFmt> {
            xtick: TickInfo<XI>,
            ytick: TickInfo<YI>,
            fmt: PF,
        }

        impl<XI: IntoIterator, YI: IntoIterator, PF: BaseFmt<X = XI::Item, Y = YI::Item>>
            BaseFmtAndTicks for PlotAllStruct<XI, YI, PF>
        where
            XI::Item: PlotNum,
            YI::Item: PlotNum,
        {
            type X = PF::X;
            type Y = PF::Y;
            type Fmt = PF;
            type XI = XI;
            type YI = YI;

            fn gen(self) -> (Self::Fmt, TickInfo<Self::XI>, TickInfo<Self::YI>) {
                (self.fmt, self.xtick, self.ytick)
            }
        }

        self.plot_with_all(PlotAllStruct {
            fmt: plot_fmt,
            xtick,
            ytick,
        })
    }

    ///
    /// Create a plotter directly from a [`BaseFmtAndTicks`]
    ///
    fn plot_with_all<PF: BaseFmtAndTicks<X = X, Y = Y>>(self, p: PF) -> Plotter<impl Disp> {
        struct Combine<A: BaseFmtAndTicks, B: Flop> {
            pub a: A,
            pub b: B,
        }

        impl<A: BaseFmtAndTicks, B: Flop<X = A::X, Y = A::Y>> BaseAndPlotsFmt for Combine<A, B> {
            type X = A::X;
            type Y = A::Y;
            type A = A;
            type B = B;
            fn gen(self) -> (Self::A, Self::B) {
                (self.a, self.b)
            }
        }

        struct InnerPlotter<PF: BaseAndPlotsFmt, K: Borrow<Canvas>> {
            all: PF,
            boundx: DataBound<PF::X>,
            boundy: DataBound<PF::Y>,
            canvas: K,
        }

        impl<PF: BaseAndPlotsFmt, K: Borrow<Canvas>> Disp for InnerPlotter<PF, K> {
            fn disp<T: std::fmt::Write>(self, mut writer: T) -> fmt::Result {
                render::render(&mut writer, self.all, self.boundx, self.boundy, self.canvas)
            }
        }

        let pp = InnerPlotter {
            all: Combine {
                a: p,
                b: self.res.plots,
            },
            boundx: self.res.boundx,
            boundy: self.res.boundy,
            canvas: self.canvas,
        };

        let dim = pp.canvas.borrow().get_dim();
        Plotter {
            inner: Some(pp),
            dim,
        }
    }
}
