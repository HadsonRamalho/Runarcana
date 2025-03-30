import { Card, CardContent } from "@/components/ui/card";
import Layout from "@/layouts/default";

function App() {
  return (
    <>
      <Layout>
        <main className="flex items-center justify-center w-full">
          <Card className="md:w-1/3 md:h-1/2">
            <CardContent className="flex items-center justify-center">
              coisa e tal
            </CardContent>
          </Card>
        </main>
      </Layout>
    </>
  );
}

export default App;
