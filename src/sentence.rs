use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use rand::seq::IndexedRandom;

const NO_SENTENCE: &str = "nenhuma frase disponível.";

pub fn get_sentence_thyria(index: Option<usize>) -> String {
    let sentences = [
        "não me tira, não me tira GlitchNRG", // 0
        "antes que eu me esqueça, vai tomar no cool raissa11RaiBRAVA", // 1
        "você é muito ruim, podre, lixo raissa11Faquinha", // 2
        "vai tomando raissa11NEA",            // 3
        "certeza que sua mãe usa saia Kappa", // 4
        "beijos da tia Thy 🥰",               // 5
        "MANO, VOCÊ SABE QUE TEM 4 GEN UP SEU FILHO DA PULGA raissa11RaiBRAVA", // 6
        "vai dar meia hora com relógio parado 🤭", // 7
        "vai taca taca taca 🎶",              // 8
        "bom dia senhora raissa112anosdecanal", // 9
        "eu sou uma rata senhoraa raissa11RAI", // 10
        "sua mãe toma banho em pé CoolCat",   // 11
        "sua mãe dorme deitada NotLikeThis",  // 12
        "sua mãe nasceu pelada BabyRage",     // 13
        "vem meu casquinha de bala KomodoHype", // 14
        "eu te batizo em nome de Jane Romero FBBlock", // 15
        "toma então, batizado PowerUpR",      // 16
        "bruxa, fedida, tomara que te dor de barriga 🎶", // 17
    ];

    select_sentence(&sentences, index)
}

pub fn get_sentence_jonhsullivan(index: Option<usize>) -> String {
    let sentences = [
        "seven minutes is all I can spare to play with you.",
        "poor performance indeed.",
        "you disappoint me. Is that the best you`ve got?",
        "is that all you have?",
        "o poderoso Nemesis.",
        "main killer safado.",
    ];

    select_sentence(&sentences, index)
}

pub async fn get_sentence(Path(name): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    // Get vector from Streamelements argument
    let args: Vec<String> = name
        .trim()
        .to_lowercase()
        .split_whitespace()
        .take(2)
        .map(str::to_string)
        .collect();

    // Filter possible second argument
    let index: Option<usize> = args.get(1).and_then(|s| s.parse::<usize>().ok());
    let sentence = match args[0].as_str() {
        "thyria" => get_sentence_thyria(index),
        "jonhsullivan" => get_sentence_jonhsullivan(index),
        _ => String::from("comando desconhecido."),
    };

    Ok(sentence)
}

fn select_sentence(sentences: &[&str], index: Option<usize>) -> String {
    let sentence = match index {
        Some(i) => {
            if i < sentences.len() {
                sentences[i]
            } else {
                sentences.first().unwrap_or(&NO_SENTENCE)
            }
        }
        None => {
            let mut rng = rand::rng();
            sentences.choose(&mut rng).unwrap_or(&NO_SENTENCE)
        }
    };

    sentence.to_string()
}
