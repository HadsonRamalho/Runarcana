import { Card, CardContent, CardHeader } from "@/components/ui/card"
import { Label } from "@/components/ui/label"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { AtributosClasse, HabilidadeClasse, InformacoesConjuracao, Subclasse, VariacaoHabilidadeClasse } from "@/interfaces/classe"
import { Eye } from "lucide-react"
import { useEffect, useState } from "react"
import { InformacaoGlossario } from "../origens"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"

interface CardInfoClasseProps{
    classe: AtributosClasse
}

interface InfoHabilidadeProps{
  habilidade: HabilidadeClasse;
}

interface InfoConjuracaoProps{
  info: InformacoesConjuracao;
}

const CardInfoConjuracao = ({info}: InfoConjuracaoProps) => {
  const [showInfo, setShowInfo] = useState(false);
  return(
    <Card className="w-full">
      <CardHeader className="flex justify-center items-center gap-2">
        <h4>Habilidade de Conjuração: <strong>{info.habilidade_conjuracao}</strong>
        </h4>
        <h4>
        Magias no Nível 1: <strong>{info.qtd_magias_inicial}</strong>
        </h4>
        <Eye onClick={() => {setShowInfo(!showInfo)}}/>

      </CardHeader>
      {showInfo && (
        <CardContent>
        {info.qtd_truques}
      </CardContent>
      )}
    </Card>
  )
}

interface InfoVariacaoHabilidadeProps{
  habilidade: VariacaoHabilidadeClasse;
}

const CardInfoVariacaoHabilidade = ({habilidade}: InfoVariacaoHabilidadeProps) => {
  const [showInfo, setShowInfo] = useState(false);
  return(
    <Card className="w-full">
      <CardHeader className="flex justify-center items-center gap-2">
        <h4>{habilidade.nome}
        </h4>
        <Eye onClick={() => {setShowInfo(!showInfo)}}/>

      </CardHeader>
      {showInfo && (
        <CardContent>
        {habilidade.descricao}
      </CardContent>
      )}
    </Card>
  )
}

const CardInfoHabilidade = ({habilidade}: InfoHabilidadeProps) => {
  const [showInfo, setShowInfo] = useState(false);
  return(
    <Card className="w-full">
      <CardHeader className="flex justify-center items-center gap-2">
        <h4>{habilidade.nome}
        </h4>
        <Eye onClick={() => {setShowInfo(!showInfo)}}/>

      </CardHeader>
      {showInfo && (
        <CardContent>
        {habilidade.descricao}
        {habilidade.variacoes && (
          habilidade.variacoes.map((variacao) => (
            <div className="m-4">
              <CardInfoVariacaoHabilidade habilidade={variacao}/>
            </div>
          ))
        )}
      </CardContent>
      )}
    </Card>
  )
}

interface InfoSubclasseProps{
  subclasse: Subclasse;
}

const CardInfoSubclasse = ({subclasse}: InfoSubclasseProps) => {
  const [showInfo, setShowInfo] = useState(false);
  const [skill, setSkill] = useState<HabilidadeClasse>();
  return(
    <Card className="m-4 p-0">
      <CardHeader className="flex justify-center items-center gap-2">
        <h4>{subclasse.nome}
        </h4>
        <Eye onClick={() => {setShowInfo(!showInfo)}}/>

      </CardHeader>
      {showInfo && (
        <CardContent className="p-0 m-4">
          <div className="p-0 m-0">
          <InformacaoGlossario dados={{nome: "Descrição", descricao: subclasse.descricao}}/>
          </div>
          <div>
          
          <Card className="mt-4 mb-4 flex items-center justify-center p-4">
          <Label><h4>Lista de Habilidades</h4></Label>
          <Select onValueChange={(e) => {setSkill(subclasse.habilidades?.find((o) => o.nome === e))}}>
           <SelectTrigger className="w-auto mt-2 bg-[var(--red-1)]">
            <SelectValue placeholder="Selecione uma habilidade" />
          </SelectTrigger>
          <SelectContent className="bg-[var(--primary)]">
          <SelectItem value="Selecione">Selecione uma habilidade</SelectItem>
          {subclasse.habilidades?.map((habilidade) => (
            <SelectItem key={habilidade.nome} value={habilidade.nome}>{habilidade.nome}</SelectItem>
          ))}
          </SelectContent>
          </Select>
          {skill && (
            <CardInfoHabilidade habilidade={skill}/>
          )}
          </Card>
          
          <div className="grid grid-cols-1 gap-4 mt-4">
          {skill && (
            skill.caracteristicas?.map((habilidade) => (
              <CardInfoHabilidade habilidade={habilidade}/>
            ))
          )}
          
          {subclasse.informacoes_conjuracao && (
            <CardInfoConjuracao info={subclasse.informacoes_conjuracao}/>
          )}
          </div>
          </div>
        </CardContent>
      )}
    </Card>
  )
}

