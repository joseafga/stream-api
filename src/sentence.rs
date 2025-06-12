use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use rand::seq::IndexedRandom;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Params {
    id: Option<usize>,
}

pub async fn get_sentence_thyria_rng() -> Result<impl IntoResponse, StatusCode> {
    let params = Params { id: None };

    get_sentence_thyria(Path(params)).await
}

pub async fn get_sentence_thyria(
    Path(Params { id }): Path<Params>,
) -> Result<impl IntoResponse, StatusCode> {
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

    Ok(sentence.to_string())
}
