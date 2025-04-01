export interface AtributosClasse {
    nome: string;
    dado_vida: string;
    pontos_vida: number;
    modificador_pontos_vida: string;
    pontos_vida_nivel: number;
    dados_vida_nivel: string;

    proficiencias_armaduras?: string[];
    proficiencias_armas?: string[];
    proficiencias_oficios?: string[];
    proficiencias_salvaguardas?: string[];
    proficiencias_pericias?: string[];

    qtd_proficiencias_pericias?: number;
    opcoes_pericias?: string[];

    equipamento_base?: EquipamentoBase[];

    informacoes_conjuracao?: InformacoesConjuracao;
    lista_magias?: InformacoesMagia[];

    lista_subclasses?: Subclasse[];
    titulo_subclasse?: string;
    descricao_subclasse?: string;

    habilidades?: HabilidadeClasse[];
}

export interface InformacoesConjuracao{
    qtd_truques: string,
    lista_magia: string,
    qtd_magias_inicial: number,

    habilidade_conjuracao: string,

    info_truques: string,
    info_espacos_magia: string,
    info_magias_conhecidas: string,
    info_habilidade_conjuracao: string,

    cd_magia: string,
    ataque_magia: string,

    conjuracao_ritual?: string,
    foco_conjuracao?: string,
}

export interface InformacoesMagia {
    nome: string;
    descricao: string;
    componente_sinal?: string;
    componente_verbal?: string;
    componente_materia?: string;

    alcance?: string;
    duracao?: string;
}

export interface EquipamentoBase {
    opcoes: string[];
}

export interface HabilidadeClasse {
    nome: string;
    descricao: string;

    caracteristicas?: CaracteristicaHabilidadeClasse[];
    variacoes?: VariacaoHabilidadeClasse[];
}

export interface CaracteristicaHabilidadeClasse {
    nome: string;
    descricao: string;
    caracteristicas?: CaracteristicaHabilidadeClasse[];
    variacoes?: VariacaoHabilidadeClasse[];
}

export interface VariacaoHabilidadeClasse {
    nome: string;
    descricao: string;
}

export interface Subclasse {
    nome: string;
    descricao: string;

    proficiencias_armaduras?: string[];
    proficiencias_armas?: string[];
    proficiencias_oficios?: string[];
    proficiencias_salvaguardas?: string[];
    proficiencias_pericias?: string[];

    informacoes_conjuracao?: InformacoesConjuracao;

    habilidades?: HabilidadeClasse[];
    variacoes?: VariacaoHabilidadeClasse[];
}
