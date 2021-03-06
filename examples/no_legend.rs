use poloto::prelude::*;
fn main() -> core::fmt::Result {
    let mut s = poloto::plot_io(std::io::stdout());

    // TEST 3
    let data = [[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];

    s.scatter(wr!(""), data.iter().map(|x| *x));

    s.render(wr!("Cows Per Year"), wr!("Year"), wr!("Cow"))?;

    Ok(())
}