const CardInformacoesClasse = ({classe}: CardInfoClasseProps) => {
  const [showInfo, setShowInfo] = useState(false);
    return(
    <Card className="w-full">
    <CardHeader className="flex items-center justify-center">
    <h4>{classe.nome}</h4>
    <Eye onClick={() => {setShowInfo(!showInfo)}}/>
    </CardHeader>
    {showInfo && (
      <Tabs defaultValue="infobasica" className="w-full">
        <TabsList className="w-full md:flex grid grid-cols-1 h-auto">
          <TabsTrigger value="infobasica" className="bg-emerald-100">Informações Básicas</TabsTrigger>
          <TabsTrigger value="caracteristicas" className="bg-emerald-100">Caracteristicas</TabsTrigger>
          <TabsTrigger value="subclasses" className="bg-emerald-100">Subclasses</TabsTrigger>
        </TabsList>
        <TabsContent value="infobasica" className="pr-4">
          <Card className="m-2 p-2 gap-2 w-full h-max">
            <Label><strong>Dados de Vida:</strong> {classe.dado_vida}</Label>
            <Label><strong>Pontos de Vida no 1° Nível:</strong> {classe.dado_vida} + seu modificador de {classe.modificador_pontos_vida}</Label>
            <Label><strong>Pontos de Vida nos Níveis Seguintes:</strong> {classe.dados_vida_nivel}</Label>
          </Card>
          <div  className="grid grid-cols-1 md:grid-cols-3 gap-2 md:gap-4 md:pr-4">
            
          <Card className="w-max m-2 p-2 gap-2 w-full">
          <Label className="mb-0">Proficiências em Armaduras:</Label>
          {classe.proficiencias_armaduras?.map((proficiencia) => (
            <li className="mt-0 pt-0 ml-4">
              {proficiencia}
            </li>
          ))}
          </Card>
          <Card className="w-max m-2 p-2 gap-2 w-full">
            <Label className="mb-0">Proficiências em Armas: </Label>
            {classe.proficiencias_armas?.map((proficiencia) => (
              <li className="mt-0 pt-0 ml-4">
                {proficiencia}
              </li>
            ))}
          </Card>
          <Card className="w-max m-2 p-2 gap-2 w-full">
            <Label className="mb-0">Proficiências em Ofícios: </Label>
            {classe.proficiencias_oficios?.map((proficiencia) => (
              <li className="mt-0 pt-0 ml-4">
                {proficiencia}
              </li>
            ))}
          </Card>
          <Card className="w-max m-2 p-2 gap-2 w-full">
            <Label className="mb-0">Proficiências em Salvaguardas: </Label>
            {classe.proficiencias_salvaguardas?.map((proficiencia) => (
              <li className="mt-0 pt-0 ml-4">
                {proficiencia}
              </li>
            ))}
          </Card>
          {classe.proficiencias_pericias && (
            <Card className="w-max m-2 p-2 gap-2 w-full">
            <Label className="mb-0">Proficiências em Perícias: </Label>
            {classe.proficiencias_pericias?.map((proficiencia) => (
              <li className="mt-0 pt-0 ml-4">
                {proficiencia}
              </li>
            ))}
          </Card>
          )}
          <Card className="w-max m-2 p-2 gap-2 w-full">
            <Label className="mb-0">Opções de Perícias: </Label>
            {classe.opcoes_pericias?.map((opcao) => (
              <li className="mt-0 pt-0 ml-4">
                {opcao}
              </li>
            ))}
          </Card>
          </div>
        </TabsContent>
        <TabsContent value="caracteristicas">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4 m-2">
          {classe.habilidades?.map((habilidade) => (
            <CardInfoHabilidade habilidade={habilidade} />
          ))}
          </div>
        </TabsContent>
        <TabsContent value="subclasses">
          <div className="p-4">
            <Card className="flex items-center justify-center p-4 pr-4">
              <h4>
              {classe.titulo_subclasse}
              </h4>
              {classe.descricao_subclasse}
            </Card>
          </div>
          <div>
            {classe.lista_subclasses?.map((subclasse) => (
              <CardInfoSubclasse subclasse={subclasse}/>
            ))}
          </div>
        </TabsContent>
      </Tabs>

    )} 
    </Card>
  )

}

export const Classes = () => {
  const [classes, setClasses] = useState<AtributosClasse[] | undefined>();
  useEffect(() => {
    const fetchOrigens = async () => {
      try {
        const res = await fetch("http://localhost:3060/classes");
        if (!res.ok) {
          throw new Error("Erro ao carregar os dados");
        }
        const data = await res.json();
        setClasses(data);
      } catch (error) {
        console.error(error);
        alert("Erro ao carregar os dados. Verifique o console para mais detalhes.");
      }
    };

    fetchOrigens();
  }, []);

  return(
    <div className="w-full">
    {classes && (
    <div className="w-full">
     {classes.map((classe) => (
    <div key={classe.nome} className="w-full pr-4 pt-2 pb-2">
      <CardInformacoesClasse classe={classe}/>
    </div>
     ))}
    </div>
    )}   
    </div>
  )
}