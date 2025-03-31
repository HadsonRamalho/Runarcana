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
    bonus_sabedoria: Option<u32>,

    altera_calculo_ca: bool,
    possui_cd_propria: bool
    
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
    pub idiomas: Vec<String>,
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
    pub recarga: Option<String>,

    pub qtd_proficiencias: Option<u32>,
    pub proficiencias: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Resistencia{
    Contundente,
    Veneno,
    DanoPsiquico
}

#[derive(Serialize, Deserialize, Debug)]

pub enum Imunidade{
    MagiasEmHumanoides,
    Queda,
    MovimentoForcado,
    Doenca,
    Comer,
    Beber,
    Respirar,
    Dormir,
    Sono
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Vantagem{
    Contundente,
    MovimentoForcado,
    Enfeiticado,
    Veneno
}



#[derive(Serialize, Deserialize)]
pub enum Desvantagem{
    Clima,
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
        idiomas: vec!["Comum".to_string(), "Escolha Livre".to_string()],
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
        idiomas: vec![ "Comum".to_string(), "Silvestre".to_string(), "Falar com Plantas".to_string(), "Escolha Livre".to_string()],
        regiao: TipoRegiao::EscolhaLivre,
        habilidades_especiais: Some(vec![
            HabilidadeEspecial{nome:"Fotossíntese".to_string(),descricao:"Antroplantæ não tem necessidade de se alimentar, mas
devem passar pelo menos 4 horas do dia no sol e tomar 5 litros de água
por dia, caso contrário recebem 1 nível de exaustão.".to_string(),variacoes:None,resistencias:None,vulnerabilidades:None,
proficiencias: None,
qtd_proficiencias: None,
    imunidades: None, recarga: None },
            HabilidadeEspecial{
                nome: "Vegetação".to_string(),
                descricao: "Antroplantæ recebem desvantagem em todas as
salvaguardas contra efeitos relacionados ao fogo. Os Antroplantæ têm
vulnerabilidade a dano ígneo.".to_string(),
            variacoes: None,
            vulnerabilidades: Some(vec![Vulnerabilidade::Igneo]),
            resistencias: None, imunidades: None, recarga: None,
            proficiencias: None,
            qtd_proficiencias: None,
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
    vulnerabilidades: None, imunidades: None, recarga: None , 
    proficiencias: None,
    qtd_proficiencias: None,},
            HabilidadeEspecial{nome:"Não-humanoides".to_string(),descricao:"Antroplantæ não podem ser alvos de magias que
afetem humanoides, como enfeitiçar pessoa. No entanto eles contam
como plantas para magias que alvejam plantas, como malogro.".to_string(),variacoes:None,
    resistencias: None,
    vulnerabilidades: None, imunidades: None, recarga: None,
    proficiencias: None,
    qtd_proficiencias: None,  }
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
                    bonus_sabedoria: None,
                    deslocamento: 25.0,
                    
                    habilidades_especiais: Some(vec![
                        HabilidadeEspecial{nome:"Pele de Árvore".to_string(),descricao:"Um Arbóreo não pode usar armadura, no entanto sua
                casca provê defesa natural. A CA de um Arbóreo é igual a 10 + seu
                modificador de Destreza + modificador de Constituição.".to_string(),
                variacoes:None,
        resistencias: None,
        vulnerabilidades: None, imunidades: None, recarga: None,
        proficiencias: None,
        qtd_proficiencias: None,  },
                        HabilidadeEspecial{nome:"Enraizamento de Batalha".to_string(),descricao:"Usando uma ação, os arbóreos podem se
    enraizar no calor da batalha, caso o chão permita. Isso faz a velocidade
    deles cair para 0, mas eles se tornam imunes a queda e a movimentos
    forçados. Eles podem se desenraizar como uma ação bônus.".to_string(),variacoes:None, recarga: None ,
        resistencias: None,
        proficiencias: None,
        qtd_proficiencias: None,
        vulnerabilidades: None,
        imunidades: Some(vec![Imunidade::Queda, Imunidade::MovimentoForcado]), },
            HabilidadeEspecial{
                nome: "Duro como um Tronco".to_string(),
                descricao: "Arbóreos tem resistência a ataques contundentes.".to_string(),
                variacoes: None,
                resistencias: Some(vec![Resistencia::Contundente]),
                vulnerabilidades: None,
                imunidades: None, 
                proficiencias: None,
                qtd_proficiencias: None,
                recarga: None 
            },
            HabilidadeEspecial{nome:"Regeneração".to_string(),descricao:"Você pode uma vez a cada dois dias, regenerar um membro
    perdido durante um descanso longo.".to_string(),
                variacoes: None,
                resistencias: None,
                vulnerabilidades: None,
                imunidades: None,
                recarga: Some("2 dias".to_string()),
                proficiencias: None,
                qtd_proficiencias: None,
            },
            HabilidadeEspecial{nome:"Fibras".to_string(),descricao:"Alguns Arbóreos podem ter propriedades especiais por conta da
            madeira do qual são feitos ou das folhas que produzem. Essas variações
            não são obrigatórias, mas pode-se selecionar apenas uma entre as
            seguintes.".to_string(),
            proficiencias: None,
            qtd_proficiencias: None,
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
                altera_calculo_ca: false,
                possui_cd_propria: false
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
                quantidade_herancas: Some(2),
                deslocamento: 30.0,
                habilidades_especiais: Some(vec![
                    HabilidadeEspecial{
                        nome: "Chicote de Espinhos".to_string(),
                        descricao: "Você pode criar um chicote a partir de seu próprio
                        corpo. Esse chicote possui 1d4 + metade do seu modificador de
                        Constituição (arredondado para baixo) de dano perfurante e é uma
                        arma com a propriedade acuidade (você não pode adicionar Força no
                        dano desse chicote). Seus chicotes podem ser destruídos por magias de
                        fogo. Você pode utilizar seu chicote como uma
                        ação bônus e pode criá-lo utilizando uma
                        ação. Você só pode ter um chicote
                        ativo por vez, podendo criar um número igual ao seu modificador de Constituição por descanso
                        longo. Quando atingir o nível 7 você poderá ter 2 chicotes ativos por
                        vez, passando a utilizar uma ação e uma ação bônus para atacar com
                        eles.".to_string(),
                        variacoes: None,
                        resistencias: None,
                        vulnerabilidades: None,
                        imunidades: None,
                        recarga: None,
                        proficiencias: None,
                        qtd_proficiencias: None,
                    },
                    HabilidadeEspecial{
                        nome: "Regeneração Acelerada".to_string(),
                        descricao: "Em descansos curtos, você pode regenerar um membro perdido.".to_string(),
                        variacoes: None,
                        resistencias: None,
                        vulnerabilidades: None,
                        imunidades: None,
                        recarga: None,
                        proficiencias: None,
                        qtd_proficiencias: None,
                    },
                    HabilidadeEspecial{
                        nome:"Corpo Frágil".to_string(),
                        descricao:"Um Floral devido a sua natureza frágil possui outras
                        maneiras de se defender. Escolha uma no 1° nível e novamente no 4°.".to_string(),
                        variacoes:Some(vec![
                            Variacao{
                                nome:"Charme Sobrenatural".to_string(),
                                descricao:"Você pode utilizar da sua beleza para distrair
                                inimigos ou encantar vendedores para conseguir descontos. Criaturas
                                que você encanta devem fazer uma salvaguarda de Carisma ou, durante
                                1 hora, estarão sob a condição Enfeitiçado.".to_string(),
                                recarga:None,
                                vantagens:None
                            },
                            Variacao{
                                nome: "Espinhos Alucinógenos".to_string(),
                                descricao: "Inimigos que te atacarem desarmados devem
                                passar em uma salvaguarda de Constituição, em uma falha o atacante
                                perde a noção de realidade e imaginário, criaturas alucinógenas avançam
                                em direção a ele. No final de cada turno ele pode repetir a salvaguarda.
                                Adicionalmente como uma ação você pode atirar um espinho a até 15 pés
                                de distância. Você deve fazer um ataque à distância e é considerado que
                                você possui proficiência com seus espinhos.".to_string(),
                                recarga: None,
                                vantagens: None
                            },
                            Variacao{
                                nome: "Esporos Venenosos".to_string(),
                                descricao: "Ao ser atacado, você pode utilizar sua reação para
                                lançar um gás venenoso em seu inimigo. Criaturas adjacentes devem
                                passar em uma salvaguarda de Constituição, em uma falha as criaturas a
                                sua volta recebem a condição Envenenado e 1d4 de dano venenoso por
                                turno, no fim de cada turno da criatura ela pode repetir a salvaguarda
                                para anular a condição.".to_string(),
                                recarga: None,
                                vantagens: None
                            },
                            Variacao{
                                nome: "Flor do Imperador".to_string(),
                                descricao: "Nega os efeitos de “Vegetação”, mas efeitos ígneos
                                prolongados duram o dobro.".to_string(),
                                recarga: None,
                                vantagens: None,
                            },
                            Variacao{
                                nome: "Flor Gélida".to_string(),
                                descricao: "No primeiro nível você pode utilizar um ponto de Herança
                                para escolher a Herança Variante Glacinata. Adicionalmente, inimigos
                                que te atacarem desarmados devem passar em uma salvaguarda de
                                Constituição, em uma falha o atacante perde 15 pés de deslocamento.
                                No final de cada turno, ele pode repetir a salvaguarda. Esse efeito não é
                                auto-cumulativo.".to_string(),
                                recarga: None,
                                vantagens: None
                            },
                            Variacao{
                                nome: "Luminescência".to_string(),
                                descricao: "Você conhece os truque luz e luzes dançantes.".to_string(),
                                recarga: None,
                                vantagens: None
                            },
                            Variacao{
                                nome: "Nictinastia".to_string(),
                                descricao: "Quando você chegar a 0 pontos de vida você se fecha em
                                suas pétalas, voltando com 1 ponto de vida mas em um estado dormente.
                                Você fica nesse estado de dormência até alguém destruir suas pétalas ou
                                até você recuperar toda sua vida. Enquanto estiver dormente você não
                                poderá se mover ou realizar nenhum tipo de ação ou reação. No começo
                                de cada turno você recupera 1d4 + seu modificador de Constituição de
                                vida até estar com vida completa ou alguém destruir suas pétalas. Essas
                                pétalas possuem 10 de CA e uma vida igual ao seu bônus de proficiência +
                                seu modificador de Constituição. Essa habilidade se ativa sozinha apenas
                                uma vez por descanso longo.".to_string(),
                                recarga: None,
                                vantagens: None
                            },
                            Variacao{
                                nome: "Raízes Detectoras".to_string(),
                                descricao: "Você pode se enraizar em um local que tenha terra,
                                areia ou lama se concentrando por 10 minutos. Ao fazer isso você pode
                                detectar uma quantidade de criaturas que esteja a até 1 milha de você, mas
                                não pode dizer a espécie das criaturas. Você também sabe dizer se existe
                                alguma região que está sendo afetada por magia dentro desse mesmo
                                alcance. Essa habilidade só possui efeito se as criaturas ou magias estejam
                                em terra, lama ou areia, ou seja, você não pode detectar criaturas que
                                estejam dentro ou acima de algo feito de pedra ou gelo.".to_string(),
                                recarga: None,
                                vantagens: None
                            },
                            Variacao{
                                nome: "Sopro de Esporos".to_string(),
                                descricao: "Você pode, uma vez por descanso longo, soprar seu
                                estoque de pólen como uma arma ofensiva em formato de cone de 15
                                pés. Criaturas nessa área devem fazer uma salvaguarda de Constituição
                                ou ficam cegas por um número de turnos igual ao seu modificador de
                                Constituição. A criatura pode refazer a salvaguarda a cada novo turno.".to_string(),
                                recarga: Some("Descanso longo".to_string()),
                                vantagens: None
                            },
                            Variacao{
                                nome: "Sopro Polinizador".to_string(),
                                descricao: "Você pode uma vez por descanso longo soprar seu
                                estoque de pólen como carga de energia curativa, em formato de cone de
                                15 pés, criaturas aliadas nessa área são curadas em 1d4 vezes o seu bônus
                                de Proficiência + o seu modificador de constituição.".to_string(),
                                recarga: Some("Descanso longo".to_string()),
                                vantagens: None
                            }
                            ]),
                        resistencias: None,
                        vulnerabilidades: None,
                        imunidades: None,
                        recarga: None, 
                        proficiencias: None,
                        qtd_proficiencias: None,
                    },
                    HabilidadeEspecial{
                        nome: "Mente Única".to_string(),
                        descricao: "Florais têm vantagem em salvaguardas contra magias que
                        lhe dariam a condição Enfeitiçado.".to_string(),
                        variacoes: Some(vec![
                            Variacao{
                                nome: "Mente Única".to_string(),
                                descricao: "Florais têm vantagem em salvaguardas contra magias que
                                lhe dariam a condição Enfeitiçado.".to_string(),
                                recarga: None,
                                vantagens: Some(vec![Vantagem::Enfeiticado])
                            }
                        ]),
                        resistencias: None,
                        vulnerabilidades: None,
                        imunidades: None,
                        recarga: None,
                        proficiencias: None,
                        qtd_proficiencias: None,
                    }
                ]),
                bonus_constituicao: Some(2),
                bonus_sabedoria: Some(2),
                altera_calculo_ca: true,
                possui_cd_propria: true
            }
            ]
        ),

        quantidade_herancas: None,
        quantidade_idiomas: 1,
        tamanho_minimo: 1.5,
        tamanho_maximo: 2.9,
        tipo_tamanho: TipoTamanho::Medio,
        pericias: None,
        qtd_aprimoramentos: None,
        oficios: None,
        qtd_oficios: None,
        descricoes_origem: DescricoesOrigem{
            heranca: "Pode receber herança da linhagem escolhida.".to_string(),
            idade: "A vida dos Antroplantæ é algo estranho, enquanto arbóreos
alcançam a maturidade entre 8 e 15 anos, Florais podem alcançar a
maturidade em 1 ou 2 anos. No entanto, podem vir a viver por milênios
dependendo da sua vida.".to_string(),
            tendencia: "Um Antroplantæ Floral adulto tende a ter um tamanho médio
entre 1,5 a 2,1 metros, enquanto um Arbóreo pode variar de 2,1 a 2,9
metros. Para todos os efeitos seu tamanho é Médio.".to_string(),
            tamanho: "Um Antroplantæ Floral adulto tende a ter um tamanho médio
entre 1,5 a 2,1 metros, enquanto um Arbóreo pode variar de 2,1 a 2,9
metros. Para todos os efeitos seu tamanho é Médio.".to_string(),
            deslocamento: "Seu deslocamento base de caminhada é 30 pés.".to_string(),
            idiomas: "Antroplantæ podem falar Silvestre e um idioma comum
adicional à sua escolha, normalmente relacionado a região mais próxima
da qual você cresceu. Eles podem se comunicar com qualquer árvore de
seu próprio tipo como se usassem a magia falar com plantas.".to_string(),
            proficiencia: None,
            regiao: "Cada região de Runeterra possui uma cultura diferente. Você
deve escolher sua Região que definirá onde você cresceu e muito de
como você vê o mundo. Você pode ver mais a respeito disso no Capítulo
11: Runeterra.".to_string(),
            pericia: None,
            aprimoramento: None,
            oficio: None,
        },
    };

    return antroplantae
}

fn info_construto() -> AtributosOrigem{
    let construto = 
    AtributosOrigem{
        nome: "Construto".to_string(),
        resumo: "Os construtos são extremamente raros em Runeterra, em uma terra
        abastecida de magia e com a existência dos cristais Hextec, fabricados
        em Piltover (as vezes até mesmo de energia quimtec, uma tecnologia
        zaunita fabricada dos resíduos da produção hextec), a criação de vida
        não é um poder exclusivo das divindades, mas sim algo que pode ser
        feito por pessoas inescrupulosas ou mesmo desesperadas.".to_string(),
        idade_maxima: 1000000,
        tamanho: "Médio".to_string(),
        deslocamento: 30.0,
        idiomas: vec!["Zaunita".to_string()],
        regiao: TipoRegiao::EscolhaLivre,
        habilidades_especiais: Some(vec![
        HabilidadeEspecial{
            nome: "Peculiaridades".to_string(),
            descricao: "Seja por algum aspecto específico da sua criação ou alguma relação com
            sua fonte, construtos normalmente tem uma ou duas peculiaridades em
            sua criação. Você pode escolher entre fazer uma jogada ou escolher
            entre uma das peculiaridades abaixo.".to_string(),
            resistencias: None,
            vulnerabilidades: None,
            imunidades: None,
            recarga: None,
            proficiencias: None,
            qtd_proficiencias: None,
            variacoes: Some(vec![
            Variacao{
                nome: "Peculiaridade 1".to_string(),
                descricao: "Você analisa (em voz alta) o potencial de ameaça de cada criatura que você
                conheça.".to_string(),
                recarga: None,
                vantagens: None,
            },
            Variacao{
                nome: "Peculiaridade 2".to_string(),
                descricao: "Você não entende emoções e frequentemente interpreta mal situações
                emocionais.".to_string(),
                recarga: None,
                vantagens: None
            },
            Variacao{
                nome: "Peculiaridade 3".to_string(),
                descricao: "Você é altamente protetor com todos aqueles que você considera amigos.".to_string(),
                recarga: None,
                vantagens: None
            },
            Variacao{
                nome: "Peculiaridade 4".to_string(),
                descricao: "Você frequentemente diz coisas que está pensando em voz alta sem ao menos
                perceber.".to_string(),
                recarga: None,
                vantagens: None
            },
            Variacao{
                nome: "Peculiaridade 5".to_string(),
                descricao: "Você tenta aplicar táticas de guerra e disciplina para todas as situações.".to_string(),
                recarga: None,
                vantagens: None
            },
            Variacao{
                nome: "Peculiaridade 6".to_string(),
                descricao: "Você não sabe como filtrar seus sentimentos e está sempre prestes a explodir
                suas emoções dramaticamente.".to_string(),
                recarga: None,
                vantagens: None
            },
            Variacao{
                nome: "Peculiaridade 7".to_string(),
                descricao: "Você não entende as roupas além de sua utilidade e assume que o que uma
                criatura veste indica seu trabalho e status.".to_string(),
                recarga: None,
                vantagens: None
            },
            Variacao{
                nome: "Peculiaridade 8".to_string(),
                descricao: "Você é obcecado com a sua aparência e lustra constantemente sua armadura.".to_string(),
                recarga: None,
                vantagens: None
            },
            Variacao{
                nome: "Peculiaridade 9".to_string(),
                descricao: "Você está profundamente preocupado em seguir os procedimentos e protocolos adequados.".to_string(),
                recarga: None,
                vantagens: None
            },
            Variacao{
                nome: "Peculiaridade 10".to_string(),
                descricao: "A guerra é a única coisa que faz sentido para você. E você está sempre procurando uma luta.".to_string(),
                vantagens: None,
                recarga: None
            }
            ])
        },
        HabilidadeEspecial{
            nome: "Construto Vivo".to_string(),
            descricao: "Construtos foram feitos para durar, por isso recebem as
            seguintes características.".to_string(),
            resistencias: Some(vec![Resistencia::Veneno]),
            recarga: None,
            vulnerabilidades: None,
            proficiencias: None,
            qtd_proficiencias: None,
            imunidades: Some(vec![Imunidade::Doenca, Imunidade::Dormir, Imunidade::Comer, Imunidade::Respirar, Imunidade::Sono]),
            variacoes: Some(vec![
            Variacao{
                nome: "Característica 1".to_string(),
                descricao: "Vantagem em salvaguardas contra venenos e resistência a dano de venenos.".to_string(),
                recarga: None,
                vantagens: Some(vec![Vantagem::Veneno,])
            },
            Variacao{
                nome: "Característica 2".to_string(),
                descricao: "Imunidade a doenças.".to_string(),
                recarga: None,
                vantagens: None
            },
            Variacao{
                nome: "Característica 3".to_string(),
                descricao: "Não precisa comer, beber ou respirar.".to_string(),
                recarga: None,
                vantagens: None
            },
            Variacao{
                nome: "Característica 4".to_string(),
                descricao: "Não precisa dormir, não sofre efeito de exaustão por falta de descanso e
                não sofre efeitos de magia de sono".to_string(),
                recarga: None,
                vantagens: None
            },
            Variacao{
                nome: "Característica 5".to_string(),
                descricao: "Cura apenas metade da vida com magias e efeitos de cura. A magia
                reparar dano cura normalmente.".to_string(),
                recarga: None,
                vantagens: None
            },
            Variacao{
                nome: "Característica 6".to_string(),
                descricao: "Desvantagem em salvaguardas por climas hostis, como frio ou desértico.".to_string(),
                recarga: None,
                vantagens: None
            }
            ])
        },
        HabilidadeEspecial{
            nome: "Descanso do Vigia".to_string(),
            descricao: "Quando você faz um descanso longo você não
            recupera pontos de vida como outras origens normalmente fazem, ao
            invés disso você precisa se consertar ou conseguir alguém que o faça.
            Para tal feito é necessário 6 horas de trabalho cuidadoso.".to_string(),
            recarga: None,
            resistencias: None,
            variacoes: None,
            vulnerabilidades: None,
            imunidades: None,
            proficiencias: None,
            qtd_proficiencias: None,
        },
        HabilidadeEspecial{
            nome: "Proteção Ajustável".to_string(),
            descricao: "Seu corpo possui camadas de defesa que definem
            sua Classe de Armadura. Você não tem benefícios de usar armadura,
            mas pode somar a CA de um escudo normalmente. Cada modelo possui
            suas próprias limitações. Você pode durante um descanso longo ajustar
            o seu corpo para modos diferentes de armadura. Caso você possua a
            característica de classe Defesa sem Armadura, você sempre terá o modo
            Nulo.".to_string(),
            recarga: None,
            resistencias: None,
            vulnerabilidades: None,
            imunidades: None,
            proficiencias: None,
            qtd_proficiencias: None,
            variacoes: Some(vec![
            Variacao{
                nome: "Armadura Nula".to_string(),
                descricao: "Requisito: Nenhum | Efeito: +1 de CA | Modelo: Todos".to_string(),
                recarga: None,
                vantagens: None
            },
            Variacao{
                nome: "Armadura Leve".to_string(),
                descricao: "Requisito: Nenhum | Efeito: CA = 13 + Mod. de Destreza | Modelo: Todos".to_string(),
                recarga: None,
                vantagens: None
            },
            Variacao{
                nome: "Armadura Média".to_string(),
                descricao: "Requisito: Proficiência com
                armaduras médias | Efeito: CA = 16 + Mod. de Destreza (Máx. de 2) | Modelo: Todos".to_string(),
                recarga: None,
                vantagens: None
            },
            Variacao{
                nome: "Armadura Pesada".to_string(),
                descricao: "Requisito: Proficiência com
                armaduras pesadas | Efeito: CA = 19 | Modelo: Brutal e Emissário".to_string(),
                recarga: None,
                vantagens: None
            },
        ])
        }
        ]),
        linhagens: Some(vec![
        Linhagem{
            nome: "Ligeiro".to_string(),
            descricao: "Os construtos de modelo ligeiro são baseados na estrutura humana,
            mas são normalmente mais finos e delicados, construídos com intuito
            de leveza e delicadeza, normalmente para funções que exigem maior
            precisão manual ou até mesmo como duelistas.".to_string(),
            heranca: Some("Seu valor de Destreza aumenta em +1, e você possui 2 pontos
            de Herança, veja o Capítulo 5: Personalização para a lista de Heranças
            disponíveis.".to_string()),
            quantidade_herancas: Some(1),
            deslocamento: 40.0,
            bonus_constituicao: None,
            bonus_sabedoria: None,
            altera_calculo_ca: false,
            possui_cd_propria: false,
            habilidades_especiais: Some(vec![
            HabilidadeEspecial{
                nome: "Energia Cinética".to_string(),
                descricao: "Você pode utilizar sua ação para realizar uma ação
                de Disparada. Se mover dessa maneira não lhe causará ataques de
                oportunidade. Adicionalmente você pode energizar um ataque
                desarmado ou com arma corpo a corpo (a arma precisa ser de metal)
                enquanto se move. Utilizando uma ação bônus você pode realizar esse
                ataque energizado, ele causará um dano adicional de 1 para cada 5
                pés que você se moveu enquanto usava Disparada. Você pode utilizar
                essa habilidade um número de vezes igual a metade de seu bônus de
                proficiência (arredondado para baixo) por descanso longo.".to_string(),
                variacoes: None,
                recarga: Some("Metade do bônus de proficiência (Arredondado para baixo) por descanso longo.".to_string()),
                resistencias: None,
                vulnerabilidades: None,
                imunidades: None,
                proficiencias: None,
                qtd_proficiencias: None,
            },
            HabilidadeEspecial{
                nome: "Duelista".to_string(),
                descricao: "Quando você estiver empunhando uma arma de acuidade com
                a qual você seja proficiente e outra criatura atingir você com um ataque
                corpo a corpo, você pode usar sua reação para fazer um ataque com
                vantagem no atacante.".to_string(),
                variacoes: None,
                recarga: None,
                resistencias: None,
                vulnerabilidades: None,
                imunidades: None,
                proficiencias: None,
                qtd_proficiencias: None,
            },
            HabilidadeEspecial{
                nome: "Hábil".to_string(),
                descricao: "Você possui proficiência em um ofício à sua escolha.".to_string(),
                variacoes: None,
                recarga: None,
                resistencias: None,
                vulnerabilidades: None,
                imunidades: None,
                proficiencias: Some("Ofício".to_string()),
                qtd_proficiencias: Some(1),
            }
            ])
        },
        Linhagem{
            nome: "Brutal".to_string(),
            descricao: "Como o próprio nome diz, construtos brutais são feitos seguindo
            modelos mais robustos e com funções que requerem maior força.
            Geralmente são demolidores, leões de chácara, construtores civis, entre
            outras funções.".to_string(),
            heranca: Some("Seu valor de Força aumenta em +1, e você possui 2 pontos
            de Herança, veja o Capítulo 5: Personalização para a lista de Heranças
            disponíveis.".to_string()),
            quantidade_herancas: Some(2),
            deslocamento: 30.,
            habilidades_especiais: Some(vec![
            HabilidadeEspecial{
                nome: "Ultrapassar os Limites".to_string(),
                descricao: "Utilizando sua energia interna, você pode
                aumentar a velocidade de seus pistões de movimento temporariamente,
                aumentando sua velocidade mas sobreaquecendo seu circuito. Com
                uma ação você aumenta sua velocidade pelos próximos 3 turnos e seu
                deslocamento base se torna 40 pés, mas no final do 3o turno
                você fica superaquecido, reduzindo seu deslocamento base
                para 15 durante 2 turnos. Você pode realizar essa ação
                uma vez por descanso curto ou longo.".to_string(),
                variacoes: None,
                recarga: Some("Descanso curto ou longo".to_string()),
                resistencias: None,
                vulnerabilidades: None,
                imunidades: None,
                qtd_proficiencias: None,
                proficiencias: None
            },
            HabilidadeEspecial{
                nome: "Força Pneumática".to_string(),
                descricao: "Você é capaz de redirecionar
                internamente sua energia para intensificar o
                poder de seus ataques. Você pode fazer isso um
                total de vezes igual a metade do seu bônus de proficiência. Durante esse turno você não pode se movimentar e
                seu modificador de Força para o primeiro ataque feito nesse turno é
                dobrado, exceto para ataques à distância. No nível 7 sua movimentação
                fica apenas reduzida pela metade. No nível 10, seu modificador de
                Força é dobrado para todos seus ataques nesse turno. No nível 14 esse
                modificador de dano é triplicado e você pode adicionar seu bônus de
                proficiência.".to_string(),
                variacoes: None,
                recarga: Some("Metade do bônus de proficiência".to_string()),
                resistencias: None,
                vulnerabilidades: None,
                imunidades: None,
                qtd_proficiencias: None,
                proficiencias: None
            },
            HabilidadeEspecial{
                nome: "Resistente".to_string(),
                descricao: "Você possui resistência a um tipo de dano a seguir, escolha
                ele durante a sua criação: ácido, elétrico, energético, ígneo, gélido,
                necrótico, psíquico ou radiante.".to_string(),
                variacoes: None,
                recarga: None,
                resistencias: None,
                vulnerabilidades: None,
                imunidades: None,
                qtd_proficiencias: None,
                proficiencias: None
            }
            ]),
            bonus_constituicao: None,
            bonus_sabedoria: None,
            altera_calculo_ca: false,
            possui_cd_propria: false
        },
        Linhagem{
            nome: "Emissário".to_string(),
            descricao: "Os emissários também são bem similares aos humanos, seja em forma
            ou tamanho. Geralmente são responsáveis por entregar mensagens a
            longas distâncias, por causa dessa funcionalidade eles possuem proteção
            contra a maior parte dos ambientes, tendo a certeza de que a mensagem
            será entregue.".to_string(),
            heranca: Some("Seu valor de Constituição aumenta em +1, e você possui 2
            pontos de Herança, veja o Capítulo 5: Personalização para a lista de
            Heranças disponíveis.".to_string()),
            quantidade_herancas: Some(2),
            deslocamento: 30.0,
            habilidades_especiais: Some(vec![
            HabilidadeEspecial{
                nome: "Geolocalização".to_string(),
                descricao: "Seu modelo é equipado com um sistema de percepção
                secundário, o Sentido Sísmico. Com ele você pode detectar e localizar
                a origem das vibrações dentro de 50 pés, desde que você e a origem
                das vibrações estejam em contato com o mesmo chão ou a mesma
                substância. Esse sentido não funciona com criaturas incorpóreas ou
                em voo. Além disso, esse sistema dá a você vantagem em testes para
                perceber ilusões, resistência a dano psíquico e seu alcance de visão é
                dobrado. Em combate o alcance do sentido sísmico reduz para 15 pés,
                mas você pode abdicar de sua movimentação para ampliar o sentido
                para o alcance máximo. Você possui vantagem em salvaguardas para
                climas hostis, como o calor do deserto ou o clima gélido de Freljord.
                Adicionalmente, você ignora terrenos difíceis feitos de lama, gelo ou
                areia.".to_string(),
                variacoes: None,
                resistencias: Some(vec![Resistencia::DanoPsiquico]),
                vulnerabilidades: None,
                imunidades: None,
                recarga: None,
                qtd_proficiencias: None,
                proficiencias: None,
            },
            HabilidadeEspecial{
                nome: "Aumentar a Voltagem".to_string(),
                descricao: "Você pode adicionar 1d4 de dano elétrico a um de
                seus ataques desarmados ou com arma corpo a corpo por turno, desde
                que elas sejam feitas de metal. Esse bônus aumenta para 1d8 no nível 6,
                1d10 no nível 12 e 2d6 no nível 18.".to_string(),
                variacoes: None,
                recarga: None,
                resistencias: None,
                vulnerabilidades: None,
                imunidades: None,
                qtd_proficiencias: None,
                proficiencias: None
            },
            HabilidadeEspecial{
                nome: "Funcional".to_string(),
                descricao: "Você possui proficiência em uma das seguintes perícias:
                Acrobacia, Arcanismo, Atletismo, História, Intuição, Medicina,
                Natureza, Percepção, Prestidigitação, Religião, Sobrevivência ou
                Tecnologia.".to_string(),
                variacoes: None,
                recarga: None,
                resistencias: None,
                vulnerabilidades: None,
                imunidades: None,
                qtd_proficiencias: None,
                proficiencias: None
            }
            ]),
            bonus_constituicao: None,
            bonus_sabedoria: None,
            altera_calculo_ca: false,
            possui_cd_propria: false
        }
        ]),
        quantidade_herancas: None,
        quantidade_idiomas: 1,
        tamanho_minimo: 1.5,
        tamanho_maximo: 2.9,
        tipo_tamanho: TipoTamanho::Medio,
        pericias: None,
        qtd_aprimoramentos: None,
        oficios: None,
        qtd_oficios: None,
        descricoes_origem:
        DescricoesOrigem{
            heranca: "Pode adquirir a herança a partir da Linhagem.".to_string(),
            idade: "Os construtos são extremamente raros em Runeterra, em uma terra
            abastecida de magia e com a existência dos cristais Hextec, fabricados
            em Piltover (as vezes até mesmo de energia quimtec, uma tecnologia
            zaunita fabricada dos resíduos da produção hextec), a criação de vida
            não é um poder exclusivo das divindades, mas sim algo que pode ser
            feito por pessoas inescrupulosas ou mesmo desesperadas.".to_string(),
            tendencia: "Quase todos os Construtos são ordeiros, talvez por
            desenvolverem consciência enquanto máquinas comandadas
            ou talvez sejam resquícios de memórias que permanecem
            em seus cristais Hextec.".to_string(),
            tamanho: "A maior parte dos construtos, sejam eles com feições
            masculinas ou femininas tendem a ser um pouco maiores que os
            humanos médios, possuindo entre 1,5 a 2 metros de altura. Com exceção
            do modelo Brutal que pode atingir até 2,9 metros de altura. Para todos
            os efeitos seu tamanho é Médio.".to_string(),
            deslocamento: "Pode adquirir deslocamento a partir da Linhagem.".to_string(),
            idiomas: "A maior parte dos construtos, sejam eles com feições
            masculinas ou femininas tendem a ser um pouco maiores que os
            humanos médios, possuindo entre 1,5 a 2 metros de altura. Com exceção
            do modelo Brutal que pode atingir até 2,9 metros de altura. Para todos
            os efeitos seu tamanho é Médio.".to_string(),
            proficiencia: None,
            regiao: "Cada região de Runeterra possui uma cultura diferente. Você
            deve escolher sua Região que definirá onde você cresceu e muito de
            como você vê o mundo. Você pode ver mais a respeito disso no Capítulo
            11: Runeterra.".to_string(),
            pericia: None,
            aprimoramento: None,
            oficio: Some("Pode adquirir ofício a partir da Linhagem.".to_string()),
        },
    };
    return construto;
}

pub fn info_origens() -> Vec<AtributosOrigem> {
    let mut origens = vec![];
    origens.push(info_humanos());
    origens.push(info_antroplantae());
    origens.push(info_construto());

    return origens;
}

#[get("/")]
pub async fn lista_origens() -> Json<Vec<AtributosOrigem>>{
    let origens = info_origens();
    return Json(origens);
}