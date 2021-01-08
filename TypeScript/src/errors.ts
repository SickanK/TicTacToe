//import { } from "./board";

export interface OutOfBounds {
    type: "OutOfBounds";
    maxVal: number;
    minVal: number;
    message: string;
}

export interface TileOccupied {
    type: "TileOccupied";
    message: string;
}
