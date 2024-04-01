import { useRef, useState } from 'react'
import Button from './Button'
import "./Modal.css";
import { FileEntry } from "@tauri-apps/api/fs";
import { OpenModal, useStore } from '../store';
import { BaseDirectory, createDir, writeTextFile, exists } from "@tauri-apps/api/fs";
import { join } from '@tauri-apps/api/path';
import TextInput from './TextInput';
import { serialize } from '../Interface';

interface Props {
  promise: Promise<FileEntry[]>;
}

const createDataFolder = async () => {
  const savegames = "savegames";
  const ex = await exists(savegames, {dir: BaseDirectory.AppData});
  if (ex) {
      return;
  }
  createDir(savegames, {
    dir: BaseDirectory.AppData,
    recursive: true,
  });
};

const createDataFile = async (name:string, content:string) => {
  await createDataFolder();
  const path = await join("savegames", name + '.json');
  await writeTextFile(
    {
      contents: content,
      path: path,
    },
    {
        dir: BaseDirectory.AppData
    },
  );
};

const SaveModal = ({promise} : Props) => {
  const [items, setItems] = useState<FileEntry[]>([]);
  const inputRef = useRef<any>(null);
  const changeOpenModal = useStore(state => state.changeOpenModal);
  const onError = useStore(state => state.changeMessage)

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
                  serialize(content => createDataFile(filename, content).catch(onError), onError);
                  changeOpenModal(OpenModal.None);
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
