use rocket::{get, post, route, serde::json::Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AtributosClasse{
    nome: String,
    dado_vida: String,
    pontos_vida: f32,
    modificador_pontos_vida: String,
    pontos_vida_nivel: f32,
    dados_vida_nivel: String,

    proficiencias_armaduras: Option<Vec<String>>,
    proficiencias_armas: Option<Vec<String>>,
    proficiencias_oficios: Option<Vec<String>>,
    proficiencias_salvaguardas: Option<Vec<String>>,
    proficiencias_pericias: Option<Vec<String>>,

    qtd_proficiencias_pericias: Option<f32>,
    opcoes_pericias: Option<Vec<String>>,

    equipamento_base: Option<Vec<EquipamentoBase>>,

    informacoes_conjuracao: Option<InformacoesConjuracao>,
    lista_magias: Option<Vec<InformacoesMagia>>,

    lista_subclasses: Option<Vec<Subclasse>>,
    titulo_subclasse: Option<String>,
    descricao_subclasse: Option<String>,

    habilidades: Option<Vec<HabilidadeClasse>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InformacoesConjuracao{
    qtd_truques: String,
    lista_magia: String,
    qtd_magias_inicial: u32,

    habilidade_conjuracao: String,

    info_truques: String,
    info_espacos_magia: String,
    info_magias_conhecidas: String,
    info_habilidade_conjuracao: String,

    cd_magia: String,
    ataque_magia: String,

    conjuracao_ritual: Option<String>,
    foco_conjuracao: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InformacoesMagia{
    nome: String,
    descricao: String,
    componente_sinal: Option<String>,
    componente_verbal: Option<String>,
    componente_materia: Option<String>,
    
    alcance: Option<String>,
    duracao: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EquipamentoBase{
    opcoes: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]

pub struct HabilidadeClasse{
    nome: String,
    descricao: String,

    caracteristicas: Option<Vec<CaracteristicaHabilidadeClasse>>,
    variacoes: Option<Vec<VariacaoHabilidadeClasse>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CaracteristicaHabilidadeClasse{
    nome: String,
    descricao: String,
    caracteristicas: Option<Vec<CaracteristicaHabilidadeClasse>>,
    variacoes: Option<Vec<VariacaoHabilidadeClasse>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VariacaoHabilidadeClasse{
    nome: String,
    descricao: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subclasse{
    nome: String,
    descricao: String,

    proficiencias_armaduras: Option<Vec<String>>,
    proficiencias_armas: Option<Vec<String>>,
    proficiencias_oficios: Option<Vec<String>>,
    proficiencias_salvaguardas: Option<Vec<String>>,
    proficiencias_pericias: Option<Vec<String>>,

    informacoes_conjuracao: Option<InformacoesConjuracao>,
    
    habilidades: Option<Vec<HabilidadeClasse>>,
    variacoes: Option<Vec<VariacaoHabilidadeClasse>>
}

pub fn info_mercurial() -> AtributosClasse{

    let subclasse_4 = Subclasse{
        nome: "Trapaceiro Rúnico".to_string(),
        descricao: "Alguns mercuriais aprimoram suas finas perícias de furtividade e
        agilidade com magia, aprendendo truques de encantamento e ilusão.
        Esses ladinos incluem não somente batedores de carteira e assaltantes,
        mas também trapaceiros, enganadores e um número significativo de
        aventureiros.".to_string(),
        proficiencias_armaduras: None,
        proficiencias_armas: None,
        proficiencias_oficios: None,
        proficiencias_salvaguardas: None,
        proficiencias_pericias: None,
        informacoes_conjuracao: Some(
        InformacoesConjuracao{
            qtd_truques: "Ao escolher essa subclasse, você adquire a habilidade de conjurar
            magias. Veja o Capítulo 10 para as regras gerais de conjuração e para a
            lista de magias de Mercurial.".to_string(),
            lista_magia: "Mercurial".to_string(),
            habilidade_conjuracao: "Carisma".to_string(),
            cd_magia: "8 + seu bônus de proficiência + Seu modificador de Carisma".to_string(),
            ataque_magia: "seu bônus de proficiência + seu modificador de Carisma".to_string(),
            conjuracao_ritual: None,
            foco_conjuracao: None,
            info_espacos_magia: "A tabela Mercurial Conjurador mostra quantos Espaços de Magia
            você tem para lançar suas magias de 1o nível ou maior. Para lançar
            uma dessas magias, você deve gastar um espaço no nível da magia
            ou superior. Você recupera todas os espaços gastos quando termina
            um longo descanso.
            Por exemplo, se você conhece a magia enfeitiçar pessoa de 1o nível
            e tem um espaço de magia de 1o e 2o nível disponível, você pode
            conjurar enfeitiçar pessoa usando um dos espaços.".to_string(),
            info_habilidade_conjuracao: "Carisma é a sua habilidade de lançar magias para suas magias de
            Trapaceiro Rúnico, uma vez que o poder de sua magia depende do
            roubo de conhecimento daqueles ao seu redor.
            Você usa seu Carisma sempre que uma magia se refere à sua
            habilidade de lançar feitiços. Além disso, você usa seu modificador
            de Carisma para definir a CD da salvaguarda para as magias de
            Mercurial que você conjura e quando você realiza uma jogada de
            ataque com uma magia.".to_string(),
            info_magias_conhecidas: "
            No 3o nível você conhece três magias de Mercurial de 1o nível de sua
            escolha. A coluna magias conhecidas mostra quando você aprende
            mais magias do 1o nível ou superior. Cada uma dessas magias
            deve estar em um nível para o qual você tem espaços de magia.
            Por exemplo, quando você atingir o 8o nível nesta classe, poderá
            aprender uma nova magia de 1o ou 2o nível.
            Sempre que você ganhar um nível nesta classe, poderá substituir uma
            das magias de Mercurial que você conhece por outra de sua escolha
            na lista de magias do Mercurial. A nova magia deve ser de um nível
            para o qual você tenha espaços de magia.".to_string(),
            info_truques: "Você aprende três truques: mãos mágicas e outros dois truques, à sua
            escolha, da lista de magias de Mercurial. Você aprende um truque de
            Mercurial adicional, à sua escolha, no 10o nível.".to_string(),
            qtd_magias_inicial: 0
        }
        ),
        habilidades: None, // IMPLEMENTAR
        variacoes: None, // IMPLEMENTAR
    };

    let subclasse_3 = Subclasse{
        nome: "Explorador".to_string(),
        descricao: "Desbravar catacumbas soterradas, envolver-se com antigas maldições
        e superar com facilidade as situações mais improváveis. Sua coragem e
        ousadia não têm limites e você prefere resolver as coisas no improviso,
        confiando parcialmente no seu talento, mas muito mais em sua sorte e
        conhecimento. Exploradores normalmente se interessam pelas coisas
        mais estranhas e aleatórias, como cartografia avançada, mecânica
        hextec, histórias antigas de Runeterra entre outros conhecimentos.
        Os Exploradores normalmente atuam como caçadores de tesouro
        profissionais, exploradores de masmorras e investigadores. Além de
        aprimorar sua agilidade e furtividade, você aprende perícias úteis para
        desbravar ruínas antigas, ler idiomas incomuns e usar itens mágicos que
        normalmente não poderia.".to_string(),
        proficiencias_armaduras: None,
        proficiencias_armas: None,
        proficiencias_oficios: None,
        proficiencias_salvaguardas: None,
        proficiencias_pericias: None,
        informacoes_conjuracao: None,
        habilidades: Some(vec![
        HabilidadeClasse{
            nome: "Sem Barreiras".to_string(),
            descricao: "Ao seguir as filosofias dessa cabala você aprende dois idiomas
            adicionais à sua escolha da lista de idiomas comuns no
            Capítulo 4: Detalhamento - Idiomas. Além disso, escolha
            mais 4 idiomas, você não é capaz de falar, escrever ou ler
            neles, mas é capaz de identificá-los seja ouvindo ou lendo.
            Ao atingir os níveis 7o, 13o e 19o você pode aprender um
            idioma à sua escolha (você pode falar, ler e escrever nele).
            Você adquire a habilidade de escalar mais rápido que o
            normal. Escalar não possui custo adicional de movimento
            para você. Além disso, quando você fizer um salto com
            corrida, o alcance que pode saltar aumenta um número
            de pés igual ao seu modificador de Destreza.".to_string(),
            caracteristicas: None,
            variacoes: None
        },
        HabilidadeClasse{
            nome: "Conhecimento Avançado".to_string(),
            descricao: "A partir do 3o nível, você consegue sintonizar sua
            intuição e sua percepção de forma extraordinária,
            recordando-se de plantas de construções,
            diagramas de engrenagens e etc. Você pode usar
            a sua ação bônus concedida pela Ação Ardilosa
            para fazer um teste de Prestidigitação, usar suas
            ferramentas de chaveiro para desarmar uma
            armadilha ou abrir uma fechadura, ou realizar a ação
            de Usar um Objeto. Além disso você recebe proficiência
            em Prestidigitação, caso já possua proficiência você
            ganha Especialista nessa perícia, adicionalmente você
            recebe proficiência com um ofício adicional à sua escolha.
            Graças a esse conhecimento e percepção, apoiado pela
            sua intuição que o sintoniza quanto à existência de armadilhas, você pode localizar armadilhas a até 120 pés de de você.
            Uma vez que você tenha usado essa habilidade, você deve realizar um
            descanso longo antes de usá-la novamente. Caso algum aliado ative
            ou desarme alguma armadilha que você não tenha localizado, você
            recarrega o uso dessa habilidade. No 6o nível, seu alcance com essa
            habilidade se amplia para 300 pés e você sabe a direção precisa de
            qualquer armadilha detectada por essa habilidade.".to_string(),
            caracteristicas: None,
            variacoes: None
        },
        HabilidadeClasse{
            nome: "Esquiva Sobrenatural".to_string(),
            descricao: "A partir do 5o nível, quando um inimigo que você possa ver o acerta
            com um ataque, você pode usar sua reação para reduzir pela metade o
            dano sofrido.".to_string(),
            caracteristicas: None,
            variacoes: None
        },
        HabilidadeClasse{
            nome: "Evasão".to_string(),
            descricao: "A partir do 7o nível, você pode esquivar-se agilmente de certos efeitos
            em área, como o sopro flamejante de um dragão infernal ou uma magia
            tempestade glacial. Quando você for alvo de um efeito que exija uma
            salvaguarda de Destreza para sofrer metade do dano, você não sofre
            dano algum se passar, e somente metade do dano se falhar.".to_string(),
            variacoes: None,
            caracteristicas: None
        },
        HabilidadeClasse{
            nome: "Aprendizado Rápido".to_string(),
            descricao: "A partir do 9o nível, você possui habilidades reconhecíveis para agrupar
            novas informações e se comprometer aos estudos. Você sempre sabe
            quando alguém está mentindo para você (ou simplesmente não sabe a
            resposta). Enquanto você estiver em uma região ou cidade desconhecida
            ou pouco familiar, você pode gastar 10 PO e 3 horas estudando para
            lhe garantir vantagem em testes de Arcanismo, História, Natureza ou
            Religião relacionados sobre a cidade ou região pela próxima semana.
            Você também sabe as criaturas nativas da região. Adicionalmente, você
            tem vantagem no teste de Furtividade se você não mover-se mais do que
            a metade de seu deslocamento em um turno.".to_string(),
            variacoes: None,
            caracteristicas: None
        },
        HabilidadeClasse{
            nome: "Talento Confiável".to_string(),
            descricao: "No 11o nível, você refinou suas perícias beirando à perfeição. Toda vez
            que você fizer um teste de perícia no qual possa adicionar seu bônus de
            proficiência, você trata um resultado no d20 de 9 ou menor como 10.
            Adicionalmente você pode escolher uma perícia que você tenha
            proficiência para receber os benefícios da habilidade
            Especialização.".to_string(),
            caracteristicas: None,
            variacoes: None
        },
        HabilidadeClasse{
            nome: "Explorador de Catacumbas".to_string(),
            descricao: "No 13o nível, você já foi exposto à grandes variedades
            de venenos e toxinas, criando uma resistência natural
            à eles, por conta disso você possui vantagem em
            salvaguardas de Constituição para resistir à condição
            Envenenado e Intoxicado. Além disso, você recebe
            resistência à dano venenoso. Adicionalmente,
            você aprende o suficiente sobre como a magia
            funciona e pode improvisar o uso de itens que
            nem mesmo foram destinados a você, também
            podendo utilizar a magia identificar um número
            de vezes igual ao seu modificador de Inteligência
            por descanso longo de forma não mágica sem
            utilizar quaisquer componentes. Você ignora todos
            os requisitos de classes, origem e níveis para uso de
            qualquer item mágico.".to_string(),
            variacoes: None,
            caracteristicas: None
        },
        HabilidadeClasse{
            nome: "Reflexos de Explorador".to_string(),
            descricao: "Quando atinge o 17o nível, você se torna
            adepto em fazer emboscadas e fugas rápidas
            de situações perigosas. Você pode realizar
            dois turnos durante a primeira rodada de cada
            combate. Você realiza seu primeiro turno na sua
            iniciativa e o segundo na ordem 10 da iniciativa.
            Você não pode usar essa característica quando
            está surpreso.".to_string(),
            variacoes: None,
            caracteristicas: None
        }
        ]),
        variacoes: None,
    };

    let subclasse_2 = Subclasse{
        nome: "Assassino".to_string(),
        descricao: "Você focou seu treinamento na macabra arte da morte. Aqueles que
        devotam-se a essa Cabala são diversos: assassinos de aluguel, espiões,
        caçadores de recompensa e, até mesmo, padres especialmente treinados
        em exterminar os inimigos das suas divindades. Subterfúgio, veneno
        e disfarces ajudam você a eliminar seus oponentes com eficiência
        mortífera.".to_string(),
        proficiencias_armaduras: None,
        proficiencias_armas: None,
        proficiencias_oficios: Some(vec!["Maquiador".to_string(), "Venefício".to_string()]),
        proficiencias_salvaguardas: None,
        proficiencias_pericias: None,
        informacoes_conjuracao: None,
        habilidades: Some(vec![
        HabilidadeClasse{
            nome: "Movimento do Pulso".to_string(),
            descricao: "A partir do 3o nível, você pode usar a ação bônus para lançar uma adaga.
            Além disso, se você errar o alvo, a adaga lançada retornará à sua mão.".to_string(),
            caracteristicas: None,
            variacoes: None
        },
        HabilidadeClasse{
            nome: "Esquiva Sobrenatural".to_string(),
            descricao: "A partir do 5o nível, quando um inimigo que você possa ver o acerta
            com um ataque, você pode usar sua reação para reduzir pela metade o
            dano sofrido.".to_string(),
            caracteristicas: None,
            variacoes: None
        },
        HabilidadeClasse{
            nome: "Espreitar".to_string(),
            descricao: "A partir do 7o nível, você aprendeu maneiras mais eficazes de se espreitar
            até seu alvo, você deve escolher entre uma dessas habilidades.".to_string(),
            caracteristicas: None,
            variacoes: Some(vec![
            VariacaoHabilidadeClasse{
                nome: "Invisibilidade".to_string(),
                descricao: "Usando uma ação bônus, no começo de seu
                próximo turno você recebe os efeitos da magia invisibilidade.
                Você pode fazer isso um número de vezes igual a seu modificador
                de Inteligência a cada descanso longo. Enquanto invisível, sua
                movimentação aumenta em 10 pés.".to_string(),
            },
            VariacaoHabilidadeClasse{
                nome: "Shunpo".to_string(),
                descricao: "Usando uma ação bônus, você pode se teleportar para trás
                de um alvo a até 30 pés de você ou em uma arma que você tenha
                lançado. Você pode usar essa habilidade um número de vezes igual
                a seu modificador de Inteligência a cada descanso longo. Caso você
                mate uma criatura, você pode utilizar essa habilidade novamente
                até o fim do próximo turno sem gastar um de seus usos por dia.".to_string()
            }])
        },
        HabilidadeClasse{
            nome: "Silenciosamente Mortal".to_string(),
            descricao: "No 9o nível, você treinou para quase não fazer barulho enquanto
            perseguia seus alvos, e pode se mover toda sua movimentação ou usar
            ação disparada sem penalidade furtiva. Você tem vantagem nos testes de
            Furtividade que envolvem som, e não pode ser detectado por criaturas
            que possuem sentido sísmico. Você também ganha visão no escuro até
            60 pés, se você já tem visão no escuro, dobre o alcance.".to_string(),
            caracteristicas: None,
            variacoes: None
        },
        HabilidadeClasse{
            nome: "Ataque de Investida".to_string(),
            descricao: "A partir do 11o nível, você aprende a atingir inimigos fora de seu alcance.
            Ao usar sua ação para fazer um ataque com uma arma de acuidade que
            você possui proficiência em um inimigo fora de seu alcance, você pode
            saltar 10 pés em direção ao alvo. Esse salto não é considerado parte da
            sua movimentação e ele ignora terreno difícil. Se o ataque atingir o alvo,
            você pode adicionar o dano de seu Ataque Furtivo, e independente da
            rolagem, o dano é considerado um acerto crítico.".to_string(),
            caracteristicas: None,
            variacoes: None
        },
        HabilidadeClasse{
            nome: "Fenda na Armadura".to_string(),
            descricao: "Quando você está no nível 13, você ganhou habilidade suficiente para
            saber onde atacar seu oponente para causar o maior dano. Agora você
            tem um alcance crítico de 19 ou 20.".to_string(),
            caracteristicas: None,
            variacoes: None
        },
        HabilidadeClasse{
            nome: "Espreita Fantasmagórica".to_string(),
            descricao: "No 17o nível, nenhum alvo pode se esconder da sua vista, sua habilidade
            Espreitar ganha uma melhoria conforme abaixo.".to_string(),
            caracteristicas: Some(vec![
            CaracteristicaHabilidadeClasse{
                nome: "Característica 1".to_string(),
                descricao: "Caso você tenha escolhido Invisibilidade, você ganha os efeitos da magia
                ver o invisível o tempo todo.".to_string(),
                caracteristicas: None,
                variacoes: None
            },
            CaracteristicaHabilidadeClasse{
                nome: "Característica 2".to_string(),
                descricao: "Caso você tenha escolhido Shunpo, você não tem limites de uso para essa habilidade.".to_string(),
                caracteristicas: None,
                variacoes: None
            }
            ]),
            variacoes: None
        }
        ]),
        variacoes: None,
    };

    let subclasse_1 = Subclasse{
        nome: "Ás Rúnico".to_string(),
        descricao: "Algumas essências são tocadas pelas runas desde seu nascimento,
        despertando capacidades ocultas nelas das formas mais variáveis, em
        alguns casos o que é despertado é a magia daquela criatura, em outros
        casos são poderes canalizados diretamente dessas runas.
        Não se sabe exatamente se é uma Runa em específico ou algum outro
        poder intermediário, ou mesmo uma configuração estelar que causa
        isso, mas algumas pessoas nascem favorecidas por poderes de sorte
        (e azar) em uma oscilação mística daquela existência que é conhecida
        como o Ás Rúnico.
        Você é uma manifestação da “Chance”, embora possa não saber o que
        isso significa exatamente a princípio, você percebe ao ganhar níveis
        nessa classe que é capaz de afetar as probabilidades aos seu redor.".to_string(),
        proficiencias_armaduras: None,
        proficiencias_armas: None,
        proficiencias_oficios: None,
        proficiencias_salvaguardas: None,
        proficiencias_pericias: None,
        informacoes_conjuracao: None,
        habilidades: Some(vec![
        HabilidadeClasse{
            nome: "Prodígio Rúnico".to_string(),
            descricao: "No 1° nível você é capaz de canalizar essa energia rúnica através de
            alguns feitos.".to_string(),
            variacoes: None,
            caracteristicas: Some(vec![
            CaracteristicaHabilidadeClasse{
                nome: "Munição Rúnica".to_string(),
                descricao: "Você é capaz de conjurar e disparar sua munição rúnica de assinatura,
                ela tem o formato à sua escolha, essa munição é uma simples arma
                de longo alcance (20/ 60 pés) com propriedades leves, acuidade,
                arremessável e causa 1d6 de dano cortante em um acerto.
                Você deve gastar 1 hora para conjurar essa munição, você conjura
                um total de 50 peças que são absorvidas pelos seus braços e podem
                ser disparadas através de ataques, intuitivamente você sempre sabe
                quanta munição você ainda tem disponível. Este processo pode
                ser realizado durante um descanso curto ou descanso longo e as
                munições duram até o próximo descanso. Ataques feitos a um alvo a
                menos de 10 pés de você são feitos com desvantagem.".to_string(),
                caracteristicas: None,
                variacoes: None
            },
            CaracteristicaHabilidadeClasse{
                nome: "Carga Rúnica".to_string(),
                descricao: "A Runa que favorece você lhe permite ter uma fonte de recurso:
                Você ganha 4 pontos de carga rúnica no 1o nível e depois mais 1
                ponto nos níveis 4, 7, 10, 13, 16 e 19.
                Seus pontos de Carga Rúnica são convertidos em dados para alguns
                efeitos, esses dados são d6 no 1o nível e se tornam d8 no nível 5, d10
                no nível 11 e d12 no 17.
                Você pode usar esses pontos para alguns recursos, você começa
                conhecendo três deles, selecione os três recursos iniciais da lista a
                seguir. Você pode aprender recursos adicionais conforme ganha
                níveis nesta classe, ganhando um recurso no 3o, 6o, 10o, 14o e 18o
                nível.
                Você recupera toda a carga gasta ao completar um descanso curto
                ou um descanso longo.".to_string(),
                variacoes: None,
                caracteristicas: Some(vec![
                CaracteristicaHabilidadeClasse{
                    nome: "Tríplice".to_string(),
                    descricao: "Ao disparar sua munição rúnica, ela se divide em um total de
                    três, com as duas adicionais se afastando da linha reta da primeira, em
                    cone para direções opostas. A jogada de ataque é feita apenas uma vez
                    para as três munições.".to_string(),
                    caracteristicas: None,
                    variacoes: None
                },
                CaracteristicaHabilidadeClasse{
                    nome: "Julgamento Errado".to_string(),
                    descricao: "Quando uma criatura que você puder ver fizer uma
                    rolagem de ataque, uma salvaguarda, um teste de perícia ou de atributo,
                    você pode usar sua reação e gastar 1 dado viciado para impedir sua
                    tentativa. Jogue um dado de carga rúnica e subtraia o número rolado da
                    rolagem da criatura. Você pode optar por usar esse recurso depois que
                    a criatura fizer o teste, mas antes do Mestre determinar se a rolagem em
                    questão foi bem sucedida ou falhou.".to_string(),
                    caracteristicas: None,
                    variacoes: None
                },
                CaracteristicaHabilidadeClasse{
                    nome: "Imbuir".to_string(),
                    descricao: "Quando você usa a ação de ataque você imbuí seu próximo
                    ataque, gaste um ponto de carga rúnica e adicione um dado ao seu ataque
                    e jogadas de dano no seu primeiro ataque. Dano adicional causado dessa
                    forma conta como dano energético. Também ganhe vantagem em todos os ataques neste turno.".to_string(),
                    caracteristicas: None,
                    variacoes: None
                },
                CaracteristicaHabilidadeClasse{
                    nome: "Fugaz".to_string(),
                    descricao: "Com uma ação, você pode gastar 1 carga rúnica para executar um
                    truque que ofusca os espectadores. Cada criatura hostil dentro de 10 pés
                    de você é imediatamente distraída pelo seu desempenho, e deve obter um
                    teste de Inteligência de CD 8 + o dado rolado da carga rúnica, ou eles têm
                    desvantagem em todas as rolagens que envolvam Atributos ou Perícias
                    até o próximo turno e o próximo ataque a criatura tem vantagem.".to_string(),
                    caracteristicas: None,
                    variacoes: None
                },
                CaracteristicaHabilidadeClasse{
                    nome: "Torrente".to_string(),
                    descricao: "Com uma ação, você pode gastar 1 carga rúnica para fazer um
                    ataque de sua munição contra qualquer número de criaturas a menos de
                    20 pés de você. Você deve fazer uma rolagem de ataque separada para
                    cada alvo.".to_string(),
                    caracteristicas: None,
                    variacoes: None
                },
                CaracteristicaHabilidadeClasse{
                    nome: "Meter o Pé".to_string(),
                    descricao: "Com uma ação bônus, você pode gastar 1 carga rúnica para
                    executar a ação Desengajar.".to_string(),
                    caracteristicas: None,
                    variacoes: None,
                },
                CaracteristicaHabilidadeClasse{
                    nome: "Esguio".to_string(),
                    descricao: "Como ação bônus, você pode gastar 1 carga rúnica para executar
                    a ação Esquiva.".to_string(),
                    caracteristicas: None,
                    variacoes: None
                },
                CaracteristicaHabilidadeClasse{
                    nome: "Jogada de Sorte".to_string(),
                    descricao: "Imediatamente depois que você falhar em uma jogada
                    de ataque, você pode gastar 1 carga rúnica para lançar um disparo na
                    face de seu alvo. Você deve então fazer uma salvaguarda de Destreza
                    de CD 20 - o número rolado do dado de carga rúnica. Caso seja bem
                    sucedido, você automaticamente acerta outra criatura de sua escolha que
                    você possa ver dentro do alcance de sua munição rúnica.".to_string(),
                    caracteristicas: None,
                    variacoes: None
                },
                CaracteristicaHabilidadeClasse{
                    nome: "Chamariz".to_string(),
                    descricao: "Com uma ação, você pode gastar uma carga Rúnica e
                    disparar uma munição que se multiplica, criando uma distração. Ao
                    fazer isso, você pode realizar um teste de furtividade com vantagem e
                    com seu dado de carga rúnica adicionado à rolagem.".to_string(),
                    variacoes: None,
                    caracteristicas: None
                },
                CaracteristicaHabilidadeClasse{
                    nome: "Arremesso Trapaceiro".to_string(),
                    descricao: "Se você sabe a localização de uma criatura
                    que você não pode atacar, você pode usar sua ação para gastar 1
                    carga rúnica ao fazer um teste de ataque contra a criatura. A munição
                    rúnica pode atacar as criaturas arqueando-as por cima e por baixo,
                    fazendo curvas em um ângulo de 90 graus ou saltando do ambiente
                    para alcançar ângulos incomuns. Arremesso Trapaceiro tem seu
                    alcance aumentado em um número de pés igual a 5 vezes o número
                    rolado do dado de carga rúnica.".to_string(),
                    caracteristicas: None,
                    variacoes: None
                }
                ])
            },
            CaracteristicaHabilidadeClasse{
                nome: "Energizar".to_string(),
                descricao: "Começando no 3° nível, você pode energizar magicamente
                sua munição rúnica. Sempre que você fizer um novo lote
                de munição rúnica, você pode gastar 1 carga rúnica para
                energizá-lo, todas as peças do lote são consideradas
                mágicas com o propósito de superar a resistência
                a ataques e danos não-mágicos. O lote se
                mantém energizado até o próximo descanso
                longo.
                Adicionalmente, você pode utilizar uma Carga
                Rúnica para encantar duas das suas munições do lote
                energizado com uma das energizações abaixo através
                de uma ação bônus. No 9o nível, ao encantar uma
                munição, você pode aplicar 2 efeitos em uma mesma
                munição.".to_string(),
                variacoes: None,
                caracteristicas: Some(vec![
                CaracteristicaHabilidadeClasse{
                    nome: "Flamejante".to_string(),
                    descricao: "Quando você acerta uma criatura,
                    você incendeia o alvo por 1d4 rodadas. A cada
                    rodada ele recebe um dado de carga rúnica
                    como dano ígneo (isto não lhe custa nenhuma
                    carga rúnica). O efeito da condição desse
                    recurso não é acumulativo.".to_string(),
                    variacoes: None,
                    caracteristicas: None
                },
                CaracteristicaHabilidadeClasse{
                    nome: "Encantador".to_string(),
                    descricao: "Sua munição é infundida com
                    um potente feromônio. Quando você acertar
                    uma criatura, seu alvo deve ter sucesso em uma
                    salvaguarda de Carisma CD 13 + um dado de carga
                    rúnica ou fica enfeitiçada até o seu próximo turno. Você
                    pode gastar 1 dado de carga rúnica adicional e subtrair o
                    número acumulado da salvaguarda de Carisma da criatura
                    alvo. Você só pode ter um alvo enfeitiçado de cada vez.".to_string(),
                    caracteristicas: None,
                    variacoes: None
                },
                CaracteristicaHabilidadeClasse{
                    nome: "Envenenador".to_string(),
                    descricao: "Quando você acertar uma criatura com uma
                    munição envenenada, ela deve fazer uma salvaguarda de
                    Constituição de CD 8 + o dado de carga rúnica. Em uma
                    falha, o alvo recebe a condição Intoxicado por 1 minuto
                    além de 1d8 de dano venenoso adicional. Em um teste
                    bem sucedido, o alvo só recebe o dano do ataque e o dano
                    venenoso adicional. Você pode gastar 1 dado de carga rúnica
                    adicional e subtrair o número rolado da salvaguarda de
                    Constituição do alvo.".to_string(),
                    caracteristicas: None,
                    variacoes: None
                },
                CaracteristicaHabilidadeClasse{
                    nome: "Afiado".to_string(),
                    descricao: "Quando você causa dano com uma munição a
                    uma criatura, a criatura deve fazer uma salvaguarda de
                    Constituição CD 13 + o dado de carga rúnica. Em caso de
                    falha, a criatura começa a sangrar no início de cada um de seus turnos, recebendo 1d4 de dano extra. Eles podem repetir a salvaguarda
                    no final de cada um de seus turnos para terminar este efeito. Um teste
                    bem sucedido de Medicina também pode interromper o sangramento.
                    Esse recurso não tem efeito em construtos ou mortos vivos. O efeito da
                    condição desse recurso não é cumulativo. Você pode gastar 1 dado de
                    carga rúnica adicional e subtrair o número acumulado da salvaguarda de
                    Constituição do alvo.".to_string(),
                    variacoes: None,
                    caracteristicas: None
                },
                CaracteristicaHabilidadeClasse{
                    nome: "Atordoamento".to_string(),
                    descricao: "Quando você ataca uma criatura, ela deve ser bem
                    sucedida em uma salvaguarda de Constituição com CD 13 + um dado
                    de carga rúnica, ou ficará atordoado até o seu próximo turno. Você não
                    precisa gastar uma carga rúnica para determinar o CD. Você pode gastar
                    1 dado de carga rúnica adicional e subtrair o número acumulado da
                    salvaguarda de Constituição da criatura alvo.".to_string(),
                    caracteristicas: None,
                    variacoes: None
                }
                ])
            },
            CaracteristicaHabilidadeClasse{
                nome: "Mãos Rápidas".to_string(),
                descricao: "Começando no 5° nível, você pode disparar sua munição com maior
velocidade. Sua munição agora causa 1d8 de dano em um acerto, e seu
alcance se torna 60/ 120 pés.".to_string(),
                caracteristicas: None,
                variacoes: None
            },
            CaracteristicaHabilidadeClasse{
                nome: "Combate Rápido".to_string(),
                descricao: "A partir do 7° nível, ninguém se compara à sua velocidade. Você
                realmente entende o que é ser “rápido”. Você recebe as seguintes
                características:".to_string(),
                variacoes: None,
                caracteristicas: Some(vec![
                CaracteristicaHabilidadeClasse{
                    nome: "Ataque Extra".to_string(),
                    descricao: "A partir do 7° nível, você pode atacar duas vezes,
                    ao invés de uma, quando usar a ação Atacar durante o seu turno.".to_string(),
                    caracteristicas: None,
                    variacoes: None
                },
                CaracteristicaHabilidadeClasse{
                    nome: "Evasão".to_string(),
                    descricao: "A partir do 7o nível, você pode esquivar-se agilmente de
                    certos efeitos em área, como o sopro flamejante de um dragão
                    vermelho ou uma magia tempestade glacial. Quando você for alvo
                    de um efeito que exige uma salvaguarda de Destreza para sofrer
                    metade do dano, você não sofre dano algum se passar, e somente
                    metade do dano se falhar. Além disso você pode adicionar seu
                    modificador de Carisma à sua Iniciativa.".to_string(),
                    variacoes: None,
                    caracteristicas: None,
                },
                ])
            },
            CaracteristicaHabilidadeClasse{
                nome: "Estilo Encantador".to_string(),
                descricao: "Começando no 9° nível, seu charme se torna extraordinariamente
                cativante. Com uma ação, você pode fazer um teste de Persuasão
                resistido por um teste de Intuição do alvo. A criatura deve ser capaz de
                ouvi-lo e vocês dois devem compartilhar um idioma.
                Se você obtiver sucesso no teste e a criatura for hostil a você, ela terá
                desvantagem em jogadas de ataque contra alvos que não sejam você e
                não poderá fazer ataques de oportunidade contra alvos que não sejam
                você. Esse efeito dura por 1 minuto, até que um de seus companheiros
                ataque o alvo ou o afete com uma magia, ou até você e o alvo estarem
                separados por mais de 60 pés.".to_string(),
                caracteristicas: None,
                variacoes: None
            },
            CaracteristicaHabilidadeClasse{
                nome: "Especialização em Combate".to_string(),
                descricao: "Começando no 11o nível, se o seu ataque errar um alvo dentro do
                alcance, você pode gastar uma carga rúnica para transformar o erro em
                um acerto.".to_string(),
                caracteristicas: None,
                variacoes: None
            },
            CaracteristicaHabilidadeClasse{
                nome: "Recarga Rúnica".to_string(),
                descricao: "No 13o nível você pode usar duas munições rúnicas se preferir, seguindo
                as regras normais de empunhadura dupla, além disso parece que sua
                munição nunca diminui. No final de uma luta, você pode recuperar
                algumas de suas munições, jogue um dado de carga rúnica e adicione
                esse número de munição de volta ao seu inventário.".to_string(),
                variacoes: None,
                caracteristicas: None
            },
            CaracteristicaHabilidadeClasse{
                nome: "Intuição do Apostador".to_string(),
                descricao: "Começando no 17o nível, sua inteligência é aprimorada a ponto de sua
                mente não ser facilmente influenciada por truques, ilusões de ótica e
                feitiços. Quando você faz uma salvaguarda de Inteligência, Sabedoria ou
                Carisma, você pode gastar um dado de carga rúnica para abaixar a CD
                para você e quaisquer aliados que possam ouvi-lo, se você lhes disser. O
                CD é subtraído pelo número rolado do dado de carga rúnica.".to_string(),
                caracteristicas: None,
                variacoes: None
            }
            ])
        }
        ]),
        variacoes: None,
    };


    let mercurial = AtributosClasse{
        nome: "Mercurial".to_string(),
        dado_vida: "1d8 por nível de Mercurial".to_string(),
        pontos_vida: 8.0,
        modificador_pontos_vida: "Constituição".to_string(),
        pontos_vida_nivel: 5.0,
        dados_vida_nivel: "1d8 + seu modificador de Constituição por nível de Mercurial".to_string(),
        proficiencias_armaduras: Some(vec!["Armadura Leve".to_string()]),
        proficiencias_armas: Some(vec!["Armas Simples".to_string(), "Besta Leve".to_string(), "Espada Longa".to_string(), 
        "Rapieira".to_string(), "Espada Curta".to_string()]),
        proficiencias_oficios: Some(vec!["Ofício de Chaveiro".to_string()]),
        proficiencias_salvaguardas: Some(vec!["Destreza".to_string(), "Inteligência".to_string()]),
        proficiencias_pericias: None,
        equipamento_base: Some(vec![
            EquipamentoBase{
                opcoes: vec!["Rapieira".to_string(), "Espada Curta".to_string()]
            },
            EquipamentoBase{
                opcoes: vec!["Arco Curto, Aljava com 20 Flechas".to_string(), "Espada Curta".to_string()],
            },
            EquipamentoBase{
                opcoes: vec!["Conjunto de Assaltante".to_string(), "Conjunto de Explorador  de Masmorras".to_string(),
                "Conjunto de Aventureiro".to_string()]
            },
            EquipamentoBase{
                opcoes: vec!["Armadura de Couro, Duas Adagas, Ferramentas de Chaveiro".to_string()]
            }
        ]),
        informacoes_conjuracao: None,
        lista_magias: None,
        qtd_proficiencias_pericias: Some(4.),
        opcoes_pericias: Some(vec!["Acrobacia".to_string(), "Atletismo".to_string(), "Enganação".to_string(), 
        "Intuição".to_string(), "Intimidação".to_string(), "Investigação".to_string(), "Percepção".to_string(),
        "Atuação".to_string(), "Persuasão".to_string(), "Prestidigitação".to_string(), "Furtividade".to_string()]),
        titulo_subclasse: Some("Cabala".to_string()),
        descricao_subclasse: Some( "Embora os Mercuriais tenham muitas características parecidas entre
        si, cada origem e treinamento acaba guiando suas especializações para caminhos diferentes fazendo com que dois Mercuriais possam diferir
        muito em suas capacidades em níveis mais elevados. A Cabala escolhida
        reflete seu treinamento e foco, não necessariamente a sua profissão, mas
        ela delineia as pessoas com as quais você tem contato e onde obtém sua
        instrução.".to_string()),
        lista_subclasses: Some(vec![subclasse_1, subclasse_2, subclasse_3, subclasse_4]),
        habilidades: Some(vec![
        HabilidadeClasse{
            nome: "Especialização".to_string(),
            descricao: "No 1o nível, você escolhe uma de suas perícias que seja proficiente, ou
            um ofício. Assim recebendo Especialização nessa perícia ou ofício, a
            Especialização faz com que seu bônus de proficiência seja dobrado em
            qualquer teste de atributo que fizer com esta perícia ou ofício.
            No 3o nível, você pode escolher mais uma de suas proficiências (em
            perícia ou ofício) para ganhar esse benefício.
            No 6o nível, você pode escolher mais duas de suas proficiências (em
            perícia ou ofício de chaveiro) para ganhar esse benefício.".to_string(),
            caracteristicas: None,
            variacoes: None
        },
        HabilidadeClasse{
            nome: "Ataque Furtivo".to_string(),
            descricao: "A partir do 1o nível, você sabe como atacar sutilmente e explorar a
            distração de seus inimigos. Uma vez por turno, você pode adicionar 1d6 nas jogadas de dano contra qualquer criatura que acertar, desde
            que tenha vantagem nas jogadas de ataque. O ataque deve ser com uma
            arma de acuidade ou à distância.
            Você não precisa ter vantagem nas jogadas de ataque se outro inimigo
            do seu alvo estiver a 5 pés de distância dele, desde que este inimigo
            não esteja incapacitado e você não tenha desvantagem nas jogadas de
            ataque.
            A quantidade de dano extra aumenta conforme você ganha níveis nessa
            classe, como mostrado na coluna Ataque Furtivo da tabela O Mercurial.".to_string(),
            variacoes: None,
            caracteristicas: None
        },
        HabilidadeClasse{
            nome: "Cabala".to_string(),
            descricao: "No 1o nível, você escolhe um 'grupo' ao qual você se conecta mesmo
            que apenas filosoficamente. Escolha entre a Cabala do Ás Rúnico, a
            Cabala do Assassino, a Cabala do Explorador ou a Cabala do Trapaceiro
            Rúnico, todas detalhadas no final da descrição de classe. Sua escolha lhe
            concederá caracterísitcas no 1o nível e novamente no 3o, 5o, 7o, 9o, 11o,
            13o e 17o níveis.".to_string(),
            variacoes: None,
            caracteristicas: None
        },
        HabilidadeClasse{
            nome: "Ação Ardilosa".to_string(),
            descricao: "Começando no 2o nível, seu pensamento rápido e agilidade lhe permitem
            se mover e agir rapidamente. Você pode executar essas ações como
            ação bônus: Correr, Desengajar ou Esconder.".to_string(),
            caracteristicas: None,
            variacoes: None
        },
        HabilidadeClasse{
            nome: "Aprimoramento".to_string(),
            descricao: "Quando você atinge o 4o nível e novamente no 8o, 10o, 12o, 16o e 19o
            nível, você pode aumentar um valor de Atributo, à sua escolha, em 2
            ou você pode aumentar dois valores de Atributo, à sua escolha, em
            1. Como padrão, você não pode elevar um valor de Atributo acima
            de 20 com essa característica. Alternativamente você pode escolher
            um dos Aprimoramentos Gerais do Capítulo 5: Personalização -
            Aprimoramentos.".to_string(),
            caracteristicas: None,
            variacoes: None
        },
        HabilidadeClasse{
            nome: "Sentido Cego".to_string(),
            descricao: "No 14o nível, se você for capaz de ouvir, você está ciente da localização
            de qualquer criatura escondida ou invisível a até 10 pés de você.".to_string(),
            caracteristicas: None,
            variacoes: None
        },
        HabilidadeClasse{
            nome: "Mente Escorregadia".to_string(),
            descricao: "No 15o nível, você adquire uma grande força de vontade, adquirindo
            proficiência na salvaguarda de Sabedoria.".to_string(),
            variacoes: None,
            caracteristicas: None,
        },
        HabilidadeClasse{
            nome: "Elusivo".to_string(),
            descricao: "A partir do 18o nível, você se torna tão sagaz que raramente alguém
            encosta a mão em você. Nenhuma jogada de ataque tem vantagem
            contra você, desde que você não esteja incapacitado.".to_string(),
            variacoes: None,
            caracteristicas: None
        },
        HabilidadeClasse{
            nome: "Golpe de Sorte".to_string(),
            descricao: "No 20o nível, você tem um talento incrível para ter sucesso quando mais
            precisa. Se o seu ataque erra um alvo no alcance, você pode transformar
            o erro em um acerto. Alternativamente, se falhar em um teste de
            habilidade, você pode tratar a jogada de d20 como um 20.
            Uma vez que use esta característica, você não pode utilizá-la novamente
            até terminar um descanso curto ou longo.".to_string(),
            caracteristicas: None,
            variacoes: None
        }
        ])
    };
    return mercurial;
}

pub fn info_classes() -> Vec<AtributosClasse> {
    let mut origens = vec![];
    origens.push(info_mercurial());

    return origens;
}

#[get("/classes")]
pub async fn lista_classes() -> Json<Vec<AtributosClasse>>{
    let classes = info_classes();
    return Json(classes)
}