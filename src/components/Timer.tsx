import { forwardRef, useEffect, useImperativeHandle, useState } from 'react'
import { useStore, State } from '../store';

const Timer = forwardRef((_, ref) => {
    const [time, setTime] = useState('00:00:00');
    const [startTime, setStartTime] = useState(new Date().getTime());

    const start = () => {
        setStartTime(new Date().getTime());
        setTime('00:00:00');
    }

    useImperativeHandle(ref, () => {
        return {
            start:start,
        };
    });

    const gameState = useStore(state => state.gameState);
    
    useEffect(() => {
        const interval = setInterval(
            function() {
                
                if (gameState == State.Running || gameState == State.Error) {
                    let now = new Date().getTime();
                    let distance = now - startTime;
    
                    let hours = Math.floor((distance % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
                    let minutes = Math.floor((distance % (1000 * 60 * 60)) / (1000 * 60));
                    let seconds = Math.floor((distance % (1000 * 60)) / 1000);
            
                    let hoursStr = hours < 10 ? '0' + hours : hours;
                    let minutesStr = minutes < 10 ? '0' + minutes : minutes;
                    let secondsStr = seconds < 10 ? '0' + seconds : seconds;
    
                    setTime(hoursStr + ":" + minutesStr + ":" + secondsStr);
    
                    if (distance < 0) {
                        clearInterval(interval);
                        setTime("EXPIRED");
                    };
                } else if (gameState == State.Solved) {
                } else if (gameState == State.Blank) {
                    setTime('00:00:00');
                }
            }, 
            1000
        );
        return () => clearInterval(interval);
    })

    return (
        <div className='menu-element key-value'>
            <label>timer:</label>
            <label id='timer'>{time}</label>
        </div>
    )
})

export default Timer
