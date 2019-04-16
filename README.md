# infobot-rs
A simple bot for displaying user information written in `Rust` with `serenity-rs`.  
In order to function the bot needs a `config.toml` file, containing your token and your prefix you want to use. 
```toml
token = "<your-bot-token-here>"
prefix = "<prefix>"
```
Make sure the file is located in the same directory as the executable!

### Usage
Run the executable and type `<prefix>info`, the bot will send the information of the user