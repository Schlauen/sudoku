import { forwardRef, useEffect, useImperativeHandle, useState } from 'react'

const State = {
    Blank: 0,
    Running: 1,
    Solved: 2,
    Error: 3,
}

const Header = forwardRef((_, ref) => {
    const [message, setMessage] = useState('Welcome to a new game of Sudoku!');
    const [gameState, setGameState] = useState(State.Blank);

    const changeGameState = (newState:number) => {

        if (gameState == State.Blank) {
            if (newState == State.Running) setMessage("Let's get cracking!");
            else if (newState == State.Solved) setMessage("Hmm, why are you so fast?");
        }
        else if (gameState == State.Running) {
            if (newState == State.Blank) setMessage("new Game new Pain!");
            else if (newState == State.Solved) setMessage("Congratulations! You made it!");
        }
        else if (gameState == State.Solved) {
            if (newState == State.Blank) setMessage("Let's see if you can do it faster this time");
        }
        else if (gameState == State.Error) {
            if (newState == State.Blank) setMessage("you give it up already?");
            else if (newState == State.Solved) setMessage("Congratulations! You made it!");
        }
        
        setGameState(newState);
    }
    
    useImperativeHandle(ref, () => {
        return {
            setMessage: setMessage,
            setGameState: changeGameState,
        }
    });
    useEffect(() => {
        const interval = setInterval(
            function() {
                if (gameState == State.Solved) return;
                setMessage('');
            }, 
            3000
        );
        return () => clearInterval(interval);
    })

    return (
        <div id='header'>
            {message}
        </div>
    )
});

export default Header
