import AppSidebar from "@/components/app-sidebar";
import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar";
import AuthProvider from "@/hooks/auth";

export default function Providers({ children }: { children: React.ReactNode }) {
  return <AuthProvider>
    <SidebarProvider className="bg-gradient-to-b from-emerald-300 to-blue-200">
    <AppSidebar />
    <SidebarTrigger />
    {children}
    </SidebarProvider>
  </AuthProvider>;
}
