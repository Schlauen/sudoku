import { useEffect, useState } from 'react'
import { useStore, AppState } from '../store';
import { incrementTimer } from '../Interface';

const Timer = () => {
    const [time, setTime] = useState('00:00:00');

    const appState = useStore(state => state.appState);
    const onError = useStore(state => state.changeMessage)
    
    useEffect(() => {
        const interval = setInterval(
            function() {
                if (appState == AppState.Solving) {
                    incrementTimer(
                        (distance: number) => {
                            let hours = Math.floor(Math.round(distance / 3600));
                            let minutes = Math.floor(Math.round(distance / 60) % 60);
                            let seconds = Math.floor(distance % 60);
                    
                            let hoursStr = hours < 10 ? '0' + hours : hours; 
                            let minutesStr = minutes < 10 ? '0' + minutes : minutes;
                            let secondsStr = seconds < 10 ? '0' + seconds : seconds;
            
                            setTime(hoursStr + ":" + minutesStr + ":" + secondsStr);

                            if (distance < 0) {
                                clearInterval(interval);
                                setTime("EXPIRED");
                            };
                        },
                        onError
                    );
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
}

export default Timer
