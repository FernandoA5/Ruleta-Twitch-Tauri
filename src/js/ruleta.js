
const invoke = window.__TAURI__.invoke
let players = [];
let last_players = [];
let spinning = false;

let panel_ruleta = document.getElementById("panel_ruleta");
let panel_controles = document.getElementById("panel_controles");
let panel_players = document.getElementById("panel_players");

//-------------------------------------------------------COSAS DE LA VENTANA -----------------------------------------------------
window.addEventListener("resize", function(){
    dividir_ruleta(players.length);

    //console.log(window.innerHeight + ":" + window.innerWidth)
    

    if(this.window.innerHeight >= this.window.innerWidth/1.3){
        panel_ruleta.className = "col-sm-6";
        panel_controles.className = "col-sm-6";
        panel_players.className = "col-md-4";
        console.log("Cambiando a 6:6");
    }
    else{
        panel_ruleta.className = "col-sm-5";
        panel_controles.className = "col-sm-3";
        panel_players.className = "col-md-4";
        console.log("Cambiando a 5:3:4");
    }


});
window.addEventListener("DOMContentLoaded", function(){
    //ESTE VA PRIMERO PORQUE AL INICIAR LA CONEXÍON CORRIGE EL ARCHIVO EN CASO DE SER UN CANAL INCORRECTO
    invoke('start_connection_twitch').then((message =>{
        console.log(message);
    })).then(()=>{
        //UNA VEZ FUNCIONANDO LA CONEXIÓN, OBTENEMOS EL CANAL QUE ESTÁ FUNCIONANDO
        invoke('get_channel').then((message) => {
            this.document.getElementById("input_canal").placeholder = message;
            console.log("Canal: " + message);
        });
        invoke('get_command').then((message) => {
            this.document.getElementById("input_comando").placeholder = message;
            console.log("Comando: " + message);
        });
    });
    
    invoke('send_players').then((message) => {
        players = message.split('\n');
        players.pop();

        dividir_ruleta(players);        
        //anunciar_ganador(0);
    });

    enlistar_players(players);
    agregar_listeners();

    
});

/*--------------------------------------------------BOTONES Y SUS EVENT-LISTENERS------------------------------------------------*/

//OBTENEMOS LOS BOTONES CON SUS RESPECTIVOS IDS
const spin_button = document.getElementById("spin_button");
const ruleta = document.getElementById("ruleta");
const clear_button = document.getElementById("clear_button");
const add_button = document.getElementById("button_add");
const channel_button = document.getElementById("button_canal");
const command_button = document.getElementById("button_comando");

channel_button.addEventListener('click', function(){
    //VERIFICAMOS QUE NO ESTÉ GIRANDO LA RULETA
    if(spinning == false){
        //OBTENEMOS EL NOMBRE DEL CANAL A AGREGAR
        let tb_channel = document.getElementById("input_canal");
        let channel = tb_channel.value.toLowerCase();

        if(channel!=""){
            //LE PEDIMOS A RUST QUE AGREGUE EL CANAL
            console.log("Enviado canal a Rust");
            invoke('set_channel', { channel: channel}).then((message) => {
                let tb_channel = document.getElementById("input_canal");
                tb_channel.value = ""; //LIMPIAMOS EL INPUT
                console.log(message);
            }).then(()=>{
                //INICIAMOS LA CONEXIÓN, ELLA SE ENCARGARÁ DE CORREGIR EL ARCHIVO EN CASO DE SER NECESARIO
                invoke('start_connection_twitch').then((message =>{
                    console.log(message);
                })).then(()=>{
                    //UNA VEZ FUNCIONANDO LA CONEXIÓN, OBTENEMOS EL CANAL QUE ESTÁ FUNCIONANDO
                    invoke('get_channel').then((message) => {
                        document.getElementById("input_canal").placeholder = message;
                        
                        console.log("Canal: " + channel+":"+message);
                        if(message === channel){ //LE DECIMOS AL USUARIO QUE FUNCIONÓ
                            alert("Conectado correctamente al canal: " + channel);
                        }else{  //LE DECIMOS AL USUARIO QUE NO FUNCIONÓ
                            alert("Error al conectarse a: "+channel);
                        }
                    });
                });
            });
        }
        else{
            alert("Todo pendejo: Te falto poner el nombre del canal");
        }
    }else{//AVISAMOS AL USUARIO QUE NO PUEDE AGREGAR UN CANAL MIENTRAS LA RULETA GIRA
        alert("No puedes agregar un canal mientras la ruleta gira");
    }
});

command_button.addEventListener('click', function(){
    //VERIFICAMOS QUE NO ESTÉ GIRANDO LA RULETA
    if(spinning == false){
        //OBTENEMOS EL NOMBRE DEL COMANDO A AGREGAR
        let tb_command = document.getElementById("input_comando");
        let command = tb_command.value;

        if(command!=""){ //VALIDAMOS QUE EL COMANDO NO ESTÉ VACÍO
            //LE PEDIMOS A RUST QUE AGREGUE EL COMANDO
            invoke('set_command', { command: command}).then((message) => {
                let tb_command = document.getElementById("input_comando");
                tb_command.value = ""; //LIMPIAMOS EL INPUT
                console.log(message);
            }).then(()=>{ //UNA VEZ AGREGADA, LE PEDIMOS INICIE LA CONEXIÓN
                invoke('start_connection_twitch').then((message =>{
                    console.log(message);
                })).then(()=>{
                    //UNA VEZ FUNCIONANDO LA CONEXIÓN, OBTENEMOS EL COMANDO QUE ESTÁ FUNCIONANDO
                    invoke('get_command').then((message) => {
                        document.getElementById("input_comando").placeholder = message;
                        console.log("Comando: " + message);
                        if(message === command){ //LE DECIMOS AL USUARIO QUE FUNCIONÓ
                            alert("Comando agregado correctamente: " + command);
                        }
                        else{  //LE DECIMOS AL USUARIO QUE NO FUNCIONÓ
                            alert("Error al agregar comando: "+command);
                        }
                    });
                });

            });
        }

    }
});


