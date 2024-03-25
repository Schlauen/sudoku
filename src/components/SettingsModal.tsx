import React, { useState } from 'react'
import Button from './Button'
import "./Modal.css";
import { BaseDirectory, createDir, writeFile, readTextFile, readDir, FileEntry } from "@tauri-apps/api/fs";
import { join, appDataDir } from '@tauri-apps/api/path';import { invoke } from '@tauri-apps/api';

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
  setOpenModal: (open:boolean) => void;
  updateMainFrame: () => void;
}

const SettingsModal = ({setOpenModal, promise, updateMainFrame} : Props) => {
  const [items, setItems] = useState<FileEntry[]>([]);

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
                      console.log(content);
                      invoke('deserialize', {msg:content}).then(state => console.log(state))
                      updateMainFrame();
                    });
                    setOpenModal(false);
                  }}
                />
              ))
            }
        </div>
    </div>
  )
}

export default SettingsModal
