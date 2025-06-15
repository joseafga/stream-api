use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use rand::seq::IndexedRandom;

pub fn get_sentence_thyria(id: Option<usize>) -> String {
    let sentences = [
        "Não me tira, não me tira GlitchNRG", // 0
        "Antes que eu me esqueça, vai tomar no cool raissa11RaiBRAVA", // 1
        "Você é muito ruim, podre, lixo raissa11Faquinha", // 2
        "Vai tomando raissa11NEA",            // 3
        "Certeza que sua mãe usa saia Kappa", // 4
        "Beijos da tia Thy 🥰",               // 5
        "MANO, VOCÊ SABE QUE TEM 4 GEN UP SEU FILHO DA PULGA raissa11RaiBRAVA", // 6
        "Vai dar meia hora com relógio parado 🤭", // 7
        "Vai taca taca taca 🎶",              // 8
        "Bom dia senhora raissa112anosdecanal", // 9
        "Eu sou uma rata senhoraa raissa11RAI", // 10
        "Sua mãe toma banho em pé CoolCat",   // 11
        "Sua mãe dorme deitada NotLikeThis",  // 12
        "Sua mãe nasceu pelada BabyRage",     // 13
        "Vem meu casquinha de bala KomodoHype", // 14
        "Eu te batizo em nome de Jane Romero FBBlock", // 15
        "Toma então, batizado PowerUpR",      // 16
        "Bruxa, fedida, tomara que te dor de barriga 🎶", // 17
    ];

    let sentence = match id {
        Some(id) => sentences[id],
        None => {
            let mut rng = rand::rng();
            sentences.choose(&mut rng).unwrap()
        }
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

    let sentence = match id {
        Some(id) => sentences[id],
        None => {
            let mut rng = rand::rng();
            sentences.choose(&mut rng).unwrap()
        }
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
