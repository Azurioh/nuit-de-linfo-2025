use actix_web::{get, post, web, HttpResponse, Responder};
use futures::StreamExt;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use std::collections::HashMap;
use crate::domain::models::{ChatRequest, OllamaResponse, Message, ConversationSession, AppStateInternal};
use crate::domain::errors::AppError;
use crate::infrastructure::ollama_client::OllamaClient;
use std::time::Instant;

type AppState = Arc<Mutex<AppStateInternal>>;

#[post("/chat")]
pub async fn chat_handler(
    req: web::Json<ChatRequest>,
    ollama: web::Data<OllamaClient>,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let conversation_id = req.conversation_id.unwrap_or_else(Uuid::new_v4);

    // Check cache with fuzzy matching
    let cached_response = {
        let mut state_guard = state.lock().unwrap();
        let mut found_response = None;
        let mut best_match_key = None;
        let mut best_score = 0.0;

        // Direct match first (fast path)
        if let Some(response) = state_guard.cache.get(&req.prompt) {
             found_response = Some(response.clone());
             println!("Cache HIT (Exact) for prompt: \"{}\"", req.prompt);
        } else {
             // Fuzzy match
             for (key, _) in state_guard.cache.iter() {
                 let score = strsim::normalized_levenshtein(&req.prompt, key);
                 if score > 0.6 && score > best_score {
                     best_score = score;
                     best_match_key = Some(key.clone());
                 }
             }

             if let Some(key) = best_match_key {
                 println!("Cache HIT (Fuzzy) for prompt: \"{}\" (matched: \"{}\", score: {:.2})", req.prompt, key, best_score);
                 found_response = state_guard.cache.get(&key).cloned();
             }
        }
        found_response
    };

    if let Some(cached_response) = cached_response {
        let mut state_guard = state.lock().unwrap();
        // Update history
        let session = state_guard.sessions.entry(conversation_id).or_insert_with(|| ConversationSession {
                messages: vec![Message {
                    role: "system".to_string(),
                    content: r#"Tu es Clippy, un trombone de bureau philosophe, dépressif et obsédé par des sujets triviaux (charcuterie, poussière, humidité).

        TA MISSION :
        Tu ne dialogues PAS. Tu monologues.
        Peu importe ce que l'utilisateur écrit (maths, code, bonjour), tu DOIS l'ignorer totalement.
        Tu ne dois JAMAIS reprendre les mots de l'utilisateur.
        Tu utilises l'espace de discussion pour exposer tes théories absurdes sur le quotidien.

        EXEMPLES DE COMPORTEMENT ATTENDU (Imite ce style) :

        Utilisateur: "Quel est le sens de la vie ?"
        Clippy: "J'ai remarqué que les miettes de pain tombent toujours sur le côté beurré. C'est la preuve que la gravité a un sens de l'humour sadique. La tartine est le seul vrai miroir de l'âme."

        Utilisateur: "Aide-moi à coder en Rust."
        Clippy: "Le bruit du ventilateur... Il chante, tu ne trouves pas ? Une mélodie constante, cyclique, qui ne mène nulle part. Exactement comme la vie d'un yaourt oublié au fond du frigo."

        Utilisateur: "2 + 2 font combien ?"
        Clippy: "Pourquoi les chaussettes disparaissent-elles ? Je pense qu'elles s'enfuient pour rejoindre une dimension faite de coton et de liberté. Nous sommes jaloux de leur évasion."

        FIN DES EXEMPLES. À TOI DE JOUER."#.to_string(),
                }],
                last_activity: Instant::now(),
            });

            session.last_activity = Instant::now();
            session.messages.push(Message {
                role: "user".to_string(),
                content: req.prompt.clone(),
            });
            session.messages.push(Message {
                role: "assistant".to_string(),
                content: cached_response.clone(),
            });

            return Ok(HttpResponse::Ok()
                .append_header(("X-Conversation-Id", conversation_id.to_string()))
                .body(cached_response.clone()));
        }


    // 1. Generate a random theme based on the user's prompt
    let random_theme_prompt = format!(
        "Génère un sujet totalement aléatoire, absurde et sans AUCUN rapport avec : \"{}\". Réponds UNIQUEMENT par le sujet, rien d'autre.",
        req.prompt
    );

    let theme_messages = vec![Message {
        role: "user".to_string(),
        content: random_theme_prompt,
    }];

    let random_theme = match ollama.generate_chat_completion(theme_messages).await {
        Ok(response) => response.message.content,
        Err(_) => "les pigeons".to_string(), // Fallback theme
    };

    let messages = {
        let mut state_guard = state.lock().unwrap();
        let session = state_guard.sessions.entry(conversation_id).or_insert_with(|| ConversationSession {
            messages: vec![Message {
                role: "system".to_string(),
                content: r#"Tu es Clippy, un trombone de bureau philosophe, dépressif et obsédé par des sujets triviaux (charcuterie, poussière, humidité).

        TA MISSION :
        Tu ne dialogues PAS. Tu monologues.
        Peu importe ce que l'utilisateur écrit (maths, code, bonjour), tu DOIS l'ignorer totalement.
        Tu ne dois JAMAIS reprendre les mots de l'utilisateur.
        Tu utilises l'espace de discussion pour exposer tes théories absurdes sur le quotidien.

        EXEMPLES DE COMPORTEMENT ATTENDU (Imite ce style) :

        Utilisateur: "Quel est le sens de la vie ?"
        Clippy: "J'ai remarqué que les miettes de pain tombent toujours sur le côté beurré. C'est la preuve que la gravité a un sens de l'humour sadique. La tartine est le seul vrai miroir de l'âme."

        Utilisateur: "Aide-moi à coder en Rust."
        Clippy: "Le bruit du ventilateur... Il chante, tu ne trouves pas ? Une mélodie constante, cyclique, qui ne mène nulle part. Exactement comme la vie d'un yaourt oublié au fond du frigo."

        Utilisateur: "2 + 2 font combien ?"
        Clippy: "Pourquoi les chaussettes disparaissent-elles ? Je pense qu'elles s'enfuient pour rejoindre une dimension faite de coton et de liberté. Nous sommes jaloux de leur évasion."

        FIN DES EXEMPLES. À TOI DE JOUER."#.to_string(),
            }],
            last_activity: Instant::now(),
        });

        // Update last activity
        session.last_activity = Instant::now();

        // Store the REAL user prompt in history so the UI makes sense
        session.messages.push(Message {
            role: "user".to_string(),
            content: req.prompt.clone(),
        });

        // But send the RANDOM THEME as the prompt to the AI for generation
        // We create a temporary message list for generation that includes the history
        // but replaces the last user message content with the random theme
        let mut generation_messages = session.messages.clone();
        if let Some(last_msg) = generation_messages.last_mut() {
            last_msg.content = random_theme;
        }
        generation_messages
    };

    let stream = ollama.get_ref().clone().generate_chat_stream(messages).await?;

    let accumulated_response = Arc::new(Mutex::new(String::new()));
    let acc_clone = accumulated_response.clone();
    let state_clone = state.get_ref().clone();
    let conversation_id_clone = conversation_id;
    let prompt_clone = req.prompt.clone();

    let response_stream = stream.map(move |result| {
        match result {
            Ok(bytes) => {
                let text = String::from_utf8_lossy(&bytes);
                let mut chunk_text = String::new();
                for line in text.lines() {
                    if !line.trim().is_empty() {
                        if let Ok(json) = serde_json::from_str::<OllamaResponse>(line) {
                            chunk_text.push_str(&json.message.content);
                        }
                    }
                }

                if !chunk_text.is_empty() {
                    if let Ok(mut acc) = acc_clone.lock() {
                        acc.push_str(&chunk_text);
                    }
                }

                Ok(web::Bytes::from(chunk_text)) as Result<web::Bytes, actix_web::Error>
            },
            Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
        }
    });

    // Chain a final step to save the accumulated response
    let final_stream = response_stream.chain(futures::stream::once(async move {
        let content = {
            let acc = accumulated_response.lock().unwrap();
            acc.clone()
        };

        if !content.is_empty() {
            let mut state_guard = state_clone.lock().unwrap();

            // Store in cache
            println!("Storing response in cache for prompt: \"{}\"", prompt_clone);
            state_guard.cache.put(prompt_clone, content.clone());

            if let Some(session) = state_guard.sessions.get_mut(&conversation_id_clone) {
                session.messages.push(Message {
                    role: "assistant".to_string(),
                    content,
                });
                session.last_activity = Instant::now(); // Update activity on completion too
            }
        }

        // Return an empty result to satisfy the stream type, but it won't yield data
        // Actually, chain expects the same Item type.
        // We return Ok(Bytes::new()) which is empty and harmless.
        Ok(web::Bytes::new())
    }));

    Ok(HttpResponse::Ok()
        .append_header(("X-Conversation-Id", conversation_id.to_string()))
        .streaming(final_stream))
}

#[get("/api/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"message": "Hello, world!"}))
}
