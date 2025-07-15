# nope.rs

## No as a Service.
A simple, localizable API that says “no” in many ways, across many languages.

⸻

✨ What is nope.rs?

A minimalist HTTP service that responds to your requests with localized, expressive denials.

Use it for:
	•	Denying access with flair
	•	Fun error messaging
	•	Saying “no” – programmatically

⸻

📦 Usage

GET https://nope.rs/

{
  "no": "Nope",
  "language": "en",
  "code": 403
}

➕ Parameters

Query Param	Description	Example
lang	ISO 639-1 language code	lang=es
tone	Optional: formal, casual	tone=casual


⸻

🌍 Supported Languages

We support most major languages and dialects.
Auto-detect coming soon.

⸻

🔌 Self-hosting

cargo run --release

Environment options:

Variable	Default	Description
PORT	8080	Port to run on


⸻

📜 License

MIT © nope.rs contributors

⸻

🛠️ Made With

Rust · Actix-Web · Lingo
