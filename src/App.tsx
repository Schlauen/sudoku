import "./App.css";
import MainFrame from "./components/MainFrame";
import LoadingModal from "./components/LoadingModal";
import { BaseDirectory, readDir } from "@tauri-apps/api/fs";
import GenerateModal from "./components/GenerateModal";
import { AppState, OpenModal, useStore } from "./store";
import SaveModal from "./components/SaveModal";

const getEntries = async () => {
  const entries = await readDir("savegames", { dir: BaseDirectory.AppData, recursive: false });
  return entries;
}

const renderModal = (openModal: number) => {
  {
    switch (openModal) {
      case OpenModal.SaveModal:
        return <SaveModal 
          promise={getEntries()}
        />
      case OpenModal.GenerateModal:
        return <GenerateModal/>
      case OpenModal.LoadModal:
        return <LoadingModal
          promise={getEntries()}
        />
    }
  }
}

function App() {
  const openModal = useStore(state => state.openModal);
  return (
    <div className="container">
      <MainFrame/>
      {
        renderModal(openModal)
      }
    </div>
  );
}

export default App;
