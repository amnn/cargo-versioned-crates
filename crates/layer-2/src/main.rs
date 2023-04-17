fn main() {
    println!("layer_0   = {:?}", layer_0::layer_0());
    println!("");
    println!("Version 1:");
    println!("ext_1     = {:?}", layer_1::ext_1(1));
    println!("layer_1_0 = {:?}", layer_1::layer_1_0(1));
    println!("layer_1_1 = {:?}", layer_1::layer_1_1(1));
    println!("");
    println!("Version 2:");
    println!("ext_1     = {:?}", layer_1::ext_1(2));
    println!("layer_1_0 = {:?}", layer_1::layer_1_0(2));
    println!("layer_1_1 = {:?}", layer_1::layer_1_1(2));
}
