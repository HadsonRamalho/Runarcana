use std::vec;

use rocket::{get, serde::json::Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Linhagem{
    nome: String,
    descricao: String,
    heranca: Option<String>,
    quantidade_herancas: Option<u32>,
    deslocamento: f32,
    habilidades_especiais: Option<Vec<HabilidadeEspecial>>,

    bonus_constituicao: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct Variacao{
    nome: String,
    descricao: String,
    recarga: Option<String>,
    vantagens: Option<Vec<Vantagem>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AtributosOrigem {
    pub nome: String,
    pub resumo: String,
    pub idade_maxima: u32,
    pub tamanho: String,
    pub deslocamento: f32,
    pub idiomas: Vec<Idioma>,
    pub regiao: TipoRegiao,
    pub habilidades_especiais: Option<Vec<HabilidadeEspecial>>,
    pub linhagens: Option<Vec<Linhagem>>,

    pub quantidade_herancas: Option<u32>,
    pub quantidade_idiomas: u32,
    pub tamanho_minimo: f32,
    pub tamanho_maximo: f32,
    pub tipo_tamanho: TipoTamanho,
    pub pericias: Option<Vec<String>>,
    pub qtd_aprimoramentos: Option<u32>,
    pub oficios: Option<Vec<String>>,
    pub qtd_oficios: Option<u32>,

    pub descricoes_origem: DescricoesOrigem,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HabilidadeEspecial{
    pub nome: String,
    pub descricao: String,  
    pub variacoes: Option<Vec<Variacao>>,
    pub resistencias: Option<Vec<Resistencia>>,
    pub vulnerabilidades: Option<Vec<Vulnerabilidade>>,
    pub imunidades: Option<Vec<Imunidade>>,
    pub recarga: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Resistencia{
    Contundente,
}

#[derive(Serialize, Deserialize, Debug)]

pub enum Imunidade{
    MagiasEmHumanoides,
    Queda,
    MovimentoForcado
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Vantagem{
    Contundente,
    MovimentoForcado
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Vulnerabilidade{
    Igneo
}

#[derive(Serialize, Deserialize, Debug)]

pub struct DescricoesOrigem {
    pub heranca: String,
    pub idade: String,
    pub tendencia: String,
    pub tamanho: String,
    pub deslocamento: String,
    pub idiomas: String,
    pub proficiencia: Option<String>,
    pub regiao: String,

    pub pericia: Option<String>,
    pub aprimoramento: Option<String>,
    pub oficio: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]

pub enum TipoRegiao {
    Shurima,
    Noxus,
    Ionia,
    Demacia,
    Piltover,
    Zaun,
    Bandopolis,
    AguasDeSentina,
    Targon,
    IlhasDasSombras,
    Runeterra,
    Vazio,
    Ixtal,
    EscolhaLivre
}

#[derive(Serialize, Deserialize, Debug)]

pub enum TipoOrigem {
    Humano(AtributosOrigem),
    Atroplantae(AtributosOrigem),
}

#[derive(Serialize, Deserialize, Debug)]

pub enum Idioma {
    Comum,
    EscolhaLivre,
    Silvestre,
    FalarComPlantas
}

#[derive(Serialize, Deserialize, Debug)]

pub enum TipoTamanho {
    Pequeno,
    Medio,
    Grande,
}

pub fn info_humanos() -> AtributosOrigem {
    let resumo = "Os membros de origem humana estão entre os mais mais numerosos de Runeterra, as principais cidades-estados do mundo são repletas deles e normalmente controladas pelos membros dessa espécie.
    Com uma distribuição majoritária, eles podem ser encontrados em todos os lugares, desde Demacia até Noxus, passando por Freljord, Ionia, Águas de Sentina ou até mesmo em Shurima, que tem se reerguido das areias.
    Embora eles todos tenham características bem similares, algumas pequenas nuances existem por conta do local de nascimento deles.".to_string();
 
    let humano = AtributosOrigem{
        nome: "Humano".to_string(),
        resumo,
        idade_maxima: 100,
        tamanho: "Médio".to_string(),
        deslocamento: 30.0,
        idiomas: vec![Idioma::Comum, Idioma::EscolhaLivre],
        regiao: TipoRegiao::EscolhaLivre,
        habilidades_especiais: None,
        linhagens: None,

        quantidade_herancas: Some(3),
        quantidade_idiomas: 1,
        tamanho_minimo: 1.5,
        tamanho_maximo: 1.8,
        tipo_tamanho: TipoTamanho::Medio,
        pericias: Some(vec!["Escolha Livre".to_string()]),
        qtd_aprimoramentos: Some(1),
        oficios: None,
        qtd_oficios: Some(1),
        descricoes_origem: DescricoesOrigem{
            heranca: "Você possui 3 pontos de Herança, veja o Capítulo 5:
Personalização para a lista de Heranças disponíveis.".to_string(),
            idade: "Os humanos chegam à idade adulta no final da adolescência e
vivem menos de um século.".to_string(),
            tendencia: "Os humanos não possuem inclinação a nenhuma tendência
em especial. Os melhores e os piores são encontrados entre eles.".to_string(),
            tamanho: "Os humanos variam muito em altura e peso, podem ter quase
1,5 metro ou mais de 1,8 metro. Independentemente da sua posição
entre esses valores, o seu tamanho é Médio.".to_string(),
            deslocamento: "Seu deslocamento base de caminhada é 30 pés.".to_string(),
            idiomas: "Você pode falar, ler e escrever um idioma comum e outro idioma
comum adicional, normalmente baseado na região em que cresceu.".to_string(),
            proficiencia: Some("Você adquire um aprimoramentos de sua escolha.".to_string()),
            regiao: "Cada região de Runeterra possui uma cultura diferente. Você
deve escolher sua Região que definirá onde você cresceu e muito de
como você vê o mundo. Você pode ver mais a respeito disso no Capítulo
11: Runeterra.".to_string(),
            pericia: Some("Você ganha proficiência em uma perícia, à sua escolha.".to_string()),
            aprimoramento: Some("Você adquire um aprimoramentos de sua escolha.".to_string()),
            oficio: Some("Você possui proficiência com um ofício à sua escolha.".to_string()),
        },
    };

    return humano
}

fn info_antroplantae() -> AtributosOrigem{
    let resumo = "Os Antroplantæ são uma origem ligada fortemente à natureza e às plantas, embora se relacionem também com as pedras e a terra. 
    Em Runeterra a vida encontra muitas formas de se manifestar abençoada pela magia, não é estranho que formas únicas surjam muitas vezes sem fazerem parte de origem mais ampla ou de uma espécie estruturada, são membros únicos e tão diferentes como os galhos de uma árvore.".to_string();
    
    let antroplantae = AtributosOrigem{
        nome: "Antroplantae".to_string(),
        resumo,
        idade_maxima: 10000,
        tamanho: "Médio".to_string(),
        deslocamento: 30.0,
        idiomas: vec![Idioma::Comum, Idioma::EscolhaLivre, Idioma::Silvestre, Idioma::FalarComPlantas],
        regiao: TipoRegiao::EscolhaLivre,
        habilidades_especiais: Some(vec![
            HabilidadeEspecial{nome:"Fotossíntese".to_string(),descricao:"Antroplantæ não tem necessidade de se alimentar, mas
devem passar pelo menos 4 horas do dia no sol e tomar 5 litros de água
por dia, caso contrário recebem 1 nível de exaustão.".to_string(),variacoes:None,resistencias:None,vulnerabilidades:None,
    imunidades: None, recarga: None },
            HabilidadeEspecial{
                nome: "Vegetação".to_string(),
                descricao: "Antroplantæ recebem desvantagem em todas as
salvaguardas contra efeitos relacionados ao fogo. Os Antroplantæ têm
vulnerabilidade a dano ígneo.".to_string(),
            variacoes: None,
            vulnerabilidades: Some(vec![Vulnerabilidade::Igneo]),
            resistencias: None, imunidades: None, recarga: None 
            },
            HabilidadeEspecial{nome:"Enraizar".to_string(),descricao:"Embora não precisem dormir, os Antroplantæ devem se
enraizar todas as noites por pelo menos 8 horas. Enquanto estão
enraizado eles se tornam imóveis embora possam fazer ações leves se
necessário. Estão cientes dos seus arredores. Antroplantæ só podem se
enraizar em areia, terra, lama e cascalho; eles não podem se enraizar
em rocha sólida ou em um chão de pedra trabalhado. Um Antroplantæ
que não consiga se enraizar por uma noite, ganha 1 nível de exaustão e
não é beneficiado por um descanso longo. Arbóreos tem vantagem em
testes de Enganação para disfarçarem-se de árvore, assim como Florais
para vegetação.".to_string(),variacoes:None,
    resistencias: None,
    vulnerabilidades: None, imunidades: None, recarga: None , },
            HabilidadeEspecial{nome:"Não-humanoides".to_string(),descricao:"Antroplantæ não podem ser alvos de magias que
afetem humanoides, como enfeitiçar pessoa. No entanto eles contam
como plantas para magias que alvejam plantas, como malogro.".to_string(),variacoes:None,
    resistencias: None,
    vulnerabilidades: None, imunidades: None, recarga: None  }
        ]),
        linhagens: Some(
            vec![
                Linhagem{
                    nome: "Arbóreo".to_string(),
                    descricao: "Arbóreos são seres mais robustos quando comparados com os Florais,
    sua casca grossa os ajudam a se manter por muito tempo em combate,
    mas possuem a tendência de ajudar aqueles que sejam bons com sua
    floresta, como regar as plantas, adubar o solo, proteger a vida local, etc.".to_string(),
                    heranca: Some("Seu valor de Constituição aumenta em +1, e você possui 2
    pontos de Herança, veja o Capítulo 5: Personalização para a lista de
    Heranças disponíveis.".to_string()),
                    quantidade_herancas: Some(2),
                    deslocamento: 25.0,
                    habilidades_especiais: Some(vec![
                        HabilidadeEspecial{nome:"Pele de Árvore".to_string(),descricao:"Um Arbóreo não pode usar armadura, no entanto sua
                casca provê defesa natural. A CA de um Arbóreo é igual a 10 + seu
                modificador de Destreza + modificador de Constituição.".to_string(),
                variacoes:None,
        resistencias: None,
        vulnerabilidades: None, imunidades: None, recarga: None  },
                        HabilidadeEspecial{nome:"Enraizamento de Batalha".to_string(),descricao:"Usando uma ação, os arbóreos podem se
    enraizar no calor da batalha, caso o chão permita. Isso faz a velocidade
    deles cair para 0, mas eles se tornam imunes a queda e a movimentos
    forçados. Eles podem se desenraizar como uma ação bônus.".to_string(),variacoes:None, recarga: None ,
        resistencias: None,
        vulnerabilidades: None,
        imunidades: Some(vec![Imunidade::Queda, Imunidade::MovimentoForcado]), },
            HabilidadeEspecial{
                nome: "Duro como um Tronco".to_string(),
                descricao: "Arbóreos tem resistência a ataques contundentes.".to_string(),
                variacoes: None,
                resistencias: Some(vec![Resistencia::Contundente]),
                vulnerabilidades: None,
                imunidades: None, 
                recarga: None 
            },
            HabilidadeEspecial{nome:"Regeneração".to_string(),descricao:"Você pode uma vez a cada dois dias, regenerar um membro
    perdido durante um descanso longo.".to_string(),
                variacoes: None,
                resistencias: None,
                vulnerabilidades: None,
                imunidades: None,
                recarga: Some("2 dias".to_string()),
            },
            HabilidadeEspecial{nome:"Fibras".to_string(),descricao:"Alguns Arbóreos podem ter propriedades especiais por conta da
    madeira do qual são feitos ou das folhas que produzem. Essas variações
    não são obrigatórias, mas pode-se selecionar apenas uma entre as
    seguintes.".to_string(),
        variacoes: Some(vec![Variacao{
            nome: "Carvalho".to_string(),
            descricao: "Mesmo quando não enraizados, esses Arbóreos têm vantagem
    em qualquer salvaguarda para resistir a movimento forçado. Você pesa
    uma categoria acima da sua atual.".to_string(),
            recarga: None,
            vantagens: Some(vec![Vantagem::MovimentoForcado]),
        },
        Variacao{
            nome: "Cedro".to_string(),
            descricao: "Você pode utilizar a magia criar muda como se fosse uma magia de 1°
    nível, fazendo com que a muda cause 2d4 de dano ao invés de 4d4, e a área
    de sua explosão se torna um raio de 5 pés ao invés de 10. Quando você
    atingir o nível 5 poderá usar poderá usá-la no 3o nível normalmente, e no
    nível 7 poderá conjurá-la como uma magia de 4o nível.".to_string(),
            recarga: None,
            vantagens: None,
        },
        Variacao{
            nome: "Cerejeira".to_string(),
            descricao: "Você pode duplicar os efeitos da magia de 1o nível névoa
    obscurecente soltando todas as suas flores de uma vez, que formam uma
    esfera de 20 pés de raio a partir de você. Uma vez que o Arbóreo use
    essa habilidade, ele não pode fazer novamente antes de completar um
    descanso longo.".to_string(),
            recarga: Some("Descanso longo".to_string()),
            vantagens: None,
        },
        Variacao{
            nome: "Ferrobrasa".to_string(),
            descricao: "Você emana calor e seus ataques desarmados recebem dano
    ígneo adicional igual ao seu modificador de Constituição. Além disso,
    uma vez por ano você pode gerar uma Semente de Ferrobrasa.".to_string(),
            recarga: None,
            vantagens: None,
        },
        Variacao{nome:"Frutífero".to_string(),descricao:"Ao se enraizar por 8 horas, você produz frutas suficientes para
    4 rações diárias.".to_string(),
        recarga: None,
        vantagens: None, },
        Variacao{
            nome: "Olmo".to_string(),
            descricao: "Você sabe e pode usar os truques druidismo e orientação.".to_string(),
            recarga: None,
            vantagens: None
        },
        Variacao{
            nome: "Petricita".to_string(),
            descricao: "Toda magia lançada a até 5 pés de você cura você um total de
    pontos de vida igual ao nível da magia.".to_string(),
            recarga: None,
            vantagens: None,
        },
        Variacao{
            nome: "Salgueiro".to_string(),
            descricao: "Você pode usar a magia amizade animal, você a conjura como
    uma magia de 2o nível no seu nível 3, de 3o nível no nível 5 e de 4o no
    nível 7.".to_string(),
            recarga: None,
            vantagens: None,
        },
        Variacao{
            nome: "Sequoia".to_string(),
            descricao: "Nega os efeitos de “Vegetação”, mas efeitos ígneos prolongados
    duram o dobro.".to_string(),
            recarga: None,
            vantagens: None,
        },
        Variacao{
            nome: "Seringueira".to_string(),
            descricao: "Quando exposto diretamente à luz do sol, você pode usar
    um Dado de Vida para se curar como uma ação contanto que não esteja
    fazendo um descanso curto. Você pode também gastar 1 Dado de Vida
    para regenerar membros perdidos.".to_string(),
            recarga: None,
            vantagens: None,
        },
        Variacao{
            nome: "Teixo".to_string(),
            descricao: "Você pode usar a magia ver o invisível um número de vezes igual a
    metade da sua proficiência (arredondado para baixo) por descanso longo.".to_string(),
            recarga: None,
            vantagens: None,
        }
        ]),
        resistencias: None,
        vulnerabilidades: None,
        imunidades: None,
        recarga: None, }
                ]),
                bonus_constituicao: Some(2),
            },
            Linhagem{
                nome: "Floral".to_string(),
                descricao: "Florais são seres que possuem uma aptidão mágica excepcional,
                entretanto acabam por ser mais vulneráveis, Florais acabam por ser mais
                agressivos e violentos com aqueles que destroem o seu lar, podendo
                envenenar ou prender uma criatura em seu território para sempre.".to_string(),
                heranca: Some("Seu valor de Sabedoria aumenta em +1, e você possui 2 pontos
                de Herança, veja o Capítulo 5: Personalização para a lista de Heranças
                disponíveis.".to_string()),
                quantidade_herancas: Some(1),
                deslocamento: 30.0,
                habilidades_especiais: None,
                bonus_constituicao: None
                
            }
            ]
        ),

        quantidade_herancas: Some(3),
        quantidade_idiomas: 1,
        tamanho_minimo: 1.5,
        tamanho_maximo: 2.9,
        tipo_tamanho: TipoTamanho::Medio,
        pericias: Some(vec!["Escolha Livre".to_string()]),
        qtd_aprimoramentos: Some(1),
        oficios: None,
        qtd_oficios: Some(1),
        descricoes_origem: DescricoesOrigem{
            heranca: "Você possui 3 pontos de Herança, veja o Capítulo 5:
Personalização para a lista de Heranças disponíveis.".to_string(),
            idade: "A vida dos Antroplantæ é algo estranho, enquanto arbóreos
alcançam a maturidade entre 8 e 15 anos, Florais podem alcançar a
maturidade em 1 ou 2 anos. No entanto, podem vir a viver por milênios
dependendo da sua vida.".to_string(),
            tendencia: "Um Antroplantæ Floral adulto tende a ter um tamanho médio
entre 1,5 a 2,1 metros, enquanto um Arbóreo pode variar de 2,1 a 2,9
metros. Para todos os efeitos seu tamanho é Médio.".to_string(),
            tamanho: "Os humanos variam muito em altura e peso, podem ter quase
1,5 metro ou mais de 1,8 metro. Independentemente da sua posição
entre esses valores, o seu tamanho é Médio.".to_string(),
            deslocamento: "Seu deslocamento base de caminhada é 30 pés.".to_string(),
            idiomas: "Antroplantæ podem falar Silvestre e um idioma comum
adicional à sua escolha, normalmente relacionado a região mais próxima
da qual você cresceu. Eles podem se comunicar com qualquer árvore de
seu próprio tipo como se usassem a magia falar com plantas.".to_string(),
            proficiencia: Some("Você adquire um aprimoramentos de sua escolha.".to_string()),
            regiao: "Cada região de Runeterra possui uma cultura diferente. Você
deve escolher sua Região que definirá onde você cresceu e muito de
como você vê o mundo. Você pode ver mais a respeito disso no Capítulo
11: Runeterra.".to_string(),
            pericia: Some("Você ganha proficiência em uma perícia, à sua escolha.".to_string()),
            aprimoramento: Some("Você adquire um aprimoramentos de sua escolha.".to_string()),
            oficio: Some("Você possui proficiência com um ofício à sua escolha.".to_string()),
        },
    };

    return antroplantae
}

pub fn info_origens() -> Vec<AtributosOrigem> {
    let mut origens = vec![];
    origens.push(info_humanos());
    origens.push(info_antroplantae());

    return origens;
}

#[get("/")]
pub async fn lista_origens() -> Json<Vec<AtributosOrigem>>{
    let origens = info_origens();
    return Json(origens);
}