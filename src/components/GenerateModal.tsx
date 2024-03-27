import { useRef } from 'react'
import Button from './Button'
import "./Modal.css";
import { invoke } from '@tauri-apps/api';
import Range from './Range';
import { useStore } from '../store';
import NumberInput from './NumberInput';

interface Props {
  setOpen: (open:boolean) => void;
}

const GenerateModal = ({setOpen: setOpen} : Props) => {
  const rangeRef = useRef<any>(null);
  const inputRef = useRef<any>(null);
  const onError = useStore(state => state.changeMessage);
  const updatePlayfield = useStore(state => state.updatePlayfield);

  return (
    <div className='modal-background'>
        <div className='modal-container'>
            <div className='title'>
                <h1>configuration</h1>
            </div>
            <Range min={20} max={57} ref={rangeRef}/>
            <NumberInput name="seed" ref={inputRef}/>
            <Button
                name='generate'
                onClick={() => invoke('generate', {
                  difficulty:rangeRef.current.getValue(),
                  seed:inputRef.current.getValue(),
                }).then((_) => {
                    updatePlayfield();
                    setOpen(false);
                })
                .catch(onError)}
            />
        </div>
    </div>
  )
}

export default GenerateModal
