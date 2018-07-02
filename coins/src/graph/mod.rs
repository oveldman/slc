use gnuplot::{Figure, Caption, Color};
use database::client::CryptoCoins;

pub fn make_plot(coin: CryptoCoins) -> String {
    let total_coins: usize = coin.gulden.len();
    let mut current_coin: usize = 0;
    let mut x: Vec<f32> = Vec::new();
    let mut y: Vec<f32> = Vec::new();

    while current_coin < total_coins {
        x.push(current_coin as f32);
        y.push(coin.gulden[current_coin].price.parse::<f32>().unwrap());
        current_coin = current_coin + 1;
    }
    
    let mut fg = Figure::new();
    fg.axes2d()
    .lines(&x, &y, &[Caption("Euro per gulden"), Color("black")]);

    let standard_location_image: String = String::from("/content/test.png");
    let save_location: String = String::from("templates");
    let best_location: String = save_location + &standard_location_image;
    fg.set_terminal(&"pngcairo", &best_location);
    fg.show();
    standard_location_image.to_string()
}