use core::fmt;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GptMessage {
    pub role: String,
    pub content: String,
}

impl Display for GptMessage {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let res = write!(f, "");
        println!("{}", &self.content);
        res
    }
}

#[derive(Serialize, Debug)]
pub struct GptBody {
    pub model: String,
    pub messages: Vec<GptMessage>,
    pub temperature: f32,
    pub max_tokens: i32,
    pub top_p: f32,
    pub presence_penalty: f32,
    pub frequency_penalty: f32,
}

impl Default for GptBody {
    fn default() -> Self {
        GptBody {
            model: String::from("gpt-3.5-turbo-0125"),
            messages: vec![GptMessage {
                role: String::new(),
                content: String::new(),
            }],
            temperature: 1.0,
            max_tokens: 4000,
            top_p: 1.0,
            presence_penalty: 0.0,
            frequency_penalty: 0.0,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GptResponseChoice {
    pub index: i32,
    pub message: GptMessage,
    pub finish_reason: String,
}

impl Display for GptResponseChoice {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GptResponseUsage {
    pub prompt_tokens: u16,
    pub completion_tokens: u16,
    pub total_tokens: u16,
}
impl GptResponseUsage {
    pub fn _add(&mut self, new: &GptResponseUsage) {
        self.prompt_tokens += new.prompt_tokens;
        self.completion_tokens += new.completion_tokens;
        self.total_tokens += new.total_tokens;
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GptResponse {
    pub id: String,
    pub object: String,
    pub created: i32,
    pub model: String,
    pub choices: Vec<GptResponseChoice>,
    pub usage: GptResponseUsage,
}

impl Display for GptResponse {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.choices[0])
    }
}
