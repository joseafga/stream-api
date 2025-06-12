use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use rand::seq::IndexedRandom;
// use serde::Deserialize;

pub fn get_sentence_thyria(id: Option<usize>) -> String {
    let sentences = [
        "Não me tira, não me tira GlitchNRG",
        "Antes que eu me esqueça, vai tomar no cool raissa11RaiBRAVA",
        "Você é muito ruim, podre, lixo raissa11Faquinha",
        "Vai tomando raissa11NEA",
        "Certeza que sua mãe usa saia Kappa",
        "Beijos da tia Thy raissa11Coracao",
        "MANO, VOCÊ SABE QUE TEM 4 GEN UP SEU FILHO DA PULGA raissa11RaiBRAVA",
        "Vai dar meia hora com relógio parado raissa11Rindo",
        "Vai taca taca taca 🎶",
        "Bom dia senhora raissa112anosdecanal",
        "Eu sou uma rata senhoraa raissa11RAI",
        "Sua mãe toma banho em pé CoolCat",
        "Sua mãe dorme deitada NotLikeThis",
        "Sua mãe nasceu pelada BabyRage",
        "Vem meu casquinha de bala KomodoHype",
        "Eu te batizo em nome de Jane Romero FBBlock",
        "Toma então, batizado PowerUpR",
        "Bruxa, fedida, tomara que te dor de barriga 🎶",
    ];

    let mut rng = rand::rng();
    let sentence = match id {
        Some(id) => sentences[id],
        None => sentences.choose(&mut rng).unwrap(),
    };

    sentence.to_string()
}

pub fn get_sentence_jonhsullivan(id: Option<usize>) -> String {
    let sentences = [
        "Seven Minutes is all I can spare to play with you.",
        "Poor performance indeed.",
        "You disappoint me. Is that the best you`ve got?",
        "Is that all you have?",
        "O poderoso Nemesis.",
        "Main Killer safado.",
    ];

    let mut rng = rand::rng();
    let sentence = match id {
        Some(id) => sentences[id],
        None => sentences.choose(&mut rng).unwrap(),
    };

    sentence.to_string()
}

pub async fn get_sentence(Path(name): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    // Get vector from Streamelements argument
    let elements: Vec<String> = name
        .trim()
        .to_lowercase()
        .split_whitespace()
        .take(2)
        .map(str::to_string)
        .collect();

    // Filter possible second argument
    let arg: Option<usize> = elements.get(1).and_then(|s| s.parse::<usize>().ok());
    let sentence = match elements[0].as_str() {
        "thyria" => get_sentence_thyria(arg),
        "jonhsullivan" => get_sentence_jonhsullivan(arg),
        _ => String::from("Comando desconhecido"),
    };

    Ok(sentence)
}
