import { useRef, useState } from "react";
import "./App.css";
import MainFrame from "./components/MainFrame";
import SettingsModal from "./components/SettingsModal";
import { BaseDirectory, createDir, writeFile, readTextFile, readDir } from "@tauri-apps/api/fs";
import { join, appDataDir } from '@tauri-apps/api/path';7

const getEntries = async () => {
  const entries = await readDir("savegames", { dir: BaseDirectory.AppData, recursive: false });
  return entries;
}

function App() {
  const [openModal, setOpenModal] = useState(false);
  const mainFrame = useRef<any>(null);
  const update = () => mainFrame.current.update();

  return (
    <div className="container">
      <MainFrame setOpenModal={setOpenModal} ref={mainFrame}/>
      {
        openModal && <SettingsModal 
          setOpenModal={setOpenModal} 
          promise={getEntries()}
          updateMainFrame={update}
        />
      }
    </div>
  );
}

export default App;
