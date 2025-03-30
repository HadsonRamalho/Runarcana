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
import { converter_pes_metros, InformacaoGlossario, Origem } from "../origens"
  
const origens = [
    "Humano",
    "Antroplantae",
];

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
        const res = await fetch("http://localhost:8000/");
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
    <div className="w-full mt-8">
      <Card className="flex items-center justify-center w-[95%]">
        <h4>Criar Ficha</h4>
        <CardContent className="w-full">
          <Card>
            <CardHeader className="flex items-center justify-center">
              <h4>Informações Básicas</h4>
            </CardHeader>
            <CardContent className="flex items-center justify-center">
              <div className="w-[40%] mr-10">
              <Label>Nome do Jogador</Label>
              <Input placeholder="Nome do Jogador"/>
              </div>
              <div className="w-[40%]">
              <Label>Nome do Personagem</Label>
              <Input placeholder="Nome do Personagem"/>
              </div>
            </CardContent>
          </Card>

          <Card className="mt-4">
            <CardHeader className="flex items-center justify-center">
              <h4>Informações da Origem</h4>
            </CardHeader>
            <CardContent className="flex items-center justify-center flex-col">
              <Label>Origem</Label>
              <Select onValueChange={(e) => {setOrigem(origens.find((o) => o.nome === e))}}>
                <SelectTrigger className="w-auto mt-2 bg-emerald-200">
                  <SelectValue placeholder="Selecione uma origem" />
                </SelectTrigger>
                <SelectContent className="bg-emerald-400">
                  <SelectItem value="Selecione">Selecione uma origem</SelectItem>
                  {origens.map((origem) => (
                      <SelectItem key={origem.nome} value={origem.nome}>{origem.nome}</SelectItem>
                   ))}
                </SelectContent>
              </Select>
            </CardContent>
            {origem && (
              <CardContent>
                <div>
                  <Label>Resumo</Label>
                  <div className="w-full border-1 border-black rounded-md p-1 mt-2">{origem.resumo}</div>
                </div>
                <CardContent className="grid grid-cols-3 mt-4 gap-4">
                  <div>
                    <Label className="mb-2">Heranças: </Label>
                    <InformacaoGlossario dados={{nome: `Quantidade: ${origem.quantidade_herancas}`, descricao: origem.descricoes_origem.heranca}}/>
                  </div>
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
                    <Label>Idiomas: </Label>
                    <InformacaoGlossario dados={{nome: `Comum e ${origem.quantidade_idiomas} idioma adicional`, descricao: ` ${origem.idiomas + '\n' + origem.descricoes_origem.idiomas + '\n'} `}}/>
                  </div>
                  <div>
                    <Label className="mb-2">Perícias:</Label>
                    <InformacaoGlossario dados={{nome: `${origem.pericias?.length} perícias`, descricao: `${origem.descricoes_origem.pericia}`}} />
                  </div>
                </CardContent>
              </CardContent>
            )}
          </Card>

          <Card className="mt-4">
            <CardHeader className="flex items-center justify-center">
              <h4>Atributos</h4>
            </CardHeader>
            <CardContent className="grid grid-cols-3 gap-4">
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
          </Card>

        </CardContent>
      </Card>
    </div>
  )
}