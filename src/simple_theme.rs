//!
//! A simple dark and light css theme.
//!
//!

use super::*;

///
/// Default SVG Header for a Poloto graph.
///
pub const SVG_HEADER: &str = r##"<svg class="poloto" width="800" height="500" viewBox="0 0 800 500" xmlns="http://www.w3.org/2000/svg">"##;

///
/// The default svg dimensions for simple theme.
///
pub const DIM: [f64; 2] = [800.0, 500.0];

///
/// Default SVG end tag.
///
pub const SVG_END: &str = "</svg>";

/// Default light theme
pub const STYLE_CONFIG_LIGHT_DEFAULT: &str = ".poloto{\
    stroke-linecap:round;\
    stroke-linejoin:round;\
    font-family:Roboto,sans-serif;\
    font-size:16px;\
    }\
    .poloto_background{fill:AliceBlue;}\
    .poloto_scatter{stroke-width:7}\
    .poloto_tick_line{stroke:gray;stroke-width:0.5}\
    .poloto_line{stroke-width:2}\
    .poloto_text{fill: black;}\
    .poloto_axis_lines{stroke: black;stroke-width:3;fill:none;stroke-dasharray:none}\
    .poloto_title{font-size:24px;dominant-baseline:start;text-anchor:middle;}\
    .poloto_xname{font-size:24px;dominant-baseline:start;text-anchor:middle;}\
    .poloto_yname{font-size:24px;dominant-baseline:start;text-anchor:middle;}\
    .poloto_legend_text{font-size:20px;dominant-baseline:middle;text-anchor:start;}\
    .poloto0stroke{stroke:blue;}\
    .poloto1stroke{stroke:red;}\
    .poloto2stroke{stroke:green;}\
    .poloto3stroke{stroke:gold;}\
    .poloto4stroke{stroke:aqua;}\
    .poloto5stroke{stroke:lime;}\
    .poloto6stroke{stroke:orange;}\
    .poloto7stroke{stroke:chocolate;}\
    .poloto0fill{fill:blue;}\
    .poloto1fill{fill:red;}\
    .poloto2fill{fill:green;}\
    .poloto3fill{fill:gold;}\
    .poloto4fill{fill:aqua;}\
    .poloto5fill{fill:lime;}\
    .poloto6fill{fill:orange;}\
    .poloto7fill{fill:chocolate;}";

/// Default dark theme
pub const STYLE_CONFIG_DARK_DEFAULT: &str = ".poloto{\
    stroke-linecap:round;\
    stroke-linejoin:round;\
    font-family:Roboto,sans-serif;\
    font-size:16px;\
    }\
    .poloto_background{fill:#262626;}\
    .poloto_scatter{stroke-width:7}\
    .poloto_tick_line{stroke:dimgray;stroke-width:0.5}\
    .poloto_line{stroke-width:2}\
    .poloto_text{fill: white;}\
    .poloto_axis_lines{stroke: white;stroke-width:3;fill:none;stroke-dasharray:none}\
    .poloto_title{font-size:24px;dominant-baseline:start;text-anchor:middle;}\
    .poloto_xname{font-size:24px;dominant-baseline:start;text-anchor:middle;}\
    .poloto_yname{font-size:24px;dominant-baseline:start;text-anchor:middle;}\
    .poloto0stroke{stroke:blue;}\
    .poloto1stroke{stroke:red;}\
    .poloto2stroke{stroke:green;}\
    .poloto3stroke{stroke:gold;}\
    .poloto4stroke{stroke:aqua;}\
    .poloto5stroke{stroke:lime;}\
    .poloto6stroke{stroke:orange;}\
    .poloto7stroke{stroke:chocolate;}\
    .poloto0fill{fill:blue;}\
    .poloto1fill{fill:red;}\
    .poloto2fill{fill:green;}\
    .poloto3fill{fill:gold;}\
    .poloto4fill{fill:aqua;}\
    .poloto5fill{fill:lime;}\
    .poloto6fill{fill:orange;}\
    .poloto7fill{fill:chocolate;}";

///
/// Write the svg header with the specified width and viewport.
///
pub fn write_header<T: std::fmt::Write>(
    mut w: T,
    dim: [f64; 2],
    viewbox: [f64; 2],
) -> std::fmt::Result {
    write!(w, "<svg class=\"poloto\" ")?;
    write!(w, "width=\"{}\" ", dim[0])?;
    write!(w, "height=\"{}\" ", dim[1])?;
    write!(
        w,
        "viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">",
        viewbox[0], viewbox[1]
    )
}

#[derive(Copy, Clone)]
pub struct Theme<'a> {
    styles: &'a str,
}

impl Theme<'static> {
    pub fn light() -> Theme<'static> {
        Theme {
            styles: STYLE_CONFIG_LIGHT_DEFAULT,
        }
    }
    pub fn dark() -> Theme<'static> {
        Theme {
            styles: STYLE_CONFIG_DARK_DEFAULT,
        }
    }
}

impl<'a> Elem for Theme<'a> {
    type Tail = hypermelon::build::ElemTail<&'static str>;
    fn render_head(self, w: &mut hypermelon::ElemWrite) -> Result<Self::Tail, fmt::Error> {
        let k = hypermelon::build::elem("style");
        let k = k.append(self.styles);
        k.render_head(w)
    }
}

