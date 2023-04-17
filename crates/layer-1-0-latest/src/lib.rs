use ext_0::{Ext0, ext_0};
use layer_0::{Layer0, layer_0};

pub fn layer_1_0() -> Layer0 {
    let Layer0(x) = layer_0();
    let Ext0(y) = ext_0();
    Layer0(2 * x + 3 * y)
}
