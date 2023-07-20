const { invoke } = window.__TAURI__.tauri;



function start() {
    window.location.href='game.html';

}

async function game(x) {
    const btn = document.querySelectorAll(".game_btn");
    btn[0].style.display = 'block';
    btn[1].style.display = 'block';
    if (x===0) {
        restart();
    } else {
        const Div = document.querySelector("#q");
        Div.textContent = await invoke("question");
    }

}

function goBack() {
    window.history.back();
}


async function answer(x) {
    const Div = document.querySelector("#a");

    const get = await invoke('check', { answer: x });
    if (get.length === 1) {
        cleanQuestion();
        Div.textContent = 'I guess it is ' + get[0];
    } else if (get.length === 0) {
        cleanQuestion();
        Div.textContent = 'I cant guess... :('

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


