extern crate darknet;
extern crate failure;
use darknet::*;

fn main() {
    run().unwrap();
}

fn run() -> Result<(), failure::Error> {
    let network = Network::new("darknet-sys/cfg/yolov2-tiny.cfg", "yolov2-tiny.weights")?;
    let meta = Meta::new("darknet-sys/cfg/coco.data")?;
    let r = detect(&network, &meta, "darknet-sys/data/dog.jpg", 0.5, 0.5, 0.45);
    println!("{:?}", r);
    Ok(())
}
