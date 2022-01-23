use rsonnet_loki::*;

fn main() {
    let x = lokirs::libmain::main();
    if let Err(e) = x {
        println!("Found error: {}", e)
    }
}
