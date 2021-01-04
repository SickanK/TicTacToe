import assert from "assert";
import { Board, BoardStatus } from "../src/board";
import { Bot } from "../src/bot";

describe("Get result of game", () => {
    it("should return win", () => {
        let board: Board  = new Board();

        board.placeGamePiece([1, 1], "X")
        board.placeGamePiece([1, 2], "X")
        board.placeGamePiece([1, 3], "X")

        let result: BoardStatus = board.status();
    
        assert.equal(result.state, "win");
    });

    it("should return undeclared", () => {
        let board: Board  = new Board();

        board.placeGamePiece([2, 3], "O")
        board.placeGamePiece([1, 2], "X")
        board.placeGamePiece([1, 3], "X")

        let result: BoardStatus = board.status();
    
        assert.equal(result.state, "undeclared");
    });

    it("should return loss", () => {
        let board: Board  = new Board();

        board.placeGamePiece([1, 1], "O")
        board.placeGamePiece([1, 2], "O")
        board.placeGamePiece([1, 3], "O")

        let result: BoardStatus = board.status();
    
        assert.equal(result.state, "loss");
    });
});

describe("Bot get placement of game-piece", () => {
    it("should return [0, 1]", () => {
        let templateBoard = [["O", "X", "#"], ["#", "X", "X"], ["#", "O", "#"]]
        let board = new Bot(templateBoard);
        assert.equal(board.nextPlacement().toString(), [0, 1].toString());
    });

    it("should return [0, 2]", () => {
        let templateBoard = [["O", "X", "X"], ["O", "X", "X"], ["#", "O", "#"]]
        let board = new Bot(templateBoard);
        assert.equal(board.nextPlacement().toString(), [0, 2].toString());
    });
});
