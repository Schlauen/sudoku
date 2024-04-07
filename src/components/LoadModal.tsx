import { useState } from 'react'
import Button from './Button'
import "./Modal.css";
import { readTextFile, FileEntry } from "@tauri-apps/api/fs";
import { AppState, OpenModal, useStore } from '../store';
import { GameState, deserialize } from '../Interface';

const readDataFile = async (path:string, onError:(error:any) => void) => {
  try {
    return await readTextFile(path);
  } catch (e) {
    onError(e);
  }
  return "";
};

interface Props {
  promise: Promise<FileEntry[]>;
}

const LoadingModal = ({promise} : Props) => {
  const [items, setItems] = useState<FileEntry[]>([]);
  const changeOpenModal = useStore(state => state.changeOpenModal);
  const onError = useStore(state => state.changeMessage);
  const changeAppState = useStore(state => state.changeAppState);
  
  promise.then(i => setItems(i));
  const includeCounts = useStore(state => state.appState) == AppState.Editing;

  const onSuccess = (state:number) => {
    if (state == GameState.Blank || state == GameState.Editing) {
      changeAppState(AppState.Editing);
    }
    else if (state == GameState.Running) {
      changeAppState(AppState.Solving);
    }
    else if (state == GameState.Solved) {
      changeAppState(AppState.Solved);
    }
    else if (state == GameState.Error) {
      changeAppState(AppState.Editing);
    }
  }

  return (
    <div className='modal-background'>
        <div className='modal-container'>
            <div className='title'>
                <h1>Choose file</h1>
            </div>
            {
              items.map(i => (
                <Button
                  name={i.name || ""}
                  onClick={() => {
                    readDataFile(i.path, onError)
                      .then(content => deserialize(content, includeCounts, includeCounts, onSuccess, onError))
                      .catch(onError);
                      
                    changeOpenModal(OpenModal.None);
                  }}
                />
              ))
            }
        </div>
    </div>
  )
}

export default LoadingModal
