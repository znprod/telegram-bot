use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting ZnProd bot...");

    let bot = Bot::from_env();
    teloxide::repl(bot, |message| async move {
        say_hello(message).await;
        ResponseResult::<()>::Ok(())
    })
    .await;
}

async fn say_hello(cx: UpdateWithCx<Message>) {
    match cx.update.new_chat_members() {
        None => return,
        Some(new_chat_members) => {
            for new_chat_member in new_chat_members {
                cx.reply_to(format!(
                    "Привет {}! Расскажи стишок про Rust",
                    new_chat_member.first_name
                ))
                .send()
                .await
                .expect("Что-то не так с ботом.");
            }
        }
    }
}

