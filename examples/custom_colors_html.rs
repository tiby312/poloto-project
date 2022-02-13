use poloto::prelude::*;
fn main() {
    let mut s = poloto::data();
    let x = (0..50).map(|x| (x as f64 / 50.0) * 10.0);

    s.line("cos", x.clone().map(|x| [x, x.cos()]));
    s.histogram("sin-10", x.clone().step_by(3).map(|x| [x, x.sin() - 10.]));

    let mut s = s
        .build()
        .plot("Demo: you can use CSS patterns if you embed SVG!", "x", "y");

    println!(
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
        poloto::simple_theme::STYLE_CONFIG_LIGHT_DEFAULT,
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        format_args!(
            "{}{}{}",
            poloto::simple_theme::SVG_HEADER,
            poloto::disp(|a| s.render(a)),
            poloto::simple_theme::SVG_END
        )
    );
}
