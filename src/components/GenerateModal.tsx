import { useRef } from 'react'
import Button from './Button'
import "./Modal.css";
import Range from './Range';
import { AppState, OpenModal, useStore } from '../store';
import NumberInput from './NumberInput';
import { generate } from '../Interface';

const GenerateModal = () => {
  const rangeRef = useRef<any>(null);
  const inputRef = useRef<any>(null);
  const changeOpenModal = useStore(state => state.changeOpenModal);
  
  const appState = useStore(state => state.appState);
  const changeAppState = useStore(state => state.changeAppState);
  
  const onError = useStore(state => state.changeMessage);
  const includeCounts = useStore(state => state.appState) == AppState.Editing; 
  
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
                onClick={() => {
                  generate(
                    rangeRef.current.getValue(),
                    inputRef.current.getValue(),
                    includeCounts, includeCounts, !includeCounts,
                    () => {
                      changeOpenModal(OpenModal.None);
                      if (appState != AppState.Editing) {
                        changeAppState(AppState.Solving);
                      }
                    },
                    onError,
                  )
                }}
            />
        </div>
    </div>
  )
}

export default GenerateModal
