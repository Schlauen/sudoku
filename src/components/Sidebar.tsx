import { useRef } from 'react'
import Checkbox from './Checkbox'
import Button from './Button'
import { invoke } from "@tauri-apps/api/tauri";
import { useStore } from '../store';

interface Props {
    setOpenLoadingModal: (open:boolean) => void;
    setOpenGenerateModal: (open:boolean) => void;
    setOpenSaveModal: (open:boolean) => void;
}

const Sidebar = ({
    setOpenLoadingModal: setOpenLoadingModal,
    setOpenGenerateModal: setOpenGenerateModal,
    setOpenSaveModal: setOpenSaveModal,
} : Props) => {
    const showErrorsRef = useRef<any>(null);
    const onError = useStore(state => state.changeMessage);
    const updatePlayfield = useStore(state => state.updatePlayfield);

    return (
        <div id='sidebar'>
            <Button
                name='generate'
                onClick={() => setOpenGenerateModal(true)}
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
                })
                .catch(onError)}
            />
            <Button
                name='load'
                onClick={() => setOpenLoadingModal(true)}
            />
            <Button
                name='save'
                onClick={() => setOpenSaveModal(true)}
            />
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
        </div>
    )
}

export default Sidebar
