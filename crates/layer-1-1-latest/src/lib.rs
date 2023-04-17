use ext_0::Ext0;
use ext_1::ext_1;
use layer_0::{Layer0, layer_0};

pub fn layer_1_1() -> Layer0 {
    let Layer0(x) = layer_0();
    let Ext0(y) = ext_1();
    Layer0(3 * x + 4 * y)
}
