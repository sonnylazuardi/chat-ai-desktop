import React from "react";

function App() {
  React.useEffect(() => {
    window.location.href = "https://chat.openai.com/chat";
  }, [])
  return (
    <div className="container"></div>
  );
}

export default App;
