import Checkbox from './Checkbox'
import Button from './Button'
import { AppState, OpenModal, useStore } from '../store';
import Timer from './Timer';
import { GameState, GameUpdateEvent, reset } from '../Interface';
import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';

const SolveSidebar = () => {
    const changeOpenModal = useStore(state => state.changeOpenModal);
    const changeAppState = useStore(state => state.changeAppState);
    const setShowError = useStore(state => state.setShowError);
    const setMessage = useStore(state => state.changeMessage);

    useEffect(() => {
        const unlisten = listen<GameUpdateEvent>('updateGame', event => {
            if (event.payload.state == GameState.Solved) {
                changeAppState(AppState.Solved);
                setMessage('solved!')
            }
        });

        return () => {unlisten.then(f => f())};
    });

    return (
        <div id='sidebar'>
            <Button
                name='hint'
                onClick={() => console.log('not implemented')}
            />
            <Button
                name='reset'
                onClick={() => reset(false, false, false, setMessage)}
            />
            <Button
                name='save'
                onClick={() => changeOpenModal(OpenModal.SaveModal)}
            />
            <Button
                name='back'
                onClick={() => {
                    changeAppState(AppState.Start);
                    setMessage('do you want to play or create a new game?');
                    reset(false, false, true, setMessage);
                }}
            />
            <Checkbox 
                name='show errors'
                onStateToggle={setShowError}
            />
            <Timer/>
        </div>
    )
}

export default SolveSidebar
