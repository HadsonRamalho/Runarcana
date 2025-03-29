import { Button } from "@/components/ui/button";
import Layout from "@/layouts/default";

function App() {
  return (
    <>
      <Layout>
        <main>
          <h1>Página inicial</h1>
          <Button className="bg-[hsl(var(--primary))]">Hello</Button>
        </main>
      </Layout>
    </>
  );
}

export default App;
