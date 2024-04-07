import Checkbox from './Checkbox'
import Button from './Button'
import { AppState, OpenModal, useStore } from '../store';
import Timer from './Timer';
import { GameState, hint, onUpdateGame, reset } from '../Interface';
import { useEffect } from 'react';

const SolveSidebar = () => {
    const changeOpenModal = useStore(state => state.changeOpenModal);
    const changeAppState = useStore(state => state.changeAppState);
    const setShowError = useStore(state => state.setShowError);
    const setMessage = useStore(state => state.changeMessage);

    useEffect(() => {
        const unlisten = onUpdateGame(event => {
            if (event.state == GameState.Solved) {
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
                onClick={() => hint(false, false, setMessage)}
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
