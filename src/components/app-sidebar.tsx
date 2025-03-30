import { Calendar, Home, HomeIcon, Inbox, NotebookIcon, Search, Settings, User2Icon } from "lucide-react"

import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar"
import { useNavigate } from "react-router-dom"
import { RacasIcon } from "./icons/racas";

const items = [
  {
    title: "Página Inicial",
    url: "/",
    icon: <HomeIcon/>
  },
  {
    title: "Criar Ficha",
    url: "/criar_ficha",
    icon: <NotebookIcon />
  },
  {
    title: "Origens",
    url: "/origens",
    icon: <RacasIcon />
  },
  {
    title: "Configurações",
    url: "/",
    icon: <Settings/>
  },
]

export function AppSidebar() {
  return (
    <Sidebar>
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupLabel>Runarcana</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              {items.map((item) => (
                <SidebarMenuItem key={item.title}>
                  <SidebarMenuButton asChild className="hover:cursor-pointer">
                    <a href={item.url}>
                      {item.icon}
                      <span>{item.title}</span>
                    </a>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
    </Sidebar>
  )
}
export default AppSidebar;