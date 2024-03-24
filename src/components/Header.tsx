import { useEffect } from 'react'
import { useStore, State } from '../store';



const Header = () => {
    const setMessage = useStore(state => state.changeMessage);
    const gameState = useStore(state => state.gameState);
    
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

    const message = useStore(state => state.message);

    return (
        <div id='header'>
            {message}
        </div>
    )
};

export default Header
