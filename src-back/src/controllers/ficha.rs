use rocket::{post, serde::json::Json};
use serde::{Deserialize, Serialize};

use super::informacoes_origens::{AtributosOrigem, TipoRegiao};

#[derive(Serialize, Deserialize, Debug)]
pub struct FichaBase{
    pub nome: String,
    pub nome_jogador: String,
    pub origem: AtributosOrigem,
    pub regiao: TipoRegiao,
    pub passado: TipoPassado,
    pub moral: TipoMoral,
    pub classe: TipoClasse,

    pub atributos: AtributosPersonagem,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AtributosPersonagem{
    pub forca: f32,
    pub destreza: f32,
    pub constituicao: f32,
    pub inteligencia: f32,
    pub sabedoria: f32,
    pub carisma: f32,

    pub vida_maxima: f32,
    pub vida_atual: f32,
    pub vida_temporaria: f32,

    pub classe_armadura: f32,
    pub bonus_escudo: Option<f32>,

    pub bonus_proficiencia: f32,
    
    pub experiencia: f32,
    pub nivel: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TipoClasse{
    Mercurial(InformacoesClasse)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InformacoesClasse{

}

#[derive(Serialize, Deserialize, Debug)]
pub enum TipoPassado{
    Heroi(InformacoesPassado)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TipoMoral{
    LealMal(InformacoesMoral)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InformacoesMoral{
    pub nome: String,
    pub descricao: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InformacoesPassado{
    pub nome: String,
    pub descricao: String
}

#[post("/cadastra_ficha", format="json", data="<ficha>")]
pub async fn cadastra_ficha(ficha: Json<FichaBase>) -> Json<FichaBase> {
    println!("{:?}", ficha);
    ficha

}