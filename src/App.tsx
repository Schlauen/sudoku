import { useState } from "react";
import "./App.css";
import MainFrame from "./components/MainFrame";

function App() {
  const [left, setLeft] = useState(0);
  const [top, setTop] = useState(0);

  // onKeyDown handler function
  const keyDownHandler = (event: React.KeyboardEvent<HTMLDivElement>) => {
    console.log(left, top);
    if (event.code === "ArrowUp") {
      setTop((top) => top - 10);
    }

    if (event.code === "ArrowDown") {
      setTop((top) => top + 10);
    }

    if (event.code === "ArrowLeft") {
      setLeft((left) => left - 10);
    }

    if (event.code === "ArrowRight") {
      setLeft((left) => left + 10);
    }
  };

  return (
    <div className="container" onKeyDown={keyDownHandler}>
      <MainFrame />
    </div>
  );
}

export default App;
