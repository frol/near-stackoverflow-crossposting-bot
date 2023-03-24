# NearProtocol Sync Bot

`nearprotocol_sync_bot` is a Telegram bot written in Rust that syncs new StackOverflow questions tagged with `nearprotocol` to a Telegram group.
It also updates the posted messages in the group when the questions are answered on StackOverflow.

## Requirements

- Rust and Cargo (latest stable version)
- A Telegram bot token from BotFather
- A Telegram group chat ID

## Setup

1. Clone the repository:

```bash
$ git clone https://github.com/frol/nearprotocol_sync_bot.git
$ cd nearprotocol_sync_bot
```

2. Set up the environment variables:

Create a `.env` file in the project root with the following content:

```
TELOXIDE_TOKEN=your_telegram_bot_token
TELEGRAM_GROUP_CHAT_ID=your_telegram_group_chat_id
```

Replace `your_telegram_bot_token` with your actual bot token obtained from the BotFather, and `your_telegram_group_chat_id` with the ID of the group where you want the bot to post messages.

3. Build the project:

```bash
$ cargo build --release
```

4. Run the bot:

```bash
$ ./target/release/nearprotocol_sync_bot
```

The bot is now running and syncing new `nearprotocol` tagged questions from StackOverflow to the specified Telegram group.

## Customization

You can customize the bot by modifying the source code. For example, you can change the StackOverflow tag being monitored, the polling interval, or the message format.

## License

This project is released under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a pull request, open an issue, or provide feedback.
