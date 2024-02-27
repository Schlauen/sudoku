const TimerState = {
    Running: 0,
    Stop: 1,
    Reset: 2,
}
var timerState = TimerState.Reset;

function Timer(handleTimerState, setTimerState) {
    let timerContainer = document.createElement('div');
    timerContainer.className = 'menu-element key-value';

    let label = document.createElement("label");
    label.innerHTML = 'timer:';
    timerContainer.appendChild(label);

    let timer = document.createElement('label');
    timer.id = 'timer';
    timerContainer.appendChild(timer);

    var x = setInterval(
        function() {
            if (timerState == TimerState.Running) {
                var now = new Date().getTime();
                var distance = now - startDate;

                var hours = Math.floor((distance % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
                var minutes = Math.floor((distance % (1000 * 60 * 60)) / (1000 * 60));
                var seconds = Math.floor((distance % (1000 * 60)) / 1000);
        
                hours = hours < 10 ? '0' + hours : hours;
                minutes = minutes < 10 ? '0' + minutes : minutes;
                seconds = seconds < 10 ? '0' + seconds : seconds;

                timer.innerHTML = hours + ":" + minutes + ":" + seconds;

                if (distance < 0) {
                    clearInterval(x);
                    timer.innerHTML = "EXPIRED";
                };
            } else if (timerState == TimerState.Stop) {
            } else if (timerState == TimerState.Reset) {
                timer.innerHTML = '00:00:00';
            }
      }, 
      1000
    );
    return timerContainer;
}

export default Timer;