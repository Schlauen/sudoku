import { create } from 'zustand';

export const State = {
    Blank: 0,
    Running: 1,
    Solved: 2,
    Error: 3,
}

type GameState = {
    gameState: number;
    message: string;
    changeGameState: (newGameState:number) => void;
    changeMessage: (newMessage:string) => void;
}

export const useStore = create<GameState>((set) => ({
    gameState: State.Blank,
    message: 'Welcome to a new game of Sudoku!',
    changeMessage: newMessage => set({message: newMessage}),
    changeGameState: newGameState => set((state) => {
        if (state.gameState == State.Blank) {
            if (newGameState == State.Running) state.changeMessage("Let's get cracking!");
            else if (newGameState == State.Solved) state.changeMessage("Hmm, why are you so fast?");
        }
        else if (state.gameState == State.Running) {
            if (newGameState == State.Blank) state.changeMessage("new Game new Pain!");
            else if (newGameState == State.Solved) state.changeMessage("Congratulations! You made it!");
        }
        else if (state.gameState == State.Solved) {
            if (newGameState == State.Blank) state.changeMessage("Let's see if you can do it faster this time");
        }
        else if (state.gameState == State.Error) {
            if (newGameState == State.Blank) state.changeMessage("you give it up already?");
            else if (newGameState == State.Solved) state.changeMessage("Congratulations! You made it!");
        }
        
        return { gameState: newGameState}
    }),
}))