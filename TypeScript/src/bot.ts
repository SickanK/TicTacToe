import { Board, Board2D } from "./board";

export class Bot extends Board {
  _nodesMap: Map<number, [number, number]>;
  _bestMove: [number, number] | undefined;

  constructor(board: Board2D = [["#", "#", "#"], ["#", "#", "#"], ["#", "#", "#"]]) {
      super();
      this._board = board;
      this._nodesMap = new Map();
      this._bestMove = [0, 0];
  }

  private _getEmptyTiles(board: Board2D): number {
    let emptyTiles = 0;
    board.forEach(slice => slice.forEach(tile => tile === "#" ? emptyTiles += 1 : null));
    return emptyTiles;
  };
    
  private _getBestValue(board: Board2D, maximize: boolean = true, depth: number = 0): number {
    if(depth === 0) this._nodesMap.clear();

    let state = this._determineBoardStatus(board);
    if(state !== "undeclared"){
        if(state === "loss") return 100 + depth;
        else if(state === "win") return 100 - depth;
        else {
         console.log("draw")
         return 0;
        }
    }

    if(maximize) {
        let best = -200; 

        board.forEach((slice, y) => {
            slice.forEach((tile, x) => {
                if(tile === "#") {
                    const child = board.map(b => b.slice())
                    child[y][x] = "X";
                    const nodeValue = this._getBestValue(child, false, depth+1);
                    if(depth === 0){
                        console.log(best, nodeValue, 1);
                        console.log(x, y, 1)
                    }
                    best = Math.max(best, nodeValue);

                    if(depth === 0)this._nodesMap.set(nodeValue, [x,y]);
                } 
            });
        });

        if(depth == 0){
            console.log(best);
            this._bestMove = this._nodesMap.get(best);
        }

        return best;
    } else {
        let best = 200;

        board.forEach((slice, y) => {
            slice.forEach((tile, x) => {
                if(tile === "#") {
                    const child = board.map(b => b.slice())
                    child[y][x] = "O";
                    const nodeValue = this._getBestValue(child, true, depth+1);

                    best = Math.min(best, nodeValue);

                    if(depth === 0) {
                        this._nodesMap.set(nodeValue, [x,y]);
                    }
                } 
            });
        });

        if(depth === 0) this._bestMove = this._nodesMap.get(best);
        return best;
    }
  }
    
  public nextPlacement(): any {
    let result: any = []
    // True if human starts else computer
    this._getBestValue(this._board, true, 0);
    console.log(this._nodesMap);
    console.log(this._bestMove);
    return this._bestMove;
  }

}
