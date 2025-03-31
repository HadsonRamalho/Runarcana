import { Card, CardContent, CardHeader } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
  } from "@/components/ui/select"
import { useEffect, useState } from "react"
import { converter_pes_metros, HabilidadeEspecial, InformacaoGlossario, InformacaoOrigem, Linhagem, Origem } from "../origens"
import { ArrowBigDown, ArrowBigUp, ArrowDown, ArrowUp, Info } from "lucide-react"
  
const origens = [
    "Humano",
    "Antroplantae",
];

interface InformacaoLinhagemProps{
  linhagem: Linhagem
}

interface InformacaoHabilidadeProps{
  habilidade: HabilidadeEspecial;
}

export const InformacaoHabilidade = ({habilidade}: InformacaoHabilidadeProps) => {
  const [showInfo, setShowInfo] = useState(false);
  return (
    <Card className="flex items-center justify-center p-4 bg-[var(--red-1)] w-full p-0 m-0">
      <CardContent className="p-2">
        <div  className="flex items-center justify-center">
        <span className="flex items-center justify-center">
          <strong>{habilidade.nome}</strong>
        </span>
        {showInfo ? (
          <ArrowBigUp onClick={() => setShowInfo(!showInfo)} className="cursor-pointer" />
        ) : (
          <ArrowBigDown onClick={() => setShowInfo(!showInfo)} className="cursor-pointer" />
        )}
        </div>
        {showInfo && (
          <div className="mt-2">
          <span>{habilidade.descricao}</span>
          {(habilidade.variacoes && habilidade.variacoes.length > 0) && 
            habilidade.variacoes.map((variacao) => (
              <div key={variacao.nome} className="mb-2">
                <InformacaoGlossario dados={{nome: variacao.nome, descricao: variacao.descricao}}/>
              </div>
            ))}
          </div>
        )}
      </CardContent>
    </Card>
  )
}

export const InformacaoLinhagem = ({linhagem}: InformacaoLinhagemProps) => {
  const [showInfo, setShowInfo] = useState(false);
  return (
    <Card className="flex items-center justify-center p-4 bg-[var(--red-1)] w-full p-0 m-0">
      <CardContent className="p-1 m-0">
        <div  className="flex items-center justify-center">
        <span className="flex items-center justify-center">
          <strong>{linhagem.nome}</strong>
        </span>
        {showInfo ? (
          <ArrowBigUp onClick={() => setShowInfo(!showInfo)} className="cursor-pointer" />
        ) : (
          <ArrowBigDown onClick={() => setShowInfo(!showInfo)} className="cursor-pointer" />
        )}
        </div>
        {showInfo && (
          <div className="mt-2">
          <span>{linhagem.descricao}</span>
          {linhagem.habilidades_especiais && linhagem.habilidades_especiais.length > 0 && (
            <div className="flex items-center justify-center">
            <h4>Traços da Linhagem: </h4>
            </div>
          )}
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          {(linhagem.habilidades_especiais && linhagem.habilidades_especiais.length > 0) && 
          linhagem.habilidades_especiais.map((habilidade) => (
            <InformacaoHabilidade habilidade={habilidade}/>
          ))}
          </div>
          </div>
        )}
      </CardContent>
    </Card>
  )
}

function verifica_modificador(valor: number) {
    switch (valor) {
      case 8:
      case 9:
        return -1;
      case 10:
      case 11:
        return 0;
      case 12:
      case 13:
        return 1;
      case 14:
      case 15:
        return 2;
      case 16:
      case 17:
        return 3;
      case 18:
      case 19:
        return 4;
      case 20:
      case 21: 
       return 5;
      default:
       return 0.
    }
  }

export const CriarFicha = () => {
  const [origens, setOrigens] = useState<Origem[]>([]);

  const [origem, setOrigem] = useState<Origem>();

  const [exibirOrigem, setExibirOrigem] = useState(true);
  const [exibirBasico, setExibirBasico] = useState(true);
  const [exibirAtributos, setExibirAtributos] = useState(true);

  const [forca, setForca] = useState<number>(10);
  const [destreza, setDestreza] = useState<number>(10);
  const [constituicao, setConstituicao] = useState<number>(10);
  const [inteligencia, setInteligencia] = useState<number>(10);
  const [sabedoria, setSabedoria] = useState<number>(10);
  const [carisma, setCarisma] = useState<number>(10);

  const [classeDeArmadura, setClasseDeArmadura] = useState<number>(10);


  useEffect(() => {
    const fetchOrigens = async () => {
      try {
        const res = await fetch("https://j1p43lfm-8000.brs.devtunnels.ms/");
        if (!res.ok) {
          throw new Error("Erro ao carregar os dados");
        }
        const data = await res.json();
        setOrigens(data);
      } catch (error) {
        console.error(error);
        alert("Erro ao carregar os dados. Verifique o console para mais detalhes.");
      }
    };

    fetchOrigens();
  }, []);

  return(
    <div className="w-full mt-8 pr-4">
      <Card className="flex items-center justify-center md:w-[95%] w-full">
        <h4>Criar Ficha</h4>
        <CardContent className="w-full p-1">
          <Card className="mb-4">
            <CardHeader className="flex items-center justify-center">
              <h4>Informações Básicas</h4>
              {exibirBasico ? (
                <ArrowUp onClick={() => {setExibirBasico(!exibirBasico)}}/>
              ) : (
                <ArrowDown onClick={() => {setExibirBasico(!exibirBasico)}}/>
              )}
            </CardHeader>
            {exibirBasico && (
                <CardContent className="p-0 grid grid-cols-1 md:grid-cols-3  w-full gap-4">
                <div className="w-full mr-10">
                <Label className="mb-2">Nome do Jogador</Label>
                <Input placeholder="Nome do Jogador"/>
                </div>
                <div className="w-full">
                <Label className="mb-2">Nome do Personagem</Label>
                <Input placeholder="Nome do Personagem"/>
                </div>
                <div className="w-full">
                <Label className="mb-2">Nível do Personagem</Label>
                <Input placeholder="Nível do Personagem" type="number" max={20}/>
                </div>
              </CardContent>
            )}
          </Card>


          {exibirOrigem && (
                <Card>
                  <CardContent className="flex items-center justify-center flex-col">
                <Label>Origem</Label>
                <Select onValueChange={(e) => {setOrigem(origens.find((o) => o.nome === e))}}>
                  <SelectTrigger className="w-auto mt-2 bg-[var(--red-1)]">
                    <SelectValue placeholder="Selecione uma origem" />
                  </SelectTrigger>
                  <SelectContent className="bg-[var(--primary)]">
                    <SelectItem value="Selecione">Selecione uma origem</SelectItem>
                    {origens.map((origem) => (
                        <SelectItem key={origem.nome} value={origem.nome}>{origem.nome}</SelectItem>
                     ))}
                  </SelectContent>
                </Select>
              </CardContent>
                </Card>
            )}

          {origem && (
          <InformacaoOrigem origem={origem}/>
          )}


          <Card className="mt-4">
            <CardHeader className="flex items-center justify-center">
              <h4>Atributos</h4>
              {exibirAtributos ? (
                <ArrowUp onClick={() => {setExibirAtributos(!exibirAtributos)}}/>
              ) : (
                <ArrowDown onClick={() => {setExibirAtributos(!exibirAtributos)}} />
              )}
            </CardHeader>
            {exibirAtributos && (
                <CardContent className="grid grid-cols-2 md:grid-cols-3 gap-4">
                <div>
                <Label>Força</Label>
                <Input type="number" max={30} min={8} onChange={(e) => {setForca(parseInt(e.target.value))}} value={forca}/>
                <span>Modificador: {verifica_modificador(forca)}</span>
                </div>
                <div>
                <Label>Destreza</Label>
                <Input type="number" max={30} min={8} onChange={(e) => {setDestreza(parseInt(e.target.value))}} value={destreza}/>
                <span>Modificador: {verifica_modificador(destreza)}</span>
                </div>
                <div>
                <Label>Constituição</Label>
                <Input type="number"  max={30} min={8} onChange={(e) => {setConstituicao(parseInt(e.target.value))}} value={constituicao}/>
                <span>Modificador: {verifica_modificador(constituicao)}</span>
                </div>
                <div className="flex flex-col">
                <Label>Inteligência</Label>
                <Input type="number" max={30} min={8} onChange={(e) => {setInteligencia(parseInt(e.target.value))}} value={inteligencia}/>
                <p>Modificador: {verifica_modificador(inteligencia)}</p>
                </div>
                <div>
                <Label>Sabedoria</Label>
                <Input type="number"  max={30} min={8} onChange={(e) => {setSabedoria(parseInt(e.target.value))}} value={sabedoria}/>
                <span>Modificador: {verifica_modificador(sabedoria)}</span>
                </div>
                <div>
                <Label>Carisma</Label>
                <Input type="number" max={30} min={8} onChange={(e) => {setCarisma(parseInt(e.target.value))}} value={carisma}/>
                <span>Modificador: {verifica_modificador(carisma)}</span>
                </div>
                <div>
                <Label>Classe de Armadura</Label>
                <Input type="number" max={100} min={8} onChange={(e) => {setClasseDeArmadura(parseInt(e.target.value))}} value={classeDeArmadura}/>
                </div>
              </CardContent>
            )}      
          </Card>

        </CardContent>
      </Card>
    </div>
  )
}