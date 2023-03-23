use std::collections::HashSet;
use std::time::Duration;

use reqwest::Client;
use teloxide::prelude::*;
use tokio::time::sleep;

mod stackoverflow;

const POLLING_INTERVAL: Duration = Duration::from_secs(10);

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    let bot = Bot::from_env();
    let group_chat_id = ChatId(
        std::env::var("TELEGRAM_GROUP_CHAT_ID")
            .expect("TELEGRAM_GROUP_CHAT_ID env variable must be specified")
            .parse()
            .unwrap(),
    );
    let http_client = Client::new();
    let mut posted_question_ids: HashSet<_> =
        stackoverflow::fetch_nearprotocol_questions(&http_client)
            .await
            .unwrap()
            .into_iter()
            .map(|question| question.question_id)
            .collect();
    let mut synced_messages = std::collections::HashMap::new();

    loop {
        sleep(POLLING_INTERVAL).await;

        let Ok(questions) = stackoverflow::fetch_nearprotocol_questions(&http_client)
            .await else { continue; };

        for question in &questions {
            if !posted_question_ids.contains(&question.question_id) {
                posted_question_ids.insert(question.question_id);

                let text = format_question_message(&question);

                match bot
                    .send_message(group_chat_id, &text)
                    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                    .send()
                    .await
                {
                    Ok(message) => {
                        synced_messages.insert(question.question_id, (message.id, text));
                    }
                    Err(err) => {
                        eprintln!("WARN: {}", err);
                    }
                }
            } else {
                // Find the existing message and update it if needed
                if let Some((synced_message_id, text)) = synced_messages.get(&question.question_id)
                {
                    let new_text = format_question_message(&question);
                    if &new_text != text {
                        match bot
                            .edit_message_text(group_chat_id, *synced_message_id, &new_text)
                            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                            .send()
                            .await
                        {
                            Ok(message) => {
                                synced_messages
                                    .insert(question.question_id, (message.id, new_text));
                            }
                            Err(err) => {
                                eprintln!("WARN: {}", err);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn format_question_message(question: &stackoverflow::Question) -> String {
    format!(
        "🆕 [{title}]({link})\n\n❓ {answers} answers \\| {status}",
        title = question.title,
        link = question.link,
        answers = question.answer_count,
        status = if question.is_answered {
            "Answered"
        } else {
            "Unanswered"
        }
    )
}
