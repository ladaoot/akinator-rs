const { invoke } = window.__TAURI__.tauri;


function start() {
    // game(1);
    window.onload = game(0);
    window.location.href = 'game.html';



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
        Div.textContent = 'Я думаю это ' + get[0];
        btn[0].style.display = 'none';
        btn[1].style.display = 'none';
        const ans = document.querySelectorAll(".answ_btn")
        ans[0].style.display = 'block';
        ans[1].style.display = 'block';
    } else if (get.length === 0) {
        Div.textContent = `Я не могу понять кто это...  Если вы хотите добавить человека, то нажмите на кнопку \"Добавить\"`;
        btn[0].style.display = 'none';
        btn[1].style.display = 'none';
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
            let add = await invoke('save', { name: enterName });
            if (add == 'ok') {
                Div.textContent = 'Добавили ' + enterName + ' в базу. Теперь я могу отгадать этого человека. Чтобы проверить нажмите "Вернуться назад".';
                await invoke('cleanYes');
            } else {
                Div.textContent = 'Это человек уже есть в базе. Хотите попробовать его отгадать?'

            }

        }

    } else {
        Div.textContent = 'Я не могу добавить это человека, нет вопросов к которым бы он подходил.'
    }

}

function win() {
    const Div = document.querySelector("#q");
    Div.textContent = 'Ура я отгадал!!!';
    const ans = document.querySelectorAll('.answ_btn');
    ans[0].style.display = 'none';
    ans[1].style.display = 'none';
}

function loss() {
    const Div = document.querySelector("#q");
    Div.textContent = 'O нет. Хочешь добавить это человека?';
    const ans = document.querySelectorAll('.answ_btn');
    ans[0].style.display = 'none';
    ans[1].style.display = 'none';
    const btn = document.querySelectorAll(".game_btn");
    btn[2].style.display = 'block';
}

