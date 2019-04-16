use chrono::NaiveDateTime;
use serenity::{
    client::{Context, EventHandler},
    command,
    framework::StandardFramework,
    model::{gateway::Game, gateway::Ready, guild::Member, misc::Mentionable, user::OnlineStatus},
    Client,
};
use serenity::CACHE;

const TOKEN: &str = "NTY3MDU3NTk0NjI4NDQwMDcw.XLXbIA.B1kC-NELIBVah_roP5w4caZQsrI";

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _ctx: Context, bot: Ready) {
        _ctx.shard.set_status(OnlineStatus::Online);
        _ctx.shard.set_game(Some(Game::playing(
            format!("Informing in {} servers!", bot.guilds.len()).as_str(),
        )));
        println!("Bot is ready: {}#{}", bot.user.name, bot.user.discriminator);
    }
}

fn main() {
    let mut client = Client::new(TOKEN, Handler).unwrap();
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("~"))
            .cmd("info", info),
    );

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

command!(info(_ctx, msg) {
    msg.channel_id.broadcast_typing().unwrap();
    let member: Member = msg.guild_id.unwrap().member(msg.author.id).unwrap();

    let username = msg.clone().author.name;
    let nickname = member.display_name();
    let discriminator = msg.author.discriminator;
    let id = msg.author.id;
    let avatar_url = msg.author.avatar_url().unwrap();
    let joined_at = NaiveDateTime::from_timestamp(member.joined_at.unwrap().timestamp(), 0);
    let joined_discord = id.created_at();

    msg.channel_id
        .send_message(|m| {
            m.embed(|e| {
                e.title("User Information")
                    .field("Username", username, true)
                    .field("Nickname", nickname, true)
                    .field("ID", id, true)
                    .field("Discriminator", discriminator, true)
                    .field("Joined The Server At", joined_at, true)
                    .field("Joined Discord At", joined_discord, true)
                    .field(
                        "Roles",
                        member
                            .roles
                            .iter()
                            .map(|r| r.to_role_cached().unwrap().mention())
                            .collect::<Vec<String>>()
                            .join(" "),
                        false,
                    )
                    .thumbnail(avatar_url)
                    .color(0x000000)
                    .footer(|f| f.text(format!("Made by {}", msg.guild_id.unwrap().member(CACHE.read().user.id).unwrap().display_name())))
            })
        })
        .unwrap();
});