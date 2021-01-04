import {Result, ok, err} from "neverthrow";
import { OutOfBounds, TileOccupied } from "./errors";
        
// Types and Interfaces
type StatusMessage = "win" | "draw" | "loss" | "undeclared";
export type Board2D = string[][];
export type Coordinates = [number, number];
export type GamePiece = "X" | "O";

export interface BoardStatus {
    board: Board2D;
    state: StatusMessage;
};

export interface Diagonal {
    left: string[];
    right: string[];
};


export class Board {
    protected _board: Board2D;
    protected _size: number;
    
    constructor(sizeOfBoard = 3) {
        this._size = sizeOfBoard;
        this._board = this._new();
    }

    protected _new(): Board2D {
        return Array(this._size).fill([]).map(() => Array(this._size).fill("#"));
    }; 

    protected _getAllPossibleCombinations(board: Board2D = this._board): string[][] {
        let combinations: string[][] = [...this._board];
        let diagonal: Diagonal = { left: [], right: [] };

        for(let i = 0; i < this._size; i++){
            diagonal.left.push(board[i][i]);
            diagonal.right.push(board[i][this._size-i-1]);
            // Vertical
            combinations.push(board.map((val: string[]) => val[i]))
        };

        return [...combinations, diagonal.left, diagonal.right]
    }

    protected _determineBoardStatus(board: Board2D = this._board): StatusMessage {
        let combinations = this._getAllPossibleCombinations(board);

        let getWinner = (comb: string[], gamePiece: "X" | "O"): Result<{}, {}> => {
            if(comb.filter(c => c === gamePiece).length === 3) return ok({});
            else return err({}); 
        } 
        for (let comb of combinations) {
            if(getWinner(comb, "X").isOk()) return "win";
            if(getWinner(comb, "O").isOk()) return "loss";
        };

        let flattenedCombinations = combinations.reduce((acc, val) => acc.concat(val), []);
        if(!flattenedCombinations.includes("#")) return "draw";

        return "undeclared";
    }

    public placeGamePiece(coords: Coordinates, gamePiece: GamePiece, bot = false): Result<string, OutOfBounds | TileOccupied> {
        let x = coords[0];
        let y = coords[1];
        if(bot) {
            x+=1
            y+=1
        }
        if(x < 1 || x > this._size || y < 1 || y > this._size) {
            return err({
                type: "OutOfBounds",
                minVal: 1,
                maxVal: this._size,
                message: "Coords are out of bounds"
            });
        } else if(this._board[y-1][x-1] === "X" || this._board[y-1][x-1] === "O"){
            return err({
                type: "TileOccupied",
                message: "Tile is occupied"
            });
        };

        this._board[y-1][x-1] = gamePiece;
        return ok("Success");
    }

    public status(): BoardStatus {
        return {board: this._board, state: this._determineBoardStatus()}
    };

    public getBoard(): Board2D {
        return this._board;
    };

};



