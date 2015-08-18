pub struct AssortedPrimitives {
    b: bool,
    eight: i8,
    sixteen: i16,
    thirtytwo: i32,
    sixtyfour: i64,
    fthirtytwo: f32,
    fsixtyfour: f64,
    o_b: Option<bool>,
    o_eight: Option<i8>,
    o_sixteen: Option<i16>,
    o_thirtytwo: Option<i32>,
    o_sixtyfour: Option<i64>,
    o_fthirtytwo: Option<f32>,
    o_fsixtyfour: Option<f64>,
}

impl AssortedPrimitives {
    pub fn new(
        b: bool,
        eight: i8,
        sixteen: i16,
        thirtytwo: i32,
        sixtyfour: i64,
        fthirtytwo: f32,
        fsixtyfour: f64,
        o_b: Option<bool>,
        o_eight: Option<i8>,
        o_sixteen: Option<i16>,
        o_thirtytwo: Option<i32>,
        o_sixtyfour: Option<i64>,
        o_fthirtytwo: Option<f32>,
        o_fsixtyfour: Option<f64>,
    ) -> AssortedPrimitives {
        AssortedPrimitives {
            b: b,
            eight: eight,
            sixteen: sixteen,
            thirtytwo: thirtytwo,
            sixtyfour: sixtyfour,
            fthirtytwo: fthirtytwo,
            fsixtyfour: fsixtyfour,
            o_b: o_b,
            o_eight: o_eight,
            o_sixteen: o_sixteen,
            o_thirtytwo: o_thirtytwo,
            o_sixtyfour: o_sixtyfour,
            o_fthirtytwo: o_fthirtytwo,
            o_fsixtyfour: o_fsixtyfour,
        }
    }
}