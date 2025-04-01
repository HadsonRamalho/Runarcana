import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip"
import { ArrowBigDown, ArrowBigUp, ArrowDown, ArrowUp } from "lucide-react";
import { useEffect, useState } from "react";
import { InformacaoHabilidade, InformacaoLinhagem } from "../criar_ficha";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Input } from "@/components/ui/input";

const informacoes_campos = {
    longevidade: `Uma medida comum aos povos de diversas origens são os anos
    contados desde o nascimento ou concepção daquele indivíduo. Essa medida serve principalmente para definir 
    quando esse personagem alcança determinadas faixas de idade. Ao criar seu personagem você
    pode escolher um idade para ele. Essa idade poderá justificar atributos
    mas ela ajuda também a definir pelo que você já passou.`,
    pontos_heranca: `A Herança é o primeiro traço de Origem, ela representa os traços de
    seus antepassados que se tornaram mais fortes em você. Ao escolher
    uma Origem, ela lhe concede uma quantidade de pontos de Herança
    que podem ser gastos durante a criação do personagem em algumas
    característica ligadas à região ou mesmo a uma Origem específica.
    Ao escolher uma Herança, você está dizendo quais foram os aspectos
    mais importantes na criação e no crescimento de seu personagem, quais
    fatores genéticos e ambientais foram pemeraldominantes e acabaram por ser
    aflorados, seja pelo seu treino ou pelas condições enfrentadas em vida.
    Mais detalhes sobre as Heranças podem ser vistos no Capítulo 5:
    Personalização do Livro de Regras.`,
    tendencia: `Cada indivíduo tem sua própria tendência, embora seja comum que
      indivíduos de mesma origem compartilhem traços em comum, algumas
      vezes tendo exatamente iguais ao do resto dos membros de sua origem.
      Essa tendência pode refletir como a sua origem encara aspectos como
      lei, religião, magia, tecnologia entre muitos outros que compõem uma
      sociedade, podendo ser escolhidas de forma a refletir aquilo em que seu
      personagem acemeraldita e como ele se comporta em relação a isso.`,
    tamanho: `Quanto ao tamanho, as Origens no geral partilham da mesma classe de
      tamanho Médio que tem entre 1,20 e 2,40 metros, as únicas exceções
      sendo o Yordle que é considerado Pequeno e o Troll que é considerado
      Grande. Quando o indivíduo possui um valor fora desse padrão, isso
      reflete em mecânicas, sejam elas para tratar de uma criatura Pequena ou
      Grande. Mais detalhes sobre isso, estão no Capítulo 9 do Livro de Regras.`,
    deslocamento: `Esse valor representa como quão rápido e longe você se move seja
      em viagem ou em combate. Mais informações sobre isso podem ser
      encontradas nos Capítulos 8 e 9.`,
    idiomas: `
      A imensidão de origens gera uma imensidão ainda maior de idiomas,
      línguas e dialetos. Em uma realidade como a de Runeterra, com milênios
      de anos de história, muitas dessa línguas se perderam no tempo enquanto
      outras foram mantidas de forma quase ininterrupta. Mais detalhes sobre
      as línguas de Runeterra podem ser encontrados no Capítulo 4.`,
    proficiencia: `
    Como membros de uma socidade em algum momento de sua vida, cada
    personagem tem acesso a determinadas proficiências baseadas em sua
    Origem, sejam elas pericias, proficiências com armas em especifico ou
    mesmo Ofícios.
    Essa característica reflete um traço comum naquela Origem e
    que é compartilhado pelos diversos membros da mesma, seja por
    ser algo ativamente ensinado ou mesmo uma habilidade que seja
    adquirida indiretamente durante o crescimento e desenvolvimento da
    personalidade em contato com os membros daquela Origem.`,
    regiao: `
    É fato que muitas Origens são endêmicas em algumas regiões, no
entanto boa parte delas está espalhada por Runeterra, seja em diferentes
cidades, assentamentos ou mesmo tribos. Ao definir sua região, você
está definindo mais do que seu local de nascimento, mas sim, em que
lugar você teve seu florescimento enquanto uma pessoa.
Caso você tenha morado em diversas regiões, essa escolha está ligada
a qual região afetou você com maior profundidade, talvez tenha sido
exatamente a sua terra natal, ou talvez você tenha sido melhor acolhido
em outra região.
As informações sobre Região podem ser encontradas no Capítulo 11:
Runeterra, detalhando quais proficiências você tem acesso por ter
aquela região como a sua região padrao.`,
    habilidades_especiais: `
    Cada Origem tem suas particularidades, algumas delas possuem certas
habilidades que flertam com a magia, são capacidades que muitas vezes
derivam de contato com a mais pura energia arcana ou mesmo com
níveis diferentes de profundidade em relação ao reino espiritual.
Algumas vezes essas habilidades são fruto de alguma condição física
específica das criaturas como uma capacidade anfíbia ou mesmo um
poderoso par de chifres que podem ser usados em combate.
Cada Habilidade Especial possui seu detalhamento e suas regras em
particular. Algumas vezes, as mesmas podem ser amplificadas ou
modificadas (ou até mesmo se tornem presentes) através de Heranças.`,
    linhagem: `A variação entre as origens é sempe existente, dois membros podem
até ter muitos traços semelhantes mas acabam por ter seu diferencial.
Algumas origens possuem derivações sutis que dificilmente são
percebidas por membros de outras origens, no entanto algumas outras
derivações podem ser perceptivelmente diferentes.
As linhagens representam membros que embora derivem de uma origem,
possuem características que os separam, que podem ser características
simplesmente culturais e em algumas vezes até mesmo fisiológicas.
As duas origens com maior número de variações em Runeterra são
os Humanos (os mais numerosos) e os Vastaya. Enquanto entre os
humanos essas linhagens costumam ser derivadas de regiões, no caso
dos Vastaya refere-se ao seus antepassados e pais, inclusive definindo
assim muitos de seus traços e geralmente sua tribo.`
}


export const converter_pes_metros = (pes: number):number => {
  return parseInt((pes * 0.3048).toFixed(0)); // 1 pé = 0.3048 metros
}


interface InformacaoCampoProps {
  informacao: string;
}

const InformacaoCampo = ({ informacao }: InformacaoCampoProps) => {
  return (
    <TooltipProvider>
      <Tooltip>
        <TooltipTrigger>
          <div className="rounded-full bg-white w-7 h-7 ml-1 pb-1"><h4>?</h4></div>
        </TooltipTrigger>
        <TooltipContent className="bg-transparent">
          <Card className="bg-emerald-600 max-w-[400px]">
            <CardContent className="items-center justify-center">
              <p>{informacao}</p>
            </CardContent>
          </Card>
        </TooltipContent>
      </Tooltip>
    </TooltipProvider>
  );
};

interface InformacaoGlossarioProps{
  dados: {
    nome: string;
    descricao: string;
  };
}

export const InformacaoGlossario = ({dados}: InformacaoGlossarioProps) => {
  const [showInfo, setShowInfo] = useState(false);
  return (
    <Card className="flex items-center justify-center p-2 md:p-4 bg-[var(--emerald-1)] w-full">
      <CardContent>
        <div  className="flex items-center justify-center">
        <span className="flex items-center justify-center">
          <strong>{dados.nome}</strong>
        </span>
        {showInfo ? (
          <ArrowBigUp onClick={() => setShowInfo(!showInfo)} className="cursor-pointer" />
        ) : (
          <ArrowBigDown onClick={() => setShowInfo(!showInfo)} className="cursor-pointer" />
        )}
        </div>
        {showInfo && (
          <div className="mt-2">
        <span>{dados.descricao}</span>
          </div>
        )}
      </CardContent>
    </Card>
  )
}

interface OrigemProps{
  origem: Origem
}

export const InformacaoOrigem = ({origem}: OrigemProps) => {
  const [exibirOrigem, setExibirOrigem] = useState(false);
  return (
    <Card className="mt-4">
          <CardHeader className="flex items-center justify-center">
              <h4>{origem.nome}</h4> 
              {exibirOrigem ? (
                <ArrowUp onClick={() => {setExibirOrigem(!exibirOrigem)}}/>
              ) : (<ArrowDown onClick={() => {setExibirOrigem(!exibirOrigem)}}/>)}
          </CardHeader>
            {(origem && exibirOrigem) && (
              <CardContent>
                <div>
                  <Label>Resumo</Label>
                  <div className="w-full border-1 border-black rounded-md p-1 mt-2">{origem.resumo}</div>
                </div>
                <CardContent className="grid grid-cols-1 md:grid-cols-3 mt-4 gap-4 p-0">
                  {(origem.quantidade_herancas && origem.quantidade_herancas > 0) && (
                    <div>
                    <Label className="mb-2">Heranças: </Label>
                    <InformacaoGlossario dados={{nome: `${origem.quantidade_herancas} Heranças`, descricao: origem.descricoes_origem.heranca}}/>
                  </div>
                  )}
                  <div>
                    <Label className="mb-2">Idade: </Label>
                      <InformacaoGlossario dados={{nome: `Idade Máxima: ${origem.idade_maxima} anos`, descricao: origem.descricoes_origem.idade}} />
                  </div>
                  <div>
                    <Label className="mb-2">Tamanho:</Label>
                    <InformacaoGlossario dados={{nome: `Entre ${origem.tamanho_minimo}m e ${origem.tamanho_maximo}m`, descricao: origem.descricoes_origem.tamanho}}/>
                  </div>
                  <div>
                    <Label className="mb-2">Deslocamento base:</Label>
                    <InformacaoGlossario dados={{nome: `${origem.deslocamento} pés (${converter_pes_metros(origem.deslocamento)} metros)`, descricao: origem.descricoes_origem.deslocamento}} />
                  </div>
                  <div>
                    <Label className="mb-2">Idiomas: </Label>
                    <InformacaoGlossario dados={{nome: `${origem.idiomas.map((idioma) => ('\n' + idioma))} e ${origem.quantidade_idiomas} idiomas adicionais`, descricao: `${origem.descricoes_origem.idiomas} `}}/>
                  </div>
                  {origem.pericias && (
                    <div>
                    <Label className="mb-2">Perícias:</Label>
                    <InformacaoGlossario dados={{nome: `${origem.pericias?.length} perícias`, descricao: `${origem.descricoes_origem.pericia}`}} />
                  </div>
                  )}
                  <div>
                    <Label className="mb-2">Região:</Label>
                    <InformacaoGlossario dados={{nome: `Região: ${origem.regiao}`, descricao: origem.descricoes_origem.regiao}} />
                  </div>
                  {(origem.qtd_oficios && origem.qtd_oficios >= 1 && origem.descricoes_origem.oficio)&& (
                    <div>
                    <Label className="mb-2">Ofícios:</Label>
                    <InformacaoGlossario dados={{nome: `${origem.qtd_oficios} Ofícios`, descricao: origem.descricoes_origem.oficio}} />
                  </div>
                  )}
                  {(origem.qtd_aprimoramentos && origem.qtd_aprimoramentos >= 1 && origem.descricoes_origem.aprimoramento	)&& (
                    <div>
                    <Label className="mb-2">Aprimoramentos:</Label>
                    <InformacaoGlossario dados={{nome: `${origem.qtd_aprimoramentos} Aprimoramentos`, descricao: origem.descricoes_origem.aprimoramento}} />
                  </div>
                  )}
                </CardContent>
              </CardContent>
            )}
            {(origem && origem.habilidades_especiais && exibirOrigem) &&(
              <CardContent className="bg-emerald-100 rounded-xl pb-4 pt-4">
              <CardHeader className="flex items-center justify-center">
              <h4>Traços de Origem</h4>
              </CardHeader>
            {( origem.habilidades_especiais.length > 0 && exibirOrigem) && 
            origem.habilidades_especiais.map((habilidade, index) => (
              <div key={index} className="mb-4">
                  <InformacaoHabilidade habilidade={habilidade}/>
              </div>
            ))
            }
          </CardContent>      
            )}
            {(origem && origem.linhagens && exibirOrigem) &&(
              <CardContent className="bg-emerald-100 rounded-xl pb-4 pt-4 p-1">
              <CardHeader className="flex items-center justify-center">
              <h4>Linhagens</h4>
              </CardHeader>
            {( origem.linhagens.length > 0) && 
            origem.linhagens.map((linhagem, index) => (
              <div key={index} className="mb-4 w-full p-0 m-0">
                  <InformacaoLinhagem linhagem={linhagem}/>
              </div>
            ))
            }
          </CardContent>      
            )}
          </Card>
  )
}


interface Variacao{
  nome: string;
  descricao: string;
  recarga: string | null;
  vantagens: string[] | null;
}

export interface HabilidadeEspecial{
  nome: string,
  descricao: string,  
  variacoes: Variacao[] | null,
  resistencias: string[] | null,
  vulnerabilidades: string[] | null,
  imunidades: string[] | null,
  recarga: string | null
}

interface DescricoesOrigem {
  heranca: string,
  idade: string,
  tendencia: string,
  tamanho: string,
  deslocamento: string,
  idiomas: string,
  proficiencia: string | null,
  regiao: string,

  pericia: string | null,
  aprimoramento: string | null,
  oficio: string | null,
}

export interface Linhagem{
  nome: string;
  descricao: string;
  heranca: string | null;
  quantidade_herancas: number | null;
  deslocamento: number,
  habilidades_especiais: HabilidadeEspecial[] | null,

  bonus_constituicao: number | null,
}

export interface Origem{
  nome: string;
  resumo: string;
  idade_maxima: number,
  tamanho: string;
  deslocamento: number,
  idiomas: string[],
  regiao: string[],

  quantidade_herancas: number | null,
  quantidade_idiomas: number,
  tamanho_minimo: number,
  tamanho_maximo: number,
  tipo_tamanho: string,
  pericias: string[] | null,
  qtd_aprimoramentos: number | null,
  oficios: string[] | null,
  qtd_oficios: number | null,

  habilidades_especiais: HabilidadeEspecial[] | null,
  descricoes_origem: DescricoesOrigem,

  linhagens: Linhagem[] | null,
}

export function Origens(){
  const [racas, setRacas] = useState<Origem[]>([]);

  const [filtro, setFiltro] = useState<string | null>();
  const [busca, setBusca] = useState<string | null>();

  const filtros = [
    "Nome da origem",
    "Habilidade",
    "Proficiência"
  ];

  useEffect(() => {
    const fetchOrigens = async () => {
      try {
        const res = await fetch("http://localhost:3060/");
        if (!res.ok) {
          throw new Error("Erro ao carregar os dados");
        }
        const data = await res.json();
        setRacas(data);
      } catch (error) {
        console.error(error);
        alert("Erro ao carregar os dados. Verifique o console para mais detalhes.");
      }
    };

    fetchOrigens();
  }, []);


  return(
    <div className="flex items-center justify-center w-full h-full mt-10 mb-10">
      <Card className="w-[95%]">
        <CardHeader className="flex flex-col items-center justify-center pb-10 rounded-xl ml-4 mr-4">
          <h4>Glossário de Traços</h4>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mt-4 p-1 md:p-4 rounded-md w-full">
            <InformacaoGlossario dados={{ nome: "Pontos de Herança", descricao: informacoes_campos.pontos_heranca }} />
            <InformacaoGlossario dados={{ nome: "Idade", descricao: informacoes_campos.longevidade }} />
            <InformacaoGlossario dados={{ nome: "Tendência", descricao: informacoes_campos.tendencia }} />
            <InformacaoGlossario dados={{ nome: "Tamanho", descricao: informacoes_campos.tamanho }} />
            <InformacaoGlossario dados={{ nome: "Deslocamento", descricao: informacoes_campos.deslocamento }} />
            <InformacaoGlossario dados={{ nome: "Idiomas", descricao: informacoes_campos.idiomas }} />
            <InformacaoGlossario dados={{ nome: "Proficiência", descricao: informacoes_campos.proficiencia }} />
            <InformacaoGlossario dados={{ nome: "Região", descricao: informacoes_campos.regiao }} />
            <InformacaoGlossario dados={{ nome: "Habilidades Especiais", descricao: informacoes_campos.habilidades_especiais }} />
            <InformacaoGlossario dados={{ nome: "Linhagem", descricao: informacoes_campos.linhagem }} />
          </div>
        </CardHeader>
        <CardContent>
          <Input placeholder="Pesquisar origens" onChange={(e) => {setBusca(e.target.value)}}/>
          <Select onValueChange={(e) => {setFiltro(e)}}>
           <SelectTrigger className="w-auto mt-2 bg-[var(--red-1)]">
            <SelectValue placeholder="Selecione um filtro" />
          </SelectTrigger>
          <SelectContent className="bg-[var(--primary)]">
          <SelectItem value="Selecione">Selecione um filtro</SelectItem>
          {filtros.map((filtroM) => (
            <SelectItem value={filtroM}>{filtroM}</SelectItem>
          ))}
          </SelectContent>
          </Select>

            {racas
            .filter((raca) => {
              if (!filtro || filtro === "Selecione") return true;
              if (filtro === "Nome da origem" && busca) return raca.nome.toLowerCase().includes(busca.toLowerCase());
              if (filtro === "Habilidade" && busca && raca.habilidades_especiais && raca.habilidades_especiais.length > 0) return raca.habilidades_especiais.some((habilidade) => habilidade.nome.toLowerCase().includes(busca.toLowerCase()));
              if (filtro === "Proficiência" && busca && raca.pericias) return raca.pericias.some((pericia) => pericia.toLowerCase().includes(busca.toLowerCase()));
              return false;
            })
            .map((raca) => (
              <div key={raca.nome} className="mb-4">
              <InformacaoOrigem origem={raca} />
              </div>
            ))}
        </CardContent>
      </Card>
    </div>
    )
}