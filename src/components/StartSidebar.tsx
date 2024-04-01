import Button from './Button'
import { AppState, OpenModal, useStore } from '../store';

const StartSidebar = () => {
    const changeOpenModal = useStore(state => state.changeOpenModal);
    const changeAppState = useStore(state => state.changeAppState);

    return (
        <div id='sidebar'>
            <Button
                name='new game'
                onClick={() => changeOpenModal(OpenModal.GenerateModal)}
            />
            <Button
                name='load'
                onClick={() => changeOpenModal(OpenModal.LoadModal)}
            />
            <Button
                name='editor'
                onClick={() => changeAppState(AppState.Editing)}
            />
        </div>
    )
}

export default StartSidebar
