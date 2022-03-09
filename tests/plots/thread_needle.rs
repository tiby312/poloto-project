///
/// Example where we pass a uncopiable/unclonable object to each formatting function.
///
use super::*;
use poloto::{plotnum::BaseFmt, plotnum::TickFormat};

struct Dummy;
impl fmt::Display for Dummy {
    fn fmt(&self, a: &mut fmt::Formatter) -> fmt::Result {
        write!(a, "##")
    }
}
struct Foo {
    dummy: Dummy,
    xtick_fmt: poloto::TickIterFmt<i128>,
    ytick_fmt: poloto::num::integer::IntegerTickFmt,
}

impl BaseFmt for Foo {
    type X = i128;
    type Y = i128;

    fn write_title(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result {
        write!(writer, "hello {}", self.dummy)
    }
    fn write_xname(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result {
        write!(writer, "hello {}", self.dummy)
    }
    fn write_yname(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result {
        write!(writer, "hello {}", self.dummy)
    }
    fn write_xwher(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result {
        self.xtick_fmt.write_where(writer)?;
        write!(writer, "{}", self.dummy)
    }
    fn write_ywher(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result {
        self.ytick_fmt.write_where(writer)?;
        write!(writer, "{}", self.dummy)
    }
    fn write_xtick(&mut self, writer: &mut dyn fmt::Write, val: &Self::X) -> fmt::Result {
        self.xtick_fmt.write_tick(writer, val)?;
        write!(writer, "{}", self.dummy)
    }
    fn write_ytick(&mut self, writer: &mut dyn fmt::Write, val: &Self::Y) -> fmt::Result {
        self.ytick_fmt.write_tick(writer, val)?;
        write!(writer, "{}", self.dummy)
    }
}

#[test]
fn thread_needle() -> fmt::Result {
    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data = [
        (2010, 3144000),
        (2011, 3518000),
        (2012, 3835000),
        (2013, 4133000),
        (2014, 4413000),
        (2015, 4682000),
        (2016, 5045000),
        (2017, 5321200),
        (2018, 5541900),
        (2019, 5773600),
        (2020, 5989400),
        (2021, 6219700),
        (2022, 0), //To complete our histogram, we manually specify when 2021 ends.
    ];

    let data = poloto::data().histogram("foo", data).ymarker(0).build();

    let (xtick, xtick_fmt) = poloto::ticks_from_iter((2010..).step_by(2));
    let (ytick, ytick_fmt) = poloto::ticks_from_default(data.boundy());

    let mut plotter = data.plot_with(
        xtick,
        ytick,
        Foo {
            dummy: Dummy,
            xtick_fmt,
            ytick_fmt,
        },
    );

    let mut w = util::create_test_file("thread_needle.svg");

    write!(
        w,
        "{}<style>{}{}</style>{}{}",
        poloto::simple_theme::SVG_HEADER,
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        ".poloto_line{stroke-dasharray:2;stroke-width:1;}",
        poloto::disp_mut(|w| plotter.render(w)),
        poloto::simple_theme::SVG_END
    )
}
