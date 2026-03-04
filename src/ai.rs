use serde_json::json;

pub fn ask_ai(error: &str) -> Option<String> {
    let api_key = std::env::var("GROQ_API_KEY").ok()?;

    let body = json!({
        "model": "llama-3.3-70b-versatile",
        "max_tokens": 300,
        "messages": [{
            "role": "user",
            "content": format!(
                "You are a developer assistant. Analyze this error and give:
1. Error type (1 line)
2. Cause (1-2 lines)
3. Fix (2-3 lines max)

Be concise and practical. Error:
{}", error
            )
        }]
    });

    let client = reqwest::blocking::Client::new();

    let res = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .ok()?;

    let json: serde_json::Value = res.json().ok()?;
    let text = json["choices"][0]["message"]["content"]
        .as_str()?
        .to_string();

    Some(text)
}