import { useEffect, useState } from 'react'
import Button from './Button'
import { AppState, OpenModal, useStore } from '../store';
import { fixResult, hint, onUpdateGame, reset, solve } from '../Interface';

const getSolutionCount = (count:number) => {
    if (count > 4) {
        return '> 4';
    }
    return count.toString();
}

const EditorSidebar = () => {
    const changeOpenModal = useStore(state => state.changeOpenModal);
    const changeAppState = useStore(state => state.changeAppState);
    const setMessage = useStore(state => state.changeMessage);
    
    const [solutionCount, setSolutionCount] = useState<number>(5);
    const [clueCount, setClueCount] = useState<number>(0);

    useEffect(() => {
        const unlisten = onUpdateGame(event => {
            setSolutionCount(event.solution_count);
            setClueCount(event.clue_count);
        });

        return () => {unlisten.then(f => f())};
    });

    return (
        <div id='sidebar'>
            <Button
                name='generate'
                onClick={() => changeOpenModal(OpenModal.GenerateModal)}
            />
            <Button
                name='solve'
                onClick={() => solve(true, true, setMessage)}
            />
            <Button
                name='reset'
                onClick={() => reset(true, true, true, setMessage)}
            />
            <Button
                name='hint'
                onClick={() => hint(true, true, setMessage)}
            />
            <Button
                name='load'
                onClick={() => changeOpenModal(OpenModal.LoadModal)}
            />
            <Button
                name='save'
                onClick={() => changeOpenModal(OpenModal.SaveModal)}
            />
            <div className='menu-element key-value'>
                <label>solutions:</label>
                <label>{getSolutionCount(solutionCount)}</label>
            </div>
            <div className='menu-element key-value'>
                <label>clues:</label>
                <label>{clueCount}</label>
            </div>
            <Button
                name='back'
                onClick={() => {
                    changeAppState(AppState.Start);
                    setMessage('enough editing? Wanna play?');
                    reset(false, false, true, setMessage);
                }}
            />
            {
                solutionCount == 1 && <Button
                    name='play'
                    onClick={() => {
                        setMessage("let's see if you can crack this Sudoku");
                        changeAppState(AppState.Solving);
                        fixResult(false, false, setMessage);
                    }}
                />
            }
        </div>
    )
}

export default EditorSidebar;
