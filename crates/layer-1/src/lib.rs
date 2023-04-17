use ext_0::Ext0;
use layer_0::Layer0;
use layer_1_v1 as v1;

pub fn ext_1(v: u8) -> Ext0 {
    match v {
        1 => v1::ext_1(),
        // 2 => v2::ext_1(),
        _ => unreachable!("ext_1: Unrecognized version, v{v}"),
    }
}

pub fn layer_1_0(v: u8) -> Layer0 {
    match v {
        1 => v1::layer_1_0(),
        // 2 => v2::layer_1_0(),
        _ => unreachable!("layer_1: Unrecognized version, v{v}"),
    }
}

pub fn layer_1_1(v: u8) -> Layer0 {
    match v {
        1 => v1::layer_1_1(),
        // 2 => v2::layer_1_1(),
        _ => unreachable!("layer_1: Unrecognized version, v{v}"),
    }
}
