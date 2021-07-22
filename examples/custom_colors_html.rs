fn main() -> core::fmt::Result {
    let mut s = poloto::plot_with_html(
        "Demo: you can use CSS patterns if you embed SVG!",
        "x",
        "y",
        poloto::HTML_CONFIG_CSS_VARIABLE_DEFAULT,
    );

    let x = (0..50).map(|x| (x as f32 / 50.0) * 10.0);

    s.line("cos", x.clone().map(|x| [x, x.cos()]));
    s.histogram("sin-10", x.clone().step_by(3).map(|x| [x, x.sin() - 10.]));

    let fs = s.render()?;

    println!(
        r###"
<html>
{0}
<button type="button" style="font-size: 24px;" onclick="monclick();">Change Color Scheme</button>
<div id="test" class="navy">
{1}
</div>
</html>
        "###,
        HEADER, fs
    );

    Ok(())
}

const HEADER: &'static str = r###"
<head>
<script>
    var flip=false;
    function monclick(){
        var foo=document.getElementById("test");
        if (flip){
            foo.classList.remove("light");
            foo.classList.add("navy");
        }else{
            foo.classList.remove("navy");
            foo.classList.add("light");
                
        }
        flip=!flip;
    }
</script>
<style>
body {
    background-color: coral;
}
.light{
    --fg:black;
    --bg:white;
    --pplot_color0:blue;
    --pplot_color1:red;
    --pplot_color2:green;
    --pplot_color3:yellow;
    --pplot_color4:purple;
    --pplot_color5:orange;
    --pplot_color6:cyan;
    --pplot_color7:lime;
    --pplot_color8:chocolate;
 
}
.navy{
    --fg:white;
    --bg:black;
    --pplot_color0:rgb(0, 88, 251);
    --pplot_color1:rgb(255, 0, 85);
    --pplot_color2:rgb(0, 151, 0);
    --pplot_color3:yellow;
    --pplot_color4:purple;
    --pplot_color5:orange;
    --pplot_color6:cyan;
    --pplot_color7:lime;
    --pplot_color8:chocolate;
 
}
.poloto{
    --poloto_fg_color:var(--fg);
    --poloto_bg_color:var(--bg);
    --poloto_color0:var(--pplot_color0);
    --poloto_color1:var(--pplot_color1);
    --poloto_color2:var(--pplot_color2);
    --poloto_color3:var(--pplot_color3);
    --poloto_color4:var(--pplot_color4);
    --poloto_color5:var(--pplot_color5);
    --poloto_color6:var(--pplot_color6);
    --poloto_color7:var(--pplot_color7);
    --poloto_color8:var(--pplot_color8);
  }
</style>
</head> 
"###;
