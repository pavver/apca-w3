const B_EXP: f64 = 1.414;
const R_SCALE: f64 = 1.14;
const B_THRESH: f64 = 0.022;
const W_OFFSET: f64 = 0.027;
const P_IN: f64 = 0.0005;
const P_OUT: f64 = 0.1;

pub fn lightness_contrast_srgb_u8(txt: [u8; 3], bg: [u8; 3]) -> f64 {
    lightness_contrast_srgb(
        [
            txt[0] as f64 / 255.0,
            txt[1] as f64 / 255.0,
            txt[2] as f64 / 255.0,
        ],
        [
            bg[0] as f64 / 255.0,
            bg[1] as f64 / 255.0,
            bg[2] as f64 / 255.0,
        ],
    )
}

pub fn lightness_contrast_srgb_f32(txt: [f32; 3], bg: [f32; 3]) -> f64 {
    lightness_contrast_srgb(
        [txt[0] as f64, txt[1] as f64, txt[2] as f64],
        [bg[0] as f64, bg[1] as f64, bg[2] as f64],
    )
}

pub fn lightness_contrast_srgb(txt: [f64; 3], bg: [f64; 3]) -> f64 {
    let y_txt = f_clamp(screen_luminance(txt[0], txt[1], txt[2]));
    let y_bg = f_clamp(screen_luminance(bg[0], bg[1], bg[2]));

    let s_norm = y_bg.powf(0.56) - y_txt.powf(0.57);
    let s_rev = y_bg.powf(0.65) - y_txt.powf(0.62);

    let c = if (y_bg - y_txt) < P_IN {
        0.0
    } else if y_txt < y_bg {
        s_norm * R_SCALE
    } else {
        s_rev * R_SCALE
    };

    let s_apc = if c.abs() < P_OUT {
        0.0
    } else if c > 0.0 {
        c - W_OFFSET
    } else {
        c + W_OFFSET
    };

    s_apc * 100.0
}

fn f_clamp(y: f64) -> f64 {
    if y >= B_THRESH {
        y
    } else {
        y + (B_THRESH - y).powf(B_EXP)
    }
}

pub fn screen_luminance(r: f64, g: f64, b: f64) -> f64 {
    r.powf(2.4) * 0.2126729 + g.powf(2.4) * 0.7151522 + b.powf(2.4) * 0.0721750
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_eq_float::assert_eq_float;

    #[test]
    fn it_works() {
        assert_eq_float!(
            lightness_contrast_srgb_u8([174, 25, 50], [224, 223, 222]),
            63.84288582006901
        );
        assert_eq_float!(
            lightness_contrast_srgb_u8([209, 111, 21], [224, 223, 222]),
            43.342135468280915
        );
        assert_eq_float!(
            lightness_contrast_srgb_u8([85, 139, 207], [224, 223, 222]),
            43.78344811985997
        );
    }
}
