use log::*;
use serde_json::Value;
use std::{
    fs,
    process::{exit, ExitCode},
};
const FILECONFIG: &str = "config.json";
pub enum JsonBot {
    IdDonos,
    TokenBot,
    GruposPermitidos,
    PontosLinkDeReferencia,
    PontosInicial,
}

// privado
fn json_format(key: &str) -> Result<Value, ExitCode> {
    let fs = fs::read_to_string(FILECONFIG).unwrap_or_else(|e| {
        error!("Arquivo de configuração ausente");
        exit(1);
    });

    if let Ok(json) = serde_json::from_str::<Value>(&fs) {
        if let Some(value_ref) = json["bot"].get(key) {
            return Ok(value_ref.to_owned());
        }

        warn!("Chave inexistente");
        exit(1);
    }

    error!("Json no formato Errado");
    exit(1);
}

impl JsonBot {
    pub fn set_json(self) -> Value {
        match self {
            Self::GruposPermitidos => json_format("allowGrups").unwrap(),

            Self::IdDonos => json_format("id_onwer").unwrap(),

            Self::PontosInicial => json_format("pontos_inicial").unwrap(),
            Self::TokenBot => json_format("token_bot").unwrap(),

            Self::PontosLinkDeReferencia => json_format("link_referencia_recev").unwrap(),
        }
    }
}
