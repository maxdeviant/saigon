use saigon::BotBuilder;

fn main() {
    let bot = BotBuilder::new(([127, 0, 0, 1], 3000)).build().unwrap();
    bot.start();
}
