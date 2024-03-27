import { useRef, useState } from 'react'
import Button from './Button'
import "./Modal.css";
import { readTextFile, FileEntry } from "@tauri-apps/api/fs";
import { invoke } from '@tauri-apps/api';
import { useStore } from '../store';
import { BaseDirectory, createDir, writeTextFile, exists } from "@tauri-apps/api/fs";
import { join } from '@tauri-apps/api/path';
import Input from './NumberInput';
import TextInput from './TextInput';

interface Props {
  promise: Promise<FileEntry[]>;
  setOpen: (open:boolean) => void;
}

const createDataFolder = async () => {
  const savegames = "savegames";
  const ex = await exists(savegames, {dir: BaseDirectory.AppData});
  if (ex) {
      return;
  }
  try {
      await createDir(savegames, {
          dir: BaseDirectory.AppData,
          recursive: true,
      });
  } catch (e) {
      console.error(e);
  }
};

const createDataFile = async (name:string, content:string) => {
  createDataFolder();
  const path = await join("savegames", name + '.json');
  try {
    await writeTextFile(
      {
        contents: content,
        path: path,
      },
      {
          dir: BaseDirectory.AppData
      },
    );
  } catch (e) {
    console.log(e);
  }
};

const SaveModal = ({setOpen: setOpen, promise} : Props) => {
  const [items, setItems] = useState<FileEntry[]>([]);
  const inputRef = useRef<any>(null);

  promise.then(i => setItems(i));
  
  return (
    <div className='modal-background'>
        <div className='modal-container'>
        <div className='title'>
            <h1>save game</h1>
            </div>
            <TextInput name="name" ref={inputRef}/>
            <Button
                name='save'
                onClick={() => {
                  console.log(inputRef);
                  const filename = inputRef.current.getValue();
                  invoke<string>('serialize')
                    .then((content: string) => createDataFile(filename, content))
                    .catch(e => console.log(e));
                  setOpen(false);
                }}
            />
            {
              items.map(i => (
                <div>{i.name || ""}</div>
              ))
            }
        </div>
    </div>
  )
}

export default SaveModal
