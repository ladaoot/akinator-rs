const { invoke } = window.__TAURI__.tauri;



function start() {
    window.location.href = 'game.html';
    game(0);

}

async function game(x) {
    if (x === 0) {
        restart();
    } else {
        const Div = document.querySelector("#q");
        Div.textContent = await invoke("question");
    }
    const btn = document.querySelectorAll(".game_btn");
    btn[0].style.display = 'block';
    btn[1].style.display = 'block';
    btn[2].style.display = 'none';
    btn[0].disabled = false;
    btn[1].disabled = false;
    

}

function goBack() {
    window.history.back();
    
}


async function answer(x) {
    const Div = document.querySelector("#a");
    const btn = document.querySelectorAll(".game_btn");
    const get = await invoke('check', { answer: x });
    if (get.length === 1) {
        cleanQuestion();
        Div.textContent = 'I guess it is ' + get[0];
        btn[0].disabled = true;
        btn[1].disabled = true;
    } else if (get.length === 0) {
        cleanQuestion();
        Div.textContent = "I cant guess... :(\nIf you want to add a person press ADD-button";
        btn[0].disabled = true;
        btn[1].disabled = true;
        btn[2].style.display = 'block';

    } else {
        game(1);
    }
}

function cleanQuestion() {
    const Div = document.querySelector("#q");
    Div.textContent = "";
}

async function restart() {
    const Div = document.querySelector("#a");
    Div.textContent = "";
    await invoke('restart');
    game(1);
}

function add() {
    window.location.href = 'add.html';
}



async function save() {
    const enterName = document.querySelector("#name").value;
    await invoke('save', { name: enterName });
    const Div = document.querySelector('.done');
    Div.textContent = 'Add ' + enterName + ' to json';
    await invoke('cleanYes');
}





