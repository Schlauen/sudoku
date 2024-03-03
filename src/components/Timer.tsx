import { forwardRef, useEffect, useImperativeHandle, useState } from 'react'

const State = {
    Running: 0,
    Stop: 1,
    Reset: 2,
}

const Timer = forwardRef((_, ref) => {
    const [time, setTime] = useState('00:00:00');
    const [state, setState] = useState(State.Reset);
    const [startTime, setStartTime] = useState(new Date().getTime());

    const start = () => {
        setState(State.Running);
        setStartTime(new Date().getTime());
        setTime('00:00:00');
    }
    const stop = () => setState(State.Stop);
    const reset = () => {
        setState(State.Reset);
        setTime('00:00:00');
    }
    useImperativeHandle(ref, () => {
        return {
            start:start,
            stop:stop,
            reset:reset,
        };
    });

    useEffect(() => {
        const interval = setInterval(
            function() {
                if (state == State.Running) {
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
                } else if (state == State.Stop) {
                } else if (state == State.Reset) {
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
