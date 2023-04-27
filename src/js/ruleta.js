
const invoke = window.__TAURI__.invoke
let players = [];

window.addEventListener("resize", function(){
    dividir_ruleta(players.length);
});
window.addEventListener("DOMContentLoaded", function(){
    invoke('send_players').then((message) => {
        players = message.split('\n');
        players.pop();

        dividir_ruleta(players.length);        
        enlistar_players(players);
    });
});

const button = document.getElementById("spin_button");
const ruleta = document.getElementById("ruleta");

button.addEventListener("click", function(){
    ruleta.classList.toggle('rotate');
});

function ask_players(){
    invoke('send_players').then((message) => {
        players = message.split('\n');
        players.pop();

        dividir_ruleta(players.length);
        enlistar_players(players);
    });
}

setInterval(ask_players, 1000);

function dividir_ruleta(players){
    let ruleta = document.getElementById("ruleta");
    let canvas = document.getElementById("ruleta_canvas");
    let ctx = canvas.getContext("2d");

    let width = ruleta.offsetWidth;
    let height = width;

    canvas.width = width;
    canvas.height = height;

    ctx.beginPath();
    let angle = (2 * Math.PI) / players;
    if (players>1){
        for(let i=0; i<players; i++){
            ctx.moveTo(width/2, height/2);
            ctx.lineTo(
                (width / 2 ) + (width / 2) * Math.cos(angle * i),
                (height / 2 ) + (height / 2) * Math.sin(angle * i)
            );
            ctx.lineWidth = 1;
            ctx.strokeStyle = "gray";
            ctx.stroke();

            
        }
    }
}

function enlistar_players(lista_players){
    let lista = document.getElementById("lista");
    lista.innerHTML = lista_players.map(lista_players => `<li>${lista_players}</li>`).join('');
}
