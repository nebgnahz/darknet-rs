extern crate darknet;
use darknet::*;
use std::io::Write;

fn main() {
    run().unwrap();
}

#[cfg(not(feature = "nnpack"))]
fn run() -> Result<(), Error> {
    std::env::set_current_dir("darknet-sys").unwrap();

    let network = Network::new("cfg/yolov2-tiny.cfg", "yolov2-tiny.weights")?;
    let meta = Meta::new("cfg/coco.data")?;
    let mut image = Image::load("data/dog.jpg")?;
    let dets = simple_detect(&network, &meta, &image);
    for d in &dets {
        image.draw_box(d, 1, 1.0, 0.0, 0.0);
    }
    println!("{:?}", dets);

    let data = image.encode_jpg();
    {
        let mut buffer = ::std::fs::File::create("prediction.jpg")?;
        buffer.write_all(&data)?;
    }
    Ok(())
}

#[cfg(feature = "nnpack")]
fn run() -> Result<(), Error> {
    std::env::set_current_dir("darknet-sys").unwrap();

    let mut network = Network::new("cfg/yolov2-tiny.cfg", "yolov2-tiny.weights")?;
    network.create_threadpool(4);

    let meta = Meta::new("cfg/coco.data")?;
    let mut image = Image::load_threaded("data/dog.jpg", network.channel(), &network.threadpool())?;
    let dets = simple_detect(&network, &meta, &image);
    for d in &dets {
        image.draw_box(d, 1, 1.0, 0.0, 0.0);
    }
    println!("{:?}", dets);

    let data = image.encode_jpg();
    {
        let mut buffer = ::std::fs::File::create("prediction.jpg")?;
        buffer.write_all(&data)?;
    }
    Ok(())
}
