import { useState } from 'react'
import Button from './Button'
import "./Modal.css";
import { readTextFile, FileEntry } from "@tauri-apps/api/fs";
import { invoke } from '@tauri-apps/api';
import { useStore } from '../store';

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
  setOpen: (open:boolean) => void;
}

const LoadingModal = ({setOpen: setOpen, promise} : Props) => {
  const [items, setItems] = useState<FileEntry[]>([]);
  const updatePlayfield = useStore(state => state.updatePlayfield);

  promise.then(i => setItems(i));

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
                    readDataFile(i.path).then(content => {
                      invoke('deserialize', {msg:content}).then(state => console.log(state))
                      updatePlayfield();
                    });
                    setOpen(false);
                  }}
                />
              ))
            }
        </div>
    </div>
  )
}

export default LoadingModal
