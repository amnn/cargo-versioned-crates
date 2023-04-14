macro_rules! test {
    ($fn:ident) => { println!("{} = {:?}", stringify!($fn), $fn::$fn()) }
}

fn main() {
    test!(layer_0);
    test!(layer_1_0);
    test!(layer_1_1);
}
