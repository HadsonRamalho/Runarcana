import About from "@/pages/about";
import Logged from "@/pages/logged";
import Home from "@/pages/home";
import { createBrowserRouter } from "react-router-dom";
import PrivateRoute from "./privateRoute";
import { Prohibited } from "@/pages/prohibited";
import { Origens } from "@/pages/origens";
import { CriarFicha } from "@/pages/criar_ficha";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Home />,
    errorElement: <h1>404</h1>,
  },
  {
    path: "/sobre",
    element: <About />,
  },
  {
    path: "/origens",
    element: <Origens />,
  },
  {
    path: "/criar_ficha",
    element: <CriarFicha />,
  },
  {
    path: "/logado",
    element: (
      <PrivateRoute>
        <Logged />,
      </PrivateRoute>
    ),
  },

  {
    path: "/entrada-proibida",
    element: <Prohibited />,
  },
]);

export default router;
