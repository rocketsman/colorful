extern crate colorful;
extern crate core;

use colorful::Colorful;
use colorful::Color;
use colorful::core::ColorInterface;
use colorful::HSL;

#[test]
fn test_color() {
    let s = "Hello world";
    for (i, color) in Color::iterator().enumerate() {
        assert_eq!(format!("\x1B[38;5;{}mHello world\x1B[0m", i.to_owned()), s.color(*color).to_string());
    }
}


#[test]
fn test_color_to_hsl() {
    let l = vec![
        HSL::new(0.0, 0.0, 0.0),
        HSL::new(0.0, 1.0, 0.25),
        HSL::new(0.3333333333333333, 1.0, 0.25),
        HSL::new(0.16666666666666666, 1.0, 0.25),
        HSL::new(0.6666666666666666, 1.0, 0.25),
        HSL::new(0.8333333333333334, 1.0, 0.25),
        HSL::new(0.5, 1.0, 0.25),
        HSL::new(0.0, 0.0, 0.75),
        HSL::new(0.0, 0.0, 0.5),
        HSL::new(0.0, 1.0, 0.5),
        HSL::new(0.3333333333333333, 1.0, 0.5),
        HSL::new(0.16666666666666666, 1.0, 0.5),
        HSL::new(0.6666666666666666, 1.0, 0.5),
        HSL::new(0.8333333333333334, 1.0, 0.5),
        HSL::new(0.5, 1.0, 0.5),
        HSL::new(0.0, 0.0, 1.0),
        HSL::new(0.0, 0.0, 0.0),
        HSL::new(0.6666666666666666, 1.0, 0.18),
        HSL::new(0.6666666666666666, 1.0, 0.26),
        HSL::new(0.6666666666666666, 1.0, 0.34),
        HSL::new(0.6666666666666666, 1.0, 0.42),
        HSL::new(0.6666666666666666, 1.0, 0.5),
        HSL::new(0.3333333333333333, 1.0, 0.18),
        HSL::new(0.5, 1.0, 0.18),
        HSL::new(0.5493827160493834, 1.0, 0.26),
        HSL::new(0.5761904761904749, 1.0, 0.34),
        HSL::new(0.5930232558139528, 1.0, 0.42),
        HSL::new(0.6045751633986917, 1.0, 0.5),
        HSL::new(0.3333333333333333, 1.0, 0.26),
        HSL::new(0.4506172839506167, 1.0, 0.26),
        HSL::new(0.5, 1.0, 0.26),
        HSL::new(0.538095238095239, 1.0, 0.34),
        HSL::new(0.5620155038759694, 1.0, 0.42),
        HSL::new(0.5784313725490194, 1.0, 0.5),
        HSL::new(0.3333333333333333, 1.0, 0.34),
        HSL::new(0.423809523809525, 1.0, 0.34),
        HSL::new(0.4619047619047611, 1.0, 0.34),
        HSL::new(0.5, 1.0, 0.34),
        HSL::new(0.5310077519379833, 1.0, 0.42),
        HSL::new(0.5522875816993472, 1.0, 0.5),
        HSL::new(0.3333333333333333, 1.0, 0.42),
        HSL::new(0.40697674418604723, 1.0, 0.42),
        HSL::new(0.43798449612403056, 1.0, 0.42),
        HSL::new(0.4689922480620166, 1.0, 0.42),
        HSL::new(0.5, 1.0, 0.42),
        HSL::new(0.5261437908496722, 1.0, 0.5),
        HSL::new(0.3333333333333333, 1.0, 0.5),
        HSL::new(0.39542483660130834, 1.0, 0.5),
        HSL::new(0.4215686274509806, 1.0, 0.5),
        HSL::new(0.4477124183006528, 1.0, 0.5),
        HSL::new(0.4738562091503278, 1.0, 0.5),
        HSL::new(0.5, 1.0, 0.5),
        HSL::new(0.0, 1.0, 0.18),
        HSL::new(0.8333333333333334, 1.0, 0.18),
        HSL::new(0.78395061728395, 1.0, 0.26),
        HSL::new(0.7571428571428583, 1.0, 0.34),
        HSL::new(0.7403100775193806, 1.0, 0.42),
        HSL::new(0.7287581699346417, 1.0, 0.5),
        HSL::new(0.16666666666666666, 1.0, 0.18),
        HSL::new(0.0, 0.0, 0.37),
        HSL::new(0.6666666666666666, 0.17, 0.45),
        HSL::new(0.6666666666666666, 0.33, 0.52),
        HSL::new(0.6666666666666666, 0.6, 0.6),
        HSL::new(0.6666666666666666, 1.0, 0.68),
        HSL::new(0.21604938271604945, 1.0, 0.26),
        HSL::new(0.3333333333333333, 0.17, 0.45),
        HSL::new(0.5, 0.17, 0.45),
        HSL::new(0.5833333333333334, 0.33, 0.52),
        HSL::new(0.6111111111111112, 0.6, 0.6),
        HSL::new(0.625, 1.0, 0.68),
        HSL::new(0.24285714285714277, 1.0, 0.34),
        HSL::new(0.3333333333333333, 0.33, 0.52),
        HSL::new(0.4166666666666667, 0.33, 0.52),
        HSL::new(0.5, 0.33, 0.52),
        HSL::new(0.5555555555555556, 0.6, 0.6),
        HSL::new(0.5833333333333334, 1.0, 0.68),
        HSL::new(0.2596899224806203, 1.0, 0.42),
        HSL::new(0.3333333333333333, 0.6, 0.6),
        HSL::new(0.3888888888888889, 0.6, 0.6),
        HSL::new(0.4444444444444444, 0.6, 0.6),
        HSL::new(0.5, 0.6, 0.6),
        HSL::new(0.5416666666666666, 1.0, 0.68),
        HSL::new(0.27124183006535946, 1.0, 0.5),
        HSL::new(0.3333333333333333, 1.0, 0.68),
        HSL::new(0.375, 1.0, 0.68),
        HSL::new(0.4166666666666667, 1.0, 0.68),
        HSL::new(0.4583333333333333, 1.0, 0.68),
        HSL::new(0.5, 1.0, 0.68),
        HSL::new(0.0, 1.0, 0.26),
        HSL::new(0.8827160493827166, 1.0, 0.26),
        HSL::new(0.8333333333333334, 1.0, 0.26),
        HSL::new(0.7952380952380944, 1.0, 0.34),
        HSL::new(0.7713178294573639, 1.0, 0.42),
        HSL::new(0.7549019607843138, 1.0, 0.5),
        HSL::new(0.11728395061728389, 1.0, 0.26),
        HSL::new(0.0, 0.17, 0.45),
        HSL::new(0.8333333333333334, 0.17, 0.45),
        HSL::new(0.75, 0.33, 0.52),
        HSL::new(0.7222222222222222, 0.6, 0.6),
        HSL::new(0.7083333333333334, 1.0, 0.68),
        HSL::new(0.16666666666666666, 1.0, 0.26),
        HSL::new(0.16666666666666666, 0.17, 0.45),
        HSL::new(0.0, 0.0, 0.52),
        HSL::new(0.6666666666666666, 0.2, 0.6),
        HSL::new(0.6666666666666666, 0.5, 0.68),
        HSL::new(0.6666666666666666, 1.0, 0.76),
        HSL::new(0.2047619047619047, 1.0, 0.34),
        HSL::new(0.25, 0.33, 0.52),
        HSL::new(0.3333333333333333, 0.2, 0.6),
        HSL::new(0.5, 0.2, 0.6),
        HSL::new(0.5833333333333334, 0.5, 0.68),
        HSL::new(0.6111111111111112, 1.0, 0.76),
        HSL::new(0.22868217054263557, 1.0, 0.42),
        HSL::new(0.2777777777777778, 0.6, 0.6),
        HSL::new(0.3333333333333333, 0.5, 0.68),
        HSL::new(0.4166666666666667, 0.5, 0.68),
        HSL::new(0.5, 0.5, 0.68),
        HSL::new(0.5555555555555556, 1.0, 0.76),
        HSL::new(0.2450980392156864, 1.0, 0.5),
        HSL::new(0.2916666666666667, 1.0, 0.68),
        HSL::new(0.3333333333333333, 1.0, 0.76),
        HSL::new(0.3888888888888889, 1.0, 0.76),
        HSL::new(0.4444444444444444, 1.0, 0.76),
        HSL::new(0.5, 1.0, 0.76),
        HSL::new(0.0, 1.0, 0.34),
        HSL::new(0.9095238095238083, 1.0, 0.34),
        HSL::new(0.8714285714285722, 1.0, 0.34),
        HSL::new(0.8333333333333334, 1.0, 0.34),
        HSL::new(0.80232558139535, 1.0, 0.42),
        HSL::new(0.7810457516339862, 1.0, 0.5),
        HSL::new(0.09047619047619054, 1.0, 0.34),
        HSL::new(0.0, 0.33, 0.52),
        HSL::new(0.9166666666666666, 0.33, 0.52),
        HSL::new(0.8333333333333334, 0.33, 0.52),
        HSL::new(0.7777777777777778, 0.6, 0.6),
        HSL::new(0.75, 1.0, 0.68),
        HSL::new(0.12857142857142861, 1.0, 0.34),
        HSL::new(0.08333333333333333, 0.33, 0.52),
        HSL::new(0.0, 0.2, 0.6),
        HSL::new(0.8333333333333334, 0.2, 0.6),
        HSL::new(0.75, 0.5, 0.68),
        HSL::new(0.7222222222222222, 1.0, 0.76),
        HSL::new(0.16666666666666666, 1.0, 0.34),
        HSL::new(0.16666666666666666, 0.33, 0.52),
        HSL::new(0.16666666666666666, 0.2, 0.6),
        HSL::new(0.0, 0.0, 0.68),
        HSL::new(0.6666666666666666, 0.33, 0.76),
        HSL::new(0.6666666666666666, 1.0, 0.84),
        HSL::new(0.1976744186046511, 1.0, 0.42),
        HSL::new(0.2222222222222222, 0.6, 0.6),
        HSL::new(0.25, 0.5, 0.68),
        HSL::new(0.3333333333333333, 0.33, 0.76),
        HSL::new(0.5, 0.33, 0.76),
        HSL::new(0.5833333333333334, 1.0, 0.84),
        HSL::new(0.21895424836601304, 1.0, 0.5),
        HSL::new(0.25, 1.0, 0.68),
        HSL::new(0.2777777777777778, 1.0, 0.76),
        HSL::new(0.3333333333333333, 1.0, 0.84),
        HSL::new(0.4166666666666667, 1.0, 0.84),
        HSL::new(0.5, 1.0, 0.84),
        HSL::new(0.0, 1.0, 0.42),
        HSL::new(0.926356589147286, 1.0, 0.42),
        HSL::new(0.8953488372093028, 1.0, 0.42),
        HSL::new(0.8643410852713166, 1.0, 0.42),
        HSL::new(0.8333333333333334, 1.0, 0.42),
        HSL::new(0.8071895424836611, 1.0, 0.5),
        HSL::new(0.07364341085271306, 1.0, 0.42),
        HSL::new(0.0, 0.6, 0.6),
        HSL::new(0.9444444444444444, 0.6, 0.6),
        HSL::new(0.8888888888888888, 0.6, 0.6),
        HSL::new(0.8333333333333334, 0.6, 0.6),
        HSL::new(0.7916666666666666, 1.0, 0.68),
        HSL::new(0.10465116279069778, 1.0, 0.42),
        HSL::new(0.05555555555555555, 0.6, 0.6),
        HSL::new(0.0, 0.5, 0.68),
        HSL::new(0.9166666666666666, 0.5, 0.68),
        HSL::new(0.8333333333333334, 0.5, 0.68),
        HSL::new(0.7777777777777778, 1.0, 0.76),
        HSL::new(0.13565891472868222, 1.0, 0.42),
        HSL::new(0.1111111111111111, 0.6, 0.6),
        HSL::new(0.08333333333333333, 0.5, 0.68),
        HSL::new(0.0, 0.33, 0.76),
        HSL::new(0.8333333333333334, 0.33, 0.76),
        HSL::new(0.75, 1.0, 0.84),
        HSL::new(0.16666666666666666, 1.0, 0.42),
        HSL::new(0.16666666666666666, 0.6, 0.6),
        HSL::new(0.16666666666666666, 0.5, 0.68),
        HSL::new(0.16666666666666666, 0.33, 0.76),
        HSL::new(0.0, 0.0, 0.84),
        HSL::new(0.6666666666666666, 1.0, 0.92),
        HSL::new(0.19281045751633974, 1.0, 0.5),
        HSL::new(0.20833333333333334, 1.0, 0.68),
        HSL::new(0.2222222222222222, 1.0, 0.76),
        HSL::new(0.25, 1.0, 0.84),
        HSL::new(0.3333333333333333, 1.0, 0.92),
        HSL::new(0.5, 1.0, 0.92),
        HSL::new(0.0, 1.0, 0.5),
        HSL::new(0.937908496732025, 1.0, 0.5),
        HSL::new(0.9117647058823528, 1.0, 0.5),
        HSL::new(0.8856209150326805, 1.0, 0.5),
        HSL::new(0.8594771241830055, 1.0, 0.5),
        HSL::new(0.8333333333333334, 1.0, 0.5),
        HSL::new(0.06209150326797389, 1.0, 0.5),
        HSL::new(0.0, 1.0, 0.68),
        HSL::new(0.9583333333333334, 1.0, 0.68),
        HSL::new(0.9166666666666666, 1.0, 0.68),
        HSL::new(0.875, 1.0, 0.68),
        HSL::new(0.8333333333333334, 1.0, 0.68),
        HSL::new(0.08823529411764694, 1.0, 0.5),
        HSL::new(0.041666666666666664, 1.0, 0.68),
        HSL::new(0.0, 1.0, 0.76),
        HSL::new(0.9444444444444444, 1.0, 0.76),
        HSL::new(0.8888888888888888, 1.0, 0.76),
        HSL::new(0.8333333333333334, 1.0, 0.76),
        HSL::new(0.11437908496732027, 1.0, 0.5),
        HSL::new(0.08333333333333333, 1.0, 0.68),
        HSL::new(0.05555555555555555, 1.0, 0.76),
        HSL::new(0.0, 1.0, 0.84),
        HSL::new(0.9166666666666666, 1.0, 0.84),
        HSL::new(0.8333333333333334, 1.0, 0.84),
        HSL::new(0.14052287581699335, 1.0, 0.5),
        HSL::new(0.125, 1.0, 0.68),
        HSL::new(0.1111111111111111, 1.0, 0.76),
        HSL::new(0.08333333333333333, 1.0, 0.84),
        HSL::new(0.0, 1.0, 0.92),
        HSL::new(0.8333333333333334, 1.0, 0.92),
        HSL::new(0.16666666666666666, 1.0, 0.5),
        HSL::new(0.16666666666666666, 1.0, 0.68),
        HSL::new(0.16666666666666666, 1.0, 0.76),
        HSL::new(0.16666666666666666, 1.0, 0.84),
        HSL::new(0.16666666666666666, 1.0, 0.92),
        HSL::new(0.0, 0.0, 1.0),
        HSL::new(0.0, 0.0, 0.03),
        HSL::new(0.0, 0.0, 0.07),
        HSL::new(0.0, 0.0, 0.1),
        HSL::new(0.0, 0.0, 0.14),
        HSL::new(0.0, 0.0, 0.18),
        HSL::new(0.0, 0.0, 0.22),
        HSL::new(0.0, 0.0, 0.26),
        HSL::new(0.0, 0.0, 0.3),
        HSL::new(0.0, 0.0, 0.34),
        HSL::new(0.0, 0.0, 0.37),
        HSL::new(0.0, 0.0, 0.4),
        HSL::new(0.0, 0.0, 0.46),
        HSL::new(0.0, 0.0, 0.5),
        HSL::new(0.0, 0.0, 0.54),
        HSL::new(0.0, 0.0, 0.58),
        HSL::new(0.0, 0.0, 0.61),
        HSL::new(0.0, 0.0, 0.65),
        HSL::new(0.0, 0.0, 0.69),
        HSL::new(0.0, 0.0, 0.73),
        HSL::new(0.0, 0.0, 0.77),
        HSL::new(0.0, 0.0, 0.81),
        HSL::new(0.0, 0.0, 0.85),
        HSL::new(0.0, 0.0, 0.89),
        HSL::new(0.0, 0.0, 0.93), ];
    for (i, color) in Color::iterator().enumerate() {
        assert_eq!(color.to_hsl(), l[i]);
    }
}