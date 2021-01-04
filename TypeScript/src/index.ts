const reader = require("readline-sync");
import { Board, Board2D } from "./board";
import { Bot } from "./bot";
import {err} from "neverthrow";

function logBoard(board: Board2D) {
    board.forEach(b => console.log(b.toString()))
}

let board = new Board();

while (true) {
let x = reader.question("X: ");
let y = reader.question("Y: ");
let result = board.placeGamePiece([parseInt(x), parseInt(y)], "X");
if (result.isErr()) {
    console.log(err(result))
    continue;
}

logBoard(board.getBoard());
if(board.status().state !== "undeclared") break;

console.log("Opponent is placing");
let bot = new Bot(board.getBoard())
let test = bot.nextPlacement();
board.placeGamePiece(test, "O", true);

logBoard(board.getBoard());

if(board.status().state !== "undeclared") break;
}

console.log(board.status().state);

