const reader = require("readline-sync");
import { Board, Board2D } from "./board";
import { Bot } from "./bot";
import { err } from "neverthrow";

function logBoard(board: Board2D) {
    board.forEach(b => console.log(b.toString()))
}

function sleep(ms: number): any  {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}   

async function main(){
    let board = new Board();
    
    console.clear();
    console.log("Welcome to TicTacToe! (3x3)");
    console.log("X: You\nO: Opponent\n#: Empty\n");
    reader.question("Press any key to start!");
    console.clear();

    console.log(`
 █   ██▀ ▀█▀ ▀ ▄▀▀   ██▄ ██▀ ▄▀  █ █▄ █ █
 █▄▄ █▄▄  █    ▄██   █▄█ █▄▄ ▀▄█ █ █ ▀█ ▄
                `);
    await sleep(1000);
    console.clear();

    console.log("Board:\n#,#,#\n#,#,#\n#,#,#\n");
    
    while (true) {
        console.log("Select a row!");
        let y: string = reader.question("Row: ");

        console.clear();
        
        console.log("Board:");
        board.getBoard().forEach((row, i: number) => {
            if(i === parseInt(y)-1) console.log(`${row} <--`);
            else console.log(`${row}`);
        });
        console.log("");

        console.log("Select a tile!");
        let x: string = reader.question("Tile: ");
        let result = board.placeGamePiece([parseInt(x), parseInt(y)], "X");
    
        if (result.isErr()) {
            console.clear();

            console.log("Board:");
            logBoard(board.getBoard());
            console.log(`\nError: ${result.error.message}\n`)
        }
    
        console.clear()
        console.log("Opponent is placing...");
    
        let sleepTiming = Math.floor(Math.random() * 1000);

        await sleep(sleepTiming);
        console.clear()
    
        if(board.status().state !== "undeclared") break;
        
        let bot = new Bot(board.getBoard());
        board.placeGamePiece(bot.getNextPlacement(), "O", true);
    
        console.log("Board:");
        logBoard(board.getBoard());
        console.log("");
    
        if(board.status().state !== "undeclared") break;
    }
}

main();
