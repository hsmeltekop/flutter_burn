use anyhow::Result;
use test_burn::predictor::predictor;

fn main() -> Result<()> {
    /*
    convert eight.png -resize 28x28! eight28.png
    convert eight28.png -set colorspace Gray -separate -average gray_eight28.bmp
    convert gray_eight28.bmp [ -rotate 180 -flop ] gray_eight28r.bmp
    stream -map r -storage-type char gray_eight28r.bmp eight.raw
     */

    let h = std::thread::spawn(|| {
        println!("inside another thread....");
        std::thread::sleep(std::time::Duration::from_millis(1000));
        let bytes = include_bytes!("../../five.raw");
        let predicted = predictor().predict(bytes, 5);
        println!("Predicted {} Expected {} inside thread...", predicted, 5);
    });

    println!("{:=>78}", "");

    let bytes = include_bytes!("../../four.raw");
    let predicted = predictor().predict(bytes, 4);
    println!("Predicted {} Expected {}", predicted, 4);

    let bytes = include_bytes!("../../eight.raw");
    let predicted = predictor().predict(bytes, 8);
    println!("Predicted {} Expected {}", predicted, 8);

    let bytes = include_bytes!("../../seven.raw");
    let predicted = predictor().predict(bytes, 7);
    println!("Predicted {} Expected {}", predicted, 7);

    let bytes = include_bytes!("../../three.raw");
    let predicted = predictor().predict(bytes, 3);
    println!("Predicted {} Expected {}", predicted, 3);

    h.join().unwrap();

    Ok(())
}
