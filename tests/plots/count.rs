
#[test]
fn test_buffered_cloned_count() {
    use std::cell::Cell;

    let clone_counter = Cell::new(0);
    let a = (0i128..100)
        .map(|x| {
            clone_counter.set(clone_counter.take() + 1);
            [x, x]
        })
        .cloned_plot()
        .line("σ=1.0");

    let buffered_counter = Cell::new(0);
    let b = (0i128..100)
        .map(|x| {
            buffered_counter.set(buffered_counter.take() + 1);
            [x, x]
        })
        .buffered_plot()
        .line("σ=1.0");

    assert_eq!(clone_counter.get(), 0);
    assert_eq!(buffered_counter.get(), 0);

    let p = quick_fmt!("gaussian", "x", "y", a, b);

    assert_eq!(clone_counter.get(), 100);
    assert_eq!(buffered_counter.get(), 100);

    let mut s = String::new();
    p.render(&mut s).unwrap();

    assert_eq!(clone_counter.get(), 200);
    assert_eq!(buffered_counter.get(), 100);
}
