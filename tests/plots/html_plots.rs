use super::*;

use poloto::build::plot;
#[test]
fn custom_colors_html() -> fmt::Result {
    let x: Vec<_> = (0..50).map(|x| (x as f64 / 50.0) * 10.0).collect();

    let l1 = plot("cos").line(x.iter().copied().zip_output(|x| x.cos()));
    let l2 = plot("sin-10").histogram(x.iter().copied().step_by(3).map(|x| [x, x.sin() - 10.]));

    let s = poloto::data(poloto::plots!(l1, l2)).build_and_label((
        "Demo: you can use CSS patterns if you embed SVG!",
        "x",
        "y",
    ));

    let graph = s.headless().render_string().unwrap();

    let mut w = util::create_test_file("custom_colors.html");

    let b = hypermelon::elem::BufferedElem::new(poloto::header(), hypermelon::render::NoFmt)?;

    let light = poloto::render::Theme::light().get_str().replace('\n', "");
    let dark = poloto::render::Theme::dark().get_str().replace('\n', "");

    let (header_start, header_end) = b.into_parts();
    write!(
        w,
        r###"
        <html>
        <head>
        <script>
            var flip=false;
            function monclick(){{
                var foo = document.getElementById("mystyle");
                if (flip){{
                    foo.innerHTML="{0}";
                }}else{{
                    foo.innerHTML="{1}";
                }}
                flip=!flip;
            }}
        </script>
        <style id="mystyle">
        {0}
        </style>
        <style>
        body {{
            background-color: coral;
        }}
        </style>
        </head> 
        <button type="button" style="font-size: 24px;" onclick="monclick();">Change Color Scheme</button>
        <div id="test">
        {2}
        </div>
        </htmls>
        "###,
        light,
        dark,
        format_args!("{}{}{}", header_start, graph, header_end)
    )
}

#[test]
fn hover_shadow() -> fmt::Result {
    const HEADER: &'static str = r###"
<head>
<svg width=0 height=0>
<defs>
<filter id="dropshadow" height="130%">
  <feGaussianBlur in="SourceAlpha" stdDeviation="3"/> <!-- stdDeviation is how much to blur -->
  <feOffset dx="2" dy="2" result="offsetblur"/> <!-- how much to offset -->
  <feComponentTransfer>
    <feFuncA type="linear" slope="0.5"/> <!-- slope is the opacity of the shadow -->
  </feComponentTransfer>
  <feMerge> 
    <feMergeNode/> <!-- this contains the offset blurred image -->
    <feMergeNode in="SourceGraphic"/> <!-- this contains the element that the filter is applied to -->
  </feMerge>
</defs>
</svg>
<style>
body {
    background-color: coral;
}
@keyframes gelatine {
    from, to { transform: scale(1, 1); }
    25% { transform: scale(0.9, 1.1); }
    50% { transform: scale(1.1, 0.9); }
    75% { transform: scale(0.95, 1.05); }
  }
.poloto{
    
    text-shadow: 2px 2px 5px black;
}
.poloto0stroke:hover{
    stroke:black;
    stroke-width:3;
}
.poloto1fill{
    animation: gelatine 3.0s infinite;
    transform-origin: center;
    filter:url(#dropshadow);
}
.poloto1fill:hover{
    
    stroke:black;
    stroke-width:3;
}

.poloto2fill:hover{

    stroke:black;
    stroke-width:3;
}
.poloto2fill{
    filter:url(#dropshadow);
}

</style>
</head> 
"###;

    let x: Vec<_> = (0..50).map(|x| (x as f64 / 50.0) * 10.0).collect();

    let s = poloto::plots!(
        plot("cos").line(x.iter().copied().zip_output(f64::cos)),
        plot("sin-3").histogram(x.iter().copied().step_by(3).zip_output(|x| x.sin() - 3.)),
        plot("sin").scatter(x.iter().copied().step_by(3).zip_output(|x| x.sin()))
    );

    let s = poloto::data(s)
        .build_and_label(("Demo: Hovering and shadows", "x", "y"))
        .append_to(poloto::header().light_theme());

    let s = s.render_string().unwrap();

    let mut w = util::create_test_file("hover_shadow.html");

    write!(
        w,
        r###"
<html>
{0}
<div id="test" class="navy">
{1}
</div>
</html>
        "###,
        HEADER, s
    )
}
