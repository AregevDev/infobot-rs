use serenity::{
    client::{Context, EventHandler},
    command,
    framework::StandardFramework,
    model::gateway::Ready,
    Client,
};

const TOKEN: &str = "NTY3MDU3NTk0NjI4NDQwMDcw.XLXbIA.B1kC-NELIBVah_roP5w4caZQsrI";

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        println!("Bot is ready!");
    }
}

fn main() {
    let mut client = Client::new(TOKEN, Handler).unwrap();
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("~").allow_dm(false).case_insensitivity(true))
            .cmd("info", Info),
    );

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

command!(Info(_context, message) {
    let _ = message.reply("Pong!");
});
