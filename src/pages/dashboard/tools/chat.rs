use askama::Template;
use askama_axum::IntoResponse;
use axum::{extract::Path, Extension, Json};
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::query;
use sqlx::Row;

use crate::structs::open_ai::GptBody;
use crate::structs::open_ai::GptResponse;
use crate::structs::{database::DatabaseConnection, entity::user::User, open_ai::GptMessage};

#[derive(Debug, Deserialize)]
pub struct ChatPayload {
    pub prompt: String,
}

#[derive(Template)]
#[template(path = "components/tools/chat/user_item.html")]
pub struct ToolsChatUserItem {
    chat_idx: String,
    user_prompt: String,
}

pub async fn receive_chat(
    Extension(user): Extension<User>,
    DatabaseConnection(mut conn): DatabaseConnection,
    Json(payload): Json<ChatPayload>,
) -> Result<impl IntoResponse, (StatusCode, ToolsChatUserItem)> {
    let prompt = vec![GptMessage {
        role: String::from("user"),
        content: payload.prompt.clone(),
    }];
    let prompt_stringify = serde_json::to_string(&prompt).unwrap();

    let result = query!(
        r#"INSERT INTO tools_chat (user_id, idx, discusion, prompt_tokens, response_tokens, total_tokens) VALUES ($1, $2, $3, $4, $5, $6 ) RETURNING idx"#,
        user.id,
        uuid::Uuid::new_v4().to_string(),
        prompt_stringify,
        String::new(),
        String::new(),
        String::new()
    )
    .fetch_one(&mut *conn)
    .await;

    match &result {
        Ok(data) => Ok((
            StatusCode::OK,
            ToolsChatUserItem {
                chat_idx: data.idx.as_ref().unwrap().to_string(),
                user_prompt: payload.prompt,
            },
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            ToolsChatUserItem {
                chat_idx: String::new(),
                user_prompt: String::new(),
            },
        )),
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolsChat {
    pub id: i32,
    pub idx: String,
    pub discution: Vec<GptMessage>,
}

impl Default for ToolsChat {
    fn default() -> Self {
        ToolsChat {
            id: 0,
            idx: String::new(),
            discution: vec![GptMessage {
                role: String::new(),
                content: String::new(),
            }],
        }
    }
}

pub async fn handle_chat(
    Path(chat_idx): Path<String>,
    Extension(user): Extension<User>,
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<impl IntoResponse, (StatusCode, ToolsChatUserItem)> {
    let res =
        sqlx::query("SELECT id, idx, discusion FROM tools_chat WHERE user_id = $1 AND idx = $2")
            .bind(&user.id)
            .bind(&chat_idx)
            .map(|row: sqlx::postgres::PgRow| ToolsChat {
                id: row.get(0),
                idx: row.get(1),
                discution: serde_json::from_str(row.get(2)).unwrap(),
                ..Default::default()
            })
            .fetch_one(&mut *conn)
            .await
            .unwrap_or(ToolsChat {
                ..Default::default()
            });

    let gpt_body = GptBody {
        messages: res.discution,
        ..Default::default()
    };

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header(
            AUTHORIZATION,
            format!("Bearer {}", &std::env::var("OPEN_AI_KEY").unwrap()),
        )
        .header(CONTENT_TYPE, "application/json")
        .json(&gpt_body)
        .send()
        .await
        .expect("Request failed");

    match res.status() {
        reqwest::StatusCode::OK => match res.json::<GptResponse>().await {
            Ok(data) => {
                let mut messages: Vec<GptMessage> = vec![];
                for message in gpt_body.messages {
                    messages.push(message);
                }
                for choice in data.choices {
                    messages.push(choice.message);
                }
                let messages_stringify = serde_json::to_string(&messages).unwrap();

                let _ = query!(
                    r#"UPDATE tools_chat SET discusion = $1 WHERE idx = $2 AND user_id = $3;"#,
                    messages_stringify,
                    chat_idx,
                    user.id,
                )
                .execute(&mut *conn)
                .await;
                return Ok((
                    StatusCode::OK,
                    markdown::to_html(&messages.pop().unwrap().content),
                ));
            }
            Err(e) => panic!("{}", e.to_string()),
        },
        _other => println!("{:#?}", res.text().await),
    };
    Ok((StatusCode::BAD_REQUEST, String::new()))
}
