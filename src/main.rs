extern crate darknet;
use darknet::*;
use std::io::Write;

fn main() {
    run().unwrap();
}

fn run() -> Result<(), Error> {
    std::env::set_current_dir("darknet-sys").unwrap();
    // let network = Network::new("cfg/yolov2-tiny.cfg", "yolov2-tiny.weights")?;
    // let meta = Meta::new("cfg/coco.data")?;
    let mut image = Image::load("data/dog.jpg")?;
    // let dets = simple_detect(&network, &meta, &image);
    // for d in &dets {
    //     image.draw_box(d, 1, 1.0, 0.0, 0.0);
    // }
    // println!("{:?}", dets);
    image.save("dog2")?;
    image.save_jpg("dog2")?;

    let data = image.encode_jpg();
    {
        let mut buffer = ::std::fs::File::create("foo.jpg")?;
        buffer.write_all(&data)?;
    }
    Ok(())
}
