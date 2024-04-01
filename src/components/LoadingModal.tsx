import { useState } from 'react'
import Button from './Button'
import "./Modal.css";
import { readTextFile, FileEntry } from "@tauri-apps/api/fs";
import { AppState, OpenModal, useStore } from '../store';
import { deserialize } from '../Interface';

const readDataFile = async (path:string) => {
  try {
    return await readTextFile(path);
  } catch (e) {
    console.log(e);
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

  const appState = useStore(state => state.appState);
  const changeAppState = useStore(state => state.changeAppState);
  
  promise.then(i => setItems(i));
  const includeCounts = useStore(state => state.appState) == AppState.Editing;

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
                    readDataFile(i.path)
                      .then(content => deserialize(content, includeCounts, includeCounts, onError))
                      .catch(onError);
                      
                    changeOpenModal(OpenModal.None);
                    if (appState != AppState.Editing) {
                      changeAppState(AppState.Solving);
                    }
                  }}
                />
              ))
            }
        </div>
    </div>
  )
}

export default LoadingModal
