const { invoke } = window.__TAURI__.tauri;


function start() {
    window.location.href = 'game.html';
    // game(0);

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
    history.back();

}


async function answer(x) {
    const Div = document.querySelector("#q");
    const btn = document.querySelectorAll(".game_btn");
    const get = await invoke('check', { answer: x });
    if (get.length === 1) {
        Div.textContent = 'I guess it is ' + get[0];
        btn[0].disabled = true;
        btn[1].disabled = true;
    } else if (get.length === 0) {
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
    await invoke('restart');
    game(1);
}

function add() {
    window.location.href = 'add.html';
}


async function save() {
    let is = await invoke('isYesEmpty');
    const Div = document.querySelector('.done');
    if (!is) {
        const enterName = document.querySelector("#name").value;
        if (enterName === '') {
            Div.textContent = 'You dont enter anything';
        } else {
            await invoke('save', { name: enterName });

            Div.textContent = 'Add ' + enterName + ' to json';
            await invoke('cleanYes');
        }

    } else {
        Div.textContent = 'I cant add this person. there is no question for this person.'
    }

}

