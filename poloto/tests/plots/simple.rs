
use super::*;
use poloto::build;
use poloto::build::plot;

#[test]
fn test_cloned_cloneable() {
    let data = [[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];

    let l1 = plot("").scatter(build::cloned(data.iter()));
    let l2 = plot("").scatter(data);
    let l = plots!(l1, l2);

    let p1 = poloto::frame_build()
        .data(l.clone())
        .build_and_label(("title", "x", "y"));
    let p2 = poloto::frame_build()
        .data(l)
        .build_and_label(("title", "x", "y"));

    let mut s1 = String::new();
    let mut s2 = String::new();

    p1.headless().render_fmt_write(&mut s1).unwrap();
    p2.headless().render_fmt_write(&mut s2).unwrap();

    assert_eq!(s1, s2);
}

#[test]
fn test_single_and_chain_and_dyn_cloneable() {
    let data = [[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];

    let l1 = plot("").scatter(build::cloned(data.iter()));
    let l2 = plot("").scatter(build::cloned(data.iter()));
    let l = plots!(l1, l2);

    let p1 = poloto::frame_build()
        .data(l.clone())
        .build_and_label(("title", "x", "y"));
    let p2 = poloto::frame_build()
        .data(l.clone())
        .build_and_label(("title", "x", "y"));

    let mut s1 = String::new();
    let mut s2 = String::new();

    p1.headless().render_fmt_write(&mut s1).unwrap();
    p2.headless().render_fmt_write(&mut s2).unwrap();

    assert_eq!(s1, s2);

    let l3 = vec![plot("").scatter(build::cloned(data))];

    let l = plots!(l, l3);

    let p1 = poloto::frame_build()
        .data(l.clone())
        .build_and_label(("title", "x", "y"));
    let p2 = poloto::frame_build()
        .data(l)
        .build_and_label(("title", "x", "y"));

    let mut s1 = String::new();
    let mut s2 = String::new();

    p1.headless().render_fmt_write(&mut s1).unwrap();
    p2.headless().render_fmt_write(&mut s2).unwrap();

    assert_eq!(s1, s2);
}