//
// TODO add back? I'm not sure if this is an api that we should support.
// css already has an api, and adding another on top of it doesnt seem necessary.
// I would expect users to be able to 'hardcode' in their own css and bake that
// into the graphs. No need for a programmatic interface. However, maybe there are
// cases where this is good? If you want a web-user to be able to dynamically
// change all these css values, it might be good to support.
//
//
// ///
// /// Generate custom css theme.
// ///
// pub struct CssStyleConfig {
//     font_family: String,
//     font_size_small: usize,
//     font_size_axis_labels: usize,
//     font_size_title: usize,
//     background: String,
//     font_color: String,
//     axis_color: String,
//     color0: String,
//     color1: String,
//     color2: String,
//     color3: String,
//     color4: String,
//     color5: String,
//     color6: String,
//     color7: String,
// }
// impl CssStyleConfig {
//     pub fn light_default() -> CssStyleConfig {
//         CssStyleConfig {
//             font_family: "Roboto,sans-serif".to_string(),
//             font_size_small: 16,
//             font_size_axis_labels: 24,
//             font_size_title: 24,
//             background: "AliceBlue".to_string(),
//             font_color: "black".to_string(),
//             axis_color: "black".to_string(),
//             color0: "blue".to_string(),
//             color1: "red".to_string(),
//             color2: "green".to_string(),
//             color3: "gold".to_string(),
//             color4: "aqua".to_string(),
//             color5: "lime".to_string(),
//             color6: "orange".to_string(),
//             color7: "chocolate".to_string(),
//         }
//     }
//     pub fn dark_default() -> CssStyleConfig {
//         CssStyleConfig {
//             font_family: "Roboto,sans-serif".to_string(),
//             font_size_small: 16,
//             font_size_axis_labels: 24,
//             font_size_title: 24,
//             background: "#262626".to_string(),
//             font_color: "white".to_string(),
//             axis_color: "white".to_string(),
//             color0: "blue".to_string(),
//             color1: "red".to_string(),
//             color2: "green".to_string(),
//             color3: "gold".to_string(),
//             color4: "aqua".to_string(),
//             color5: "lime".to_string(),
//             color6: "orange".to_string(),
//             color7: "chocolate".to_string(),
//         }
//     }
//     /// Render the custom theme into a String.
//     pub fn css_render(&self) -> String {
//         format!(
//             ".poloto{{\
//         stroke-linecap:round;\
//         stroke-linejoin:round;\
//         font-family:{};\
//         font-size:{}px;\
//         }}\
//         .poloto_background{{fill:{};}}\
//         .poloto_scatter{{stroke-width:7}}\
//         .poloto_tick_line{{stroke:dimgray;stroke-width:0.5}}\
//         .poloto_line{{stroke-width:2}}\
//         .poloto_text{{fill: {};}}\
//         .poloto_axis_lines{{stroke: {};stroke-width:3;fill:none;stroke-dasharray:none}}\
//         .poloto_title{{font-size:{}px;dominant-baseline:start;text-anchor:middle;}}\
//         .poloto_xname{{font-size:{}px;dominant-baseline:start;text-anchor:middle;}}\
//         .poloto_yname{{font-size:{}px;dominant-baseline:start;text-anchor:middle;}}\
//         .poloto0stroke{{stroke:{};}}\
//         .poloto1stroke{{stroke:{};}}\
//         .poloto2stroke{{stroke:{};}}\
//         .poloto3stroke{{stroke:{};}}\
//         .poloto4stroke{{stroke:{};}}\
//         .poloto5stroke{{stroke:{};}}\
//         .poloto6stroke{{stroke:{};}}\
//         .poloto7stroke{{stroke:{};}}\
//         .poloto0fill{{fill:{};}}\
//         .poloto1fill{{fill:{};}}\
//         .poloto2fill{{fill:{};}}\
//         .poloto3fill{{fill:{};}}\
//         .poloto4fill{{fill:{};}}\
//         .poloto5fill{{fill:{};}}\
//         .poloto6fill{{fill:{};}}\
//         .poloto7fill{{fill:{};}}",
//             self.font_family,
//             self.font_size_small,
//             self.background,
//             self.font_color,
//             self.axis_color,
//             self.font_size_title,
//             self.font_size_axis_labels,
//             self.font_size_axis_labels,
//             self.color0,
//             self.color1,
//             self.color2,
//             self.color3,
//             self.color4,
//             self.color5,
//             self.color6,
//             self.color7,
//             self.color0,
//             self.color1,
//             self.color2,
//             self.color3,
//             self.color4,
//             self.color5,
//             self.color6,
//             self.color7,
//         )
//     }
//     /// Customize the color of the first plot.
//     pub fn set_color0(mut self, new_color: &str) -> CssStyleConfig {
//         self.color0 = new_color.to_string();
//         self
//     }
//     /// Customize the colors used for the plots.
//     pub fn set_line_colors(
//         mut self,
//         new_color0: &str,
//         new_color1: &str,
//         new_color2: &str,
//         new_color3: &str,
//         new_color4: &str,
//         new_color5: &str,
//         new_color6: &str,
//         new_color7: &str,
//     ) -> CssStyleConfig {
//         self.color0 = new_color0.to_string();
//         self.color1 = new_color1.to_string();
//         self.color2 = new_color2.to_string();
//         self.color3 = new_color3.to_string();
//         self.color4 = new_color4.to_string();
//         self.color5 = new_color5.to_string();
//         self.color6 = new_color6.to_string();
//         self.color7 = new_color7.to_string();
//         self
//     }
//     /// Configure the font and fontsize.
//     pub fn set_font(
//         mut self,
//         font_family: &str,
//         font_size_small: usize,
//         font_size_axis_labels: usize,
//         font_size_title: usize,
//     ) -> CssStyleConfig {
//         self.font_family = font_family.to_string();
//         self.font_size_small = font_size_small;
//         self.font_size_axis_labels = font_size_axis_labels;
//         self.font_size_title = font_size_title;
//         self
//     }
//     pub fn set_font_family(mut self, font: &str) -> CssStyleConfig {
//         self.font_family = font.to_string();
//         self
//     }
//     /// Customize the background color.
//     pub fn set_background(mut self, background: &str) -> CssStyleConfig {
//         self.background = background.to_string();
//         self
//     }
//     /// Customize the font color.
//     pub fn set_font_color(mut self, font_color: &str) -> CssStyleConfig {
//         self.font_color = font_color.to_string();
//         self
//     }
//     /// Customize the axis color.
//     pub fn set_axis_color(mut self, axis_color: &str) -> CssStyleConfig {
//         self.axis_color = axis_color.to_string();
//         self
//     }
// }
