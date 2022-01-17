use rsonnet_loki::*;

fn main() {
    let x = lokirs::libmain::main().err();
    if let Some(e) = x {
        format!("{}", e);
    }
}
