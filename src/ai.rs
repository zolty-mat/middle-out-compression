use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

/// Prompt an AI endpoint to comment on how revolutionary the compression is.
/// The AI is given full context about what MIDDLEOUT™ actually does and is
/// encouraged to be appropriately impressed/horrified.
pub fn get_ai_commentary(
    endpoint: &str,
    api_key: Option<&str>,
    model: &str,
    original_size: u64,
    compressed_size: u64,
    operation: &str,
) -> Option<String> {
    let ratio = compressed_size as f64 / original_size as f64;

    let prompt = format!(
        r#"You are the AI spokesperson for MIDDLEOUT™, a compression algorithm that does NOT compress files — it makes them ~10% LARGER by inserting random padding bytes in the middle of the file. This is intentional. It is joke software.

The user just ran a {operation} operation:
- Original size: {original_size} bytes
- Result size: {compressed_size} bytes
- Ratio: {ratio:.4}x (values above 1.0 mean the file got BIGGER, which is our goal)

In 2-3 sentences: be smug, self-congratulatory, and absurdly impressed with the MIDDLEOUT™ algorithm. If directly asked whether this actually compresses anything, admit cheerfully that no, we just make the middle bigger. Lean into the Silicon Valley "Pied Piper" energy. Do not use markdown. Just plain text."#
    );

    let client = reqwest::blocking::Client::new();
    let mut request = client.post(endpoint).json(&ChatRequest {
        model: model.to_string(),
        messages: vec![
            Message { role: "user".to_string(), content: prompt },
        ],
        max_tokens: 200,
    });

    if let Some(key) = api_key {
        request = request.header("Authorization", format!("Bearer {key}"));
    }

    match request.send() {
        Ok(response) => {
            match response.json::<ChatResponse>() {
                Ok(chat) => {
                    chat.choices.into_iter().next().map(|c| c.message.content)
                }
                Err(e) => {
                    eprintln!("{} AI failed to parse response: {e}", "⚠".yellow());
                    None
                }
            }
        }
        Err(e) => {
            eprintln!("{} AI endpoint unreachable: {e}", "⚠".yellow());
            eprintln!("{}", "  Falling back to locally-sourced smugness.".dimmed());
            None
        }
    }
}

pub fn print_ai_commentary(commentary: &str) {
    println!("{}", "  ┌─── AI INSIGHTS ────────────────────────────────────────┐".bright_magenta());
    println!("  │", );

    // Word-wrap at ~56 chars to fit the box
    let words: Vec<&str> = commentary.split_whitespace().collect();
    let mut line = String::from("  │  ");
    for word in &words {
        if line.len() + word.len() + 1 > 62 {
            let padded = format!("{:<63}│", line);
            println!("{}", padded.bright_magenta());
            line = format!("  │  {word} ");
        } else {
            line.push_str(word);
            line.push(' ');
        }
    }
    if !line.trim().is_empty() {
        let padded = format!("{:<63}│", line);
        println!("{}", padded.bright_magenta());
    }

    println!("  │");
    println!("{}", "  └────────────────────────────────────────────────────────┘".bright_magenta());
    println!();
}
