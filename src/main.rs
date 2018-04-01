extern crate darknet;
use darknet::*;

fn main() {
    run().unwrap();
}

fn run() -> Result<(), Error> {
    std::env::set_current_dir("darknet-sys").unwrap();
    let network = Network::new("cfg/yolov2-tiny.cfg", "yolov2-tiny.weights")?;
    let meta = Meta::new("cfg/coco.data")?;
    let image = Image::load("data/dog.jpg")?;
    let r = simple_detect(&network, &meta, &image);
    println!("{:?}", r);
    Ok(())
}