//LISTENER DEL BOTON PARA BORRAR USUARIOS
clear_button.addEventListener("click", function(){
    //EL USUARIO NECESITA SABER QUE PASA
    if(players.length==0)
        alert("No hay jugadores, pedazo de idiota");

    //VERIFICAMOS QUE NO ESTÉ GIRANDO LA RULETA
    if(spinning == false && players.length!=0){
        //LE PEDIMOS A RUST QUE BORRE LOS USUARIOS
        invoke('clear_players').then((message) => {
            console.log(message);
        });
    }

});

//LISTENER DEL BOTON PARA AGREGAR USUARIOS MANUALMENTE
add_button.addEventListener("click", function(){

    //VERIFICAMOS QUE NO ESTÉ GIRANDO LA RULETA
    if(spinning == false){
        //OBTENEMOS EL NOMBRE DEL USUARIO A AGREGAR
        const tb_input = document.getElementById("input_usuario");
        let user = tb_input.value;
        if(user != ""){
            tb_input.value = ""; //LIMPIAMOS EL INPUT

            if(user != ""){
                //LE PEDIMOS A RUST QUE AGREGUE EL USUARIO
                invoke('add_player', { player: user}).then((message) => {
                    console.log(message);
                });
            }
        }
        else
        {
            alert("¿Necesito decirte que no puedes agregar un usuario sin nombre?");
        }
    }
});

//LISTENER DEL BOTON PARA ROTAR LA RULETA
spin_button.addEventListener("click", function(){
    //EL USUARIO NECESITA SABER QUE PASA
    if(players.length==0)
        alert("No hay jugadores, pedazo de idiota");

    //VERIFICAMOS QUE NO ESTÉ GIRANDO LA RULETA Y QUE HAYA JUGADORES
    if(spinning==false && players.length!=0){ //SI NO ESTÁ GIRANDO PODEMOS GIRAR LA RULETA

        //RESETEAMOS LA ANIMACÓN
        ruleta.style.transform = `rotate(${0}deg)`;
        ruleta.style.transition = "transform 1ms ease-in-out";
        ruleta.classList.toggle('rotate');

        //console.log(ruleta.style)

        setTimeout(function (){
            //GENERAR ANGULO ALEATORIO
            let randomAngle = Math.floor(Math.random() * 360);
            //Añadirle 360 para que de una vuelta completa y Lo multiplicamos por la cantidad de vueltas que queremos que de
            randomAngle += 360*50;

            //LE PASAMOS A LA ANIMACIÓN SUS RESPECTIVOS VALORES
            ruleta.style.transform = `rotate(${randomAngle}deg)`;
            ruleta.style.transition = "transform 10s ease-in-out";
            
            //INICIAMOS LA ANIMACIÓN
            ruleta.classList.toggle('rotate');
            spinning=true;

            //EL ANGULO EN QUE TERMINÓ LA RULETA
            setTimeout(function(){
                //OBTENEMOS EL ANGULO DE DESFASE DE LA RULETA DESPUES DE GIRAR
                let angle = getRotationDegrees(ruleta) %360;

                //alert(randomAngle +": "+ (randomAngle % 360)+": "+angle);

                //CALCULAMOS Y ANUNCIAMOS EL GANADOR
                anunciar_ganador(angle);
                spinning=false

            },10500);

        }, 10);
    }


});

//ENLISTAMOS LOS USUARIOS EN LA LISTA
function enlistar_players(lista_players){
    let lista = document.getElementById("lista");
    lista.innerHTML = lista_players.map(lista_players => `<li><input type="button" value="${lista_players}" id="${lista_players}_button" class="drop-button"></li>`).join('');

}
//AGREGAMOS SUS RESPECTIVOS LISTENERS A LOS BOTONES DE CADA USUARIO EN LA LISTA
function agregar_listeners(){
    document.querySelector('#lista').addEventListener('click', function(event) {
        if (event.target.classList.contains('drop-button')) {
            let user = event.target.value;
            console.log(user);
            drop_user(user);
        }
    });
}


//------------------------------------------------------FEATURES ---------------------------------------------------------------

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

        console.log("Desfase: "+desface+" Angulo"+i+": "+players[i]+": " + startAngle + " | Angulo2"+players[i]+": "+ endAngle + "");
        
        if(startAngle >= 90 && ( endAngle < 90 || startAngle-(360 / players.length) <= 0  )){
            alert("El ganador es: " + players[i])             
        }
    }   
}



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
    //VERIFICAMOS QUE NO ESTÉ GIRANDO LA RULETA
    if (spinning == false){
        //SI NO ESTÁ GIRANDO PODEMOS RECIBIR USUARIOS
        invoke('send_players').then((message) => {
            players = message.split('\n');
            players.pop();

            //SI LA LISTA DE USUARIOS ENTRANTE ES LA MISMA QUE LA ANTERIOR NO HACEMOS NADA
            if(players!=last_players){    //EN CASO CONTRARIO DIBUJAMOS LA RULETA Y ENLISTAMOS LOS USUARIOS
                dividir_ruleta(players);
                enlistar_players(players);
                last_players = players;
            }
        });
    }
    
}

//CADA CUANTO LE PEDIMOS A RUST QUE NOS MANDE LA LISTA DE USUARIOS EN LA DB
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


function drop_user(user){
    invoke('drop_player', {player: user}).then((message) => {
        console.log(message);
    });
}





