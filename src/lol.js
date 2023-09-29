const { invoke } = window.__TAURI__.tauri;
const event = new Event("update");


//input variables
    let addAgentX, addAgentY;

//canvas variables
    let gameData=[];
    const gap = 5   ;
    let c_width = 500, c_height = 500;
    var c ;
    var ctx ; 
    let time = document.querySelector("#time").value;
    let oldTime = time.value;




async function add_agent(posX,posY){
    try{
        await invoke("add_agent",{posX:Number(addAgentX.value),posY:Number(addAgentY.value)});
    }
    catch(e){console.log(e);}
}
async function game_data(){
    try{
        gameData = JSON.parse(await invoke("get_game_data"));
    }
    catch(e){console.log(e);}
}
async function update_game(){
    try{
        await invoke("update_game");
    }
    catch(e){console.log(e);}
}
async function reset_game_data(){
    console.log("afsfasfas");
    try{await invoke("game_data_restart");}
    catch(e){console.log(e);}
}




function fill_sqare(pos_x, pos_y){
    pos_x = pos_x * gap, pos_y = pos_y * gap;
    ctx.fillStyle= "black";
    ctx.fillRect(pos_x, pos_y, gap,gap)
}

async function update_data(){
    gameData = JSON.parse(await update_game());
}

async function make_grid(){
    
    let x =0,y = 0;

    let counter = 0;
    //x lines
    for (let i = 0; i < c_width/gap+2; i++) {
        ctx.moveTo(x,y);
        ctx.lineTo(x,c_height);
        ctx.stroke();
        x +=gap;
    }
    x = 0;
    for (let i = 0; i < c_width/gap+2; i++) {
        ctx.moveTo(x,y);
        ctx.lineTo(c_width,y);
        ctx.stroke();
        y +=gap;
    }
}

async function update_all(){
    await update_data();
    await update_canvas();

}

async function update_canvas(){
    ctx.clearRect(0, 0, c_height, c_width);
    console.log(gameData);
    game_data();
    //data = JSON.parse(gameData);
    for (let i =0; i < gameData.length; i++) {
        fill_sqare(gameData[i][0], gameData[i][1]);
        
    }
    console.log("bitti");
}


function start_interval(time) {
    setInterval(update_canvas, time*100);
}

async function _add_agent(){
    await add_agent();
    await game_data();
    await update_canvas();
}
async function _reset(){
    await reset_game_data();
    await game_data();
    await update_canvas();
}

async function _update(){
    await update_game();
    await game_data();
    await update_canvas();
}

window.addEventListener("DOMContentLoaded", () => {
    addAgentX = document.querySelector("#input-posX")
    addAgentY = document.querySelector("#input-posY")


    
    
    document.querySelector("#reset-form").addEventListener("submit", (e) => {
        e.preventDefault();
        _reset();
    });

    document.querySelector("#second-form").addEventListener("submit", (e) => {
        e.preventDefault();
        _update();
    });
    document.querySelector("#init-game").addEventListener("submit", (e) => {
        e.preventDefault();
        _add_agent();

    });
});


//start_interval(oldTime)

async function main(){
    console.log("çalışiy");
    document.querySelector("#canvas-div").innerHTML = '<canvas width="'+c_width+'" height="'+c_height+'" id="game-screen" style="border:1px solid #000000; width: 8   0%;"></canvas>'
    c = document.getElementById("game-screen");
    ctx = c.getContext("2d");
    //make_grid();
}

await main();
