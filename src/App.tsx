import { useState } from "react";
import "./App.css";
import MainFrame from "./components/MainFrame";
import LoadingModal from "./components/LoadingModal";
import { BaseDirectory, readDir } from "@tauri-apps/api/fs";
import GenerateModal from "./components/GenerateModal";
import { useStore } from "./store";
import SaveModal from "./components/SaveModal";

const getEntries = async () => {
  const entries = await readDir("savegames", { dir: BaseDirectory.AppData, recursive: false });
  return entries;
}

function App() {
  const [openLoadingModal, setOpenLoadingModal] = useState(false);
  const [openGenerateModal, setOpenGenerateModal] = useState(false);
  const [openSaveModal, setOpenSaveModal] = useState(false);
  const setControlsEnabled = useStore(state => state.setControlsEnabled);

  const loadingModal = (open:boolean) => {
    setOpenLoadingModal(open);
    setControlsEnabled(!open);
  }

  const generateModal = (open:boolean) => {
    setOpenGenerateModal(open);
    setControlsEnabled(!open);
  }

  const saveModal = (open:boolean) => {
    setOpenSaveModal(open);
    setControlsEnabled(!open);
  }

  return (
    <div className="container">
      <MainFrame 
        setOpenLoadingModal={loadingModal} 
        setOpenGenerateModal={generateModal}
        setOpenSaveModal={saveModal}
      />
      {
        openLoadingModal && <LoadingModal 
          setOpen={loadingModal} 
          promise={getEntries()}
        />
      }
      {
        openGenerateModal && <GenerateModal 
          setOpen={generateModal}
        />
      }
      {
        openSaveModal && <SaveModal 
          setOpen={saveModal}
          promise={getEntries()}
        />
      }
    </div>
  );
}

export default App;
