use worker::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct SetWebhook<'a> {
    url: &'a str,
    allowed_updates: [&'a str; 1],
}

#[derive(Serialize, Deserialize)]
struct BanChatMember {
    chat_id: i64,
    user_id: i64,
}

#[derive(Serialize, Deserialize)]
struct Chat {
    id: i64,
}

#[derive(Serialize, Deserialize)]
struct ChatInviteLink {
    link: String,
}

#[derive(Serialize, Deserialize)]
struct User {
    id: i64,
    first_name: String,
    last_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ChatMemberMember {
    user: User,
}

#[derive(Serialize, Deserialize)]
struct ChatMemberUpdated {
    chat: Chat,
    new_chat_member: ChatMemberMember, // should be enum
    invite_link: Option<ChatInviteLink>,
}

#[derive(Serialize, Deserialize)]
struct Update {
    chat_member: Option<ChatMemberUpdated>,
}

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {

    let router = Router::new();

    router
        .post_async("/webhook/:webhook_secret", set_webhook)
        .post_async("/update/:update_secret", update)
        .run(req, env).await
}

pub async fn set_webhook<D>(_req: Request, ctx: RouteContext<D>) -> Result<Response> {
    match ctx.param_equals_secret("webhook_secret", "WEBHOOK_SECRET") {
        Ok(equal) => if !equal {
            return Ok(Response::empty()?.with_status(404));
        },
        Err(e) => return Response::error(e, 500),
    };
    let client = reqwest::Client::new();
    let bot_token = match ctx.secret("BOT_TOKEN") {
        Ok(token) => token.to_string(),
        Err(_) => return Response::error("BOT_TOKEN not set", 500),
    };
    let update_secret = match ctx.secret("UPDATE_SECRET") {
        Ok(secret) => secret.to_string(),
        Err(_) => return Response::error("UPDATE_SECRET not set", 500),
    };
    let webhook_url = match ctx.secret("WORKER_URL") {
        Ok(url) => url.to_string() + "/update/" + &update_secret,
        Err(_) => return Response::error("WORKER_URL not set", 500),
    };
    let data = SetWebhook { url: &webhook_url, allowed_updates: ["chat_member"] };
    let telegram_url = match ctx.var("TELEGRAM_URL") {
        Ok(url) => url.to_string(),
        Err(_) => return Response::error("TELEGRAM_URL not set", 500),
    };
    let api_url = telegram_url + &bot_token;
    let request = client.post(api_url + "/setWebhook")
        .json(&data)
        .send();
    
    match request.await {
        Ok(resp) => match resp.text().await {
            Ok(txt) => Response::ok(txt),
            Err(e) => Response::error(e.to_string(), 500),
        },
        Err(e) => Response::error(e.to_string(), 500),
    }
}

pub async fn update<D>(mut req: Request, ctx: RouteContext<D>) -> Result<Response> {
    let telegram_asn = 62041;
    // make sure request is coming from Telegram
    if req.cf().asn() != telegram_asn {
        return Ok(Response::empty()?.with_status(403));
    }
    match ctx.param_equals_secret("update_secret", "UPDATE_SECRET") {
        Ok(equal) => if !equal {
            return Ok(Response::empty()?.with_status(404));
        },
        Err(e) => return Response::error(e, 500),
    };

    let update: Update = match req.text().await {
        Ok(txt) => match serde_json::from_str(&txt) {
            Ok(data) => data,
            Err(_) => return Response::ok(""),
        },
        Err(_) => return Response::ok(""),
    };
    let chat_member: ChatMemberUpdated = match update.chat_member {
        Some(member) => member,
        None => return Response::ok(""),
    };
    let user = chat_member.new_chat_member.user;
    let name = match &user.last_name { Some(str) => [&user.first_name, " ", str].concat(), None => user.first_name.clone() };
    if is_name_banned(&name, "rm -rf HV CURSOS") {
        let token = match ctx.secret("BOT_TOKEN") {
            Ok(token) => token.to_string(),
            Err(_) => return Response::error("BOT_TOKEN not set", 500),
        };
        let telegram_url = match ctx.var("TELEGRAM_URL") {
            Ok(url) => url.to_string(),
            Err(_) => return Response::error("TELEGRAM_URL not set", 500),
        };
        let api_url = telegram_url + &token;
        ban_chat_member(&user, &chat_member.chat, &api_url).await?;
    }
    return Response::ok("");
}

async fn ban_chat_member(user: &User, chat: &Chat, api_url: &str) -> Result<Response> {
    let data = BanChatMember { chat_id: chat.id, user_id: user.id };
    let client = reqwest::Client::new();
    let request = client.post(String::from(api_url) + "/banChatMember")
        .json(&data)
        .send()
        .await;
    match request {
        Ok(_) => Response::ok(""),
        Err(_) => Response::error("", 500),
    }
}

fn is_name_banned(name: &str, bot_name: &str) -> bool {
    let lowercase_name = name.to_lowercase();
    let is_me = lowercase_name.eq(&bot_name.to_lowercase());
    if is_me {
        return false;
    }
    let banned_names = ["HV CURSOS", "Hadassa CURSOS", "Hadassa HV"];
    for banned in banned_names {
        if lowercase_name.eq(&banned.to_lowercase()) {
            return true;
        }
    }
    return false;
}

trait ParamSecretComparison {
    fn param_equals_secret(&self, param_name: &str, secret_name: &str) -> std::result::Result<bool, String>;
}

impl<D> ParamSecretComparison for RouteContext<D> {
    fn param_equals_secret(&self, param_name: &str, secret_name: &str) -> std::result::Result<bool, String> {
        let param = match self.param(param_name) {
            Some(p) => p,
            None => return Err(format!("param {} not found", param_name)),
        };
        match self.secret(secret_name) {
            Ok(secret) => Ok(&secret.to_string() == param),
            Err(_) => Err(format!("secret {} not found", secret_name)),
        }
    }
}
