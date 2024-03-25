import { useRef } from 'react'
import Checkbox from './Checkbox'
import Button from './Button'
import { invoke } from "@tauri-apps/api/tauri";
import Timer from './Timer';
import Range from './Range';
import { useStore } from '../store';
import { BaseDirectory, createDir, writeTextFile, exists } from "@tauri-apps/api/fs";
import { join, appDataDir } from '@tauri-apps/api/path';

interface Props {
    updatePlayfield: () => void;
    setOpenModal: (open:boolean) => void;
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

const Sidebar = ({updatePlayfield, setOpenModal} : Props) => {
    const showErrorsRef = useRef<any>(null);
    const timerRef = useRef<any>(null);
    const rangeRef = useRef<any>(null);
    const onError = useStore(state => state.changeMessage);
    
    return (
        <div id='sidebar'>
            <Checkbox 
                name='show errors'
                ref={showErrorsRef}
                onStateToggle={() => invoke<boolean>('toggle_show_errors')
                .then((state) => {
                    showErrorsRef.current.setActive(state);
                    updatePlayfield();
                })
                .catch(onError)}
            />
            <Range min={20} max={57} ref={rangeRef}/>
            <Button
                name='generate'
                onClick={() => invoke('generate', {difficulty:rangeRef.current.getValue()}).then((_) => {
                    updatePlayfield();
                    timerRef.current.start();
                })
                .catch(onError)}
            />
            <Button
                name='solve'
                onClick={() => invoke('solve').then((_) => updatePlayfield())
                .catch(onError)}
            />
            <Button
                name='reset'
                onClick={() => invoke('reset').then((_) => {
                    updatePlayfield();
                    timerRef.current.start();
                })
                .catch(onError)}
            />
            <Button
                name='load'
                onClick={() => setOpenModal(true)}
            />
            <Button
                name='save'
                onClick={() => invoke<string>('serialize')
                    .then((content: string) => createDataFile(new Date().getTime().toString(), content))
                }
            />
            <Timer ref={timerRef}/>
        </div>
    )
}

export default Sidebar
