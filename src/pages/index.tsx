import { useEffect } from "react";

function App() {
  useEffect(() => {
    document.body.classList.add("arrow");
  }, []);
  return (
    <div style={{ height: 100 }}>
    </div>
  );
}

export default App;
