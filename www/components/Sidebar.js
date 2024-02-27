import LevelRange from "./LevelRange";
import Checkbox from "./Checkbox";
import Button from "./Button";

const TimerState = {
    Running: 0,
    Stop: 1,
    Reset: 2,
}

var startDate = new Date().getTime();
var timerState = TimerState.Reset;

function startTimer() {
    timerState = TimerState.Running;
    startDate = new Date().getTime();
}

function stopTimer() {
    timerState = TimerState.Stop;
}

function resetTimer() {
    startDate = new Date().getTime();
    timerState = TimerState.Reset;
    let timer = document.getElementById('timer');
    timer.innerHTML = '00:00:00';
}

function Sidebar(state, updateCell) {
    let updateCells = () => {
        for (let i = 0; i < 9; i += 1) {
            for (let j = 0; j < 9; j += 1) {
                updateCell(i, j);
            }   
        }
    };

    let sidebar = document.createElement("div");
    sidebar.id = 'sidebar';

    sidebar.appendChild(Checkbox('show errors:', state, updateCells))
    
    sidebar.appendChild(LevelRange());

    sidebar.appendChild(
        Button(
            'generate',
            () => {
                let level = document.getElementById('level-value').value;
                console.log('level ' + level);
                state.generate(level);
                updateCells();
                startTimer();
            }
        )
    );
    sidebar.appendChild(
        Button(
            'reset',
            () => {
                state.reset();
                Array.from(document.getElementsByClassName('cell')).forEach(cell => cell.cellState = -1);
                updateCells();
                resetTimer();
            }
        )
    );
    sidebar.appendChild(
        Button(
            'solve',
            () => {
                state.solve();
                updateCells();
                stopTimer();
            }
        )
    );

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

    sidebar.appendChild(timerContainer);

    return sidebar;
}




export default Sidebar;