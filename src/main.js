const { invoke } = window.__TAURI__.tauri;

var questionDiv = document.getElementById("qus");;
// questionDiv.textContent = 'Я новое текстовое содержимое' ;

// window.addEventListener("DOMContentLoaded", () => {
//     questionDiv = document.querySelector("#qus");
//   });

function start() {
    var xhr = new XMLHttpRequest();

    xhr.onload = function () {
        if (xhr.status === 200) {
            document.body.innerHTML = xhr.responseText;

            var backButton = document.querySelector('.back');
            backButton.addEventListener('click', goBack);
        } else {
            console.log('Error loading game.html. Status code: ' + xhr.status);
        }
    };

    xhr.open('GET', 'game.html', true);
    xhr.send();

}

async function game() {
    'use strict';

    const div = document.createElement("div");

    div.classList.add('black');

    document.body.append(div);

    // Вставка текста в тег div
    div.textContent = await invoke("question");
}

function goBack() {
    window.location.reload();
}

function endGame() {
    var xhr = new XMLHttpRequest();

    xhr.onload = function () {
        if (xhr.status === 200) {
            document.body.innerHTML = xhr.responseText;

            var backButton = document.querySelector('.back');
            backButton.addEventListener('click', goBack);
        } else {
            console.log('Error loading end.html. Status code: ' + xhr.status);
        }
    };

    xhr.open('GET', 'end.html', true);
    xhr.send();
}

// window.addEventListener("DOMContentLoaded", () => {
// //  greetInputEl = document.querySelector("#greet-input");
// //  greetMsgEl = document.querySelector("#greet-msg");
// //  document.querySelector("#start-form").addEventListener("submit", (e) => {
// //    e.preventDefault();
// //    greet();

// //   });
// });
