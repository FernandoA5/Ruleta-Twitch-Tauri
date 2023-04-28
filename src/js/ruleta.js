
const invoke = window.__TAURI__.invoke
let players = [];

window.addEventListener("resize", function(){
    dividir_ruleta(players.length);
});
window.addEventListener("DOMContentLoaded", function(){
    invoke('send_players').then((message) => {
        players = message.split('\n');
        players.pop();

        dividir_ruleta(players);        
        enlistar_players(players);

        anunciar_ganador(0);
    });
});

const button = document.getElementById("spin_button");
const ruleta = document.getElementById("ruleta");

function anunciar_ganador(desface){
    for(let i = players.length-1 ; i>=0; i--){

        let startAngle = (i * (360 / players.length)) + desface;
        startAngle = startAngle - (360 * Math.floor((startAngle) / 360));
        //invertir angulo
        startAngle = 360 - startAngle;

        let endAngle = ((i+1) * (360 / players.length)) + desface
        endAngle = endAngle - (360 * Math.floor((endAngle) / 360));
        //invertir angulo
        endAngle = 360 - endAngle;

        //console.log("Desfase: "+desface+" Angulo"+i+": "+players[i]+": " + startAngle + " | Angulo2"+players[i]+": "+ endAngle + "");
        
        if(startAngle >= 90 && ( endAngle < 90 || startAngle-(360 / players.length) < 0  )){
            alert("El ganador es: " + players[i])             
        }
    }   
}

button.addEventListener("click", function(){
    //Obtener numero aleatorio entre 0 y 360
    ruleta.style.transform = `rotate(${0}deg)`;
    ruleta.style.transition = "transform 1ms ease-in-out";
    ruleta.classList.toggle('rotate');

    //console.log(ruleta.style)

    setTimeout(function (){
        let randomAngle = Math.floor(Math.random() * 360);
        //Añadirle 360 para que de una vuelta completa
        randomAngle += 360*50;
        
        ruleta.style.transform = `rotate(${randomAngle}deg)`;
        ruleta.style.transition = "transform 10s ease-in-out";

        

        //console.log(ruleta.style)
        ruleta.classList.toggle('rotate');

        //EL ANGULO EN QUE TERMINÓ LA RULETA
        setTimeout(function(){
            let angle = getRotationDegrees(ruleta) %360;
            //alert(randomAngle +": "+ (randomAngle % 360)+": "+angle);
            anunciar_ganador(angle);
        },10500);

    }, 10);


});

function getRotationDegrees(ruleta){
    const matrix = window.getComputedStyle(ruleta).getPropertyValue("transform")
    if (matrix){
        const matrixValues = matrix.match(/^matrix\((.+)\)$/)[1].split(', ');
        const angle = Math.round(Math.atan2(matrixValues[1], matrixValues[0]) * (180/Math.PI));
        return (angle < 0 ? angle + 360 : angle);
    }
    return 0;    
}

function ask_players(){
    invoke('send_players').then((message) => {
        players = message.split('\n');
        players.pop();

        dividir_ruleta(players);
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
    let angle = (2 * Math.PI) / players.length;

    if (players.length>1){
        for(let i=0; i<players.length; i++){
            ctx.moveTo(width/2, height/2);
            ctx.lineTo(
                (width / 2 ) + (width / 2) * Math.cos(angle * i),
                (height / 2 ) + (height / 2) * Math.sin(angle * i)
            );
            ctx.lineWidth = 1;
            ctx.strokeStyle = "gray";

            ctx.save();
            ctx.translate(width/2, height/2);
            ctx.rotate(angle * i + angle/2);
            ctx.textAlign = "center";

            //LO SEPARAMOS UN POCO DEL CENTRO PARA QUE NO SE SUPERPONGAN
            let text="    "+ players[i]+"   ";
            let textWidth = ctx.measureText(text).width;
            //0.7 para darle algo de holgura
            let proportion = textWidth / (width*0.65);
            let fontSize = (width / 2) * 0.05 / proportion;


            ctx.font = `${fontSize}px Arial`;
            ctx.fillStyle = "black";

            ctx.fillText(text, width / 4 , 0);
            ctx.restore();

            ctx.stroke();

            
        }
    }
}

function enlistar_players(lista_players){
    let lista = document.getElementById("lista");
    lista.innerHTML = lista_players.map(lista_players => `<li>${lista_players}</li>`).join('');
}
