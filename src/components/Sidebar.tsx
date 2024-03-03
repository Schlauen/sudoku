import { useRef } from 'react'
import Checkbox from './Checkbox'
import Button from './Button'
import { invoke } from "@tauri-apps/api/tauri";
import Timer from './Timer';
import Range from './Range';


interface Props {
    updatePlayfield: () => void;
    onError: (message:string) => void;
}

const Sidebar = ({updatePlayfield, onError} : Props) => {
    const showErrorsRef = useRef<any>(null);
    const timerRef = useRef<any>(null);
    const rangeRef = useRef<any>(null);
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
                    timerRef.current.stop();
                    updatePlayfield();
                    timerRef.current.start();
                })
                .catch(onError)}
            />
            <Button
                name='solve'
                onClick={() => invoke('solve').then((_) => {
                    updatePlayfield();
                    timerRef.current.stop();
                })
                .catch(onError)}
            />
            <Button
                name='reset'
                onClick={() => invoke('reset').then((_) => {
                    updatePlayfield();
                    timerRef.current.reset();
                })
                .catch(onError)}
            />
            <Timer ref={timerRef}/>
        </div>
    )
}

export default Sidebar
