import { create } from 'zustand';

export const AppState = {
    Start: 0,
    Solving: 1,
    Editing: 2,
    Solved: 3,
}

export const OpenModal = {
    None: 0,
    LoadModal: 1,
    GenerateModal: 2,
    SaveModal: 3
}

type GameState = {
    message: string;
    changeMessage: (newMessage:string) => void;
    controlsEnabled: boolean;
    setControlsEnabled: (enabled: boolean) => void;
    appState: number;
    changeAppState: (newAppState:number) => void;
    openModal: number;
    changeOpenModal: (newOpenModal:number) => void;
    showError: boolean;
    setShowError: (newShowError:boolean) => void;
}

export const useStore = create<GameState>((set) => ({
    message: 'Welcome to Sudoku!',
    changeMessage: newMessage => set({message: newMessage}),
    controlsEnabled: true,
    setControlsEnabled: enabled => set({controlsEnabled: enabled}),
    appState: AppState.Start,
    changeAppState: newAppState => set({appState: newAppState}),
    openModal: OpenModal.None,
    changeOpenModal: newOpenModal => set(state => {
        if (newOpenModal != OpenModal.None) {
            state.controlsEnabled = false;
        } else {
            state.controlsEnabled = true;
        }
        return {openModal: newOpenModal}
    }),
    showError: false,
    setShowError: newShowError => set({showError: newShowError}),
}));