import { Board, Board2D } from "./board";

export class Bot extends Board {
  constructor(board: Board2D = [["#", "#", "#"], ["#", "#", "#"], ["#", "#", "#"]]) {
      super();
      this._board = board;
  }

  private _getEmptyTiles(board: Board2D): number[][] {
    let emptyTiles: number[][] = [];
    board
        .forEach((slice, y) => slice
        .forEach((tile, x) => tile === "#" 
            ? emptyTiles.push([x, y]) 
            : null
    ));
    return emptyTiles;
  };

  private _findBestMove(board: Board2D): [number, number] {
      let ret: [number /* Best value */, [number, number]] = [-100, [0, 0]];
      this._getEmptyTiles(board).forEach((tile) => {
          const [x, y] = tile;
          board[y][x] = "O";
          const value = this._minimax(board, false, 0);
          board[y][x] = "#";
          if(ret[0] < value) ret = [value, [x, y]];
      });
      return ret[1];
  };
    
  private _minimax(board: Board2D, isMaximizing = true, depth = 0): number {
    const state = this._determineBoardStatus(board);
    if(state === "loss") return 100 - depth;
    else if(state === "win") return -100 + depth;
    else if(state === "draw") return 0;
      
    let bestVal = isMaximizing ? -100 : 100;
    let player = isMaximizing ? "O" : "X";
    this._getEmptyTiles(board).forEach((tile) => {
        const [x, y] = tile;
        board[y][x] = player;
        let value = this._minimax([...board], !isMaximizing, depth + 1);
        bestVal = isMaximizing 
            ? Math.max(value, bestVal) 
            : Math.min(value, bestVal);
        board[y][x] = "#";
    });

    return bestVal;   
  };
    
  public getNextPlacement(): [number, number] {
    return this._findBestMove(this._board);
  }

}
