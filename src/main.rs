use rsonnet_loki::*;

fn main() {
    let x = lokirs::libmain::main();
    if let Err(e) = x {
        for e in anyhow::Chain::new(e.as_ref()) {
            println!("{}", e);
        }
    }
}
