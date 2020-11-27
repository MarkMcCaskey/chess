export enum PieceType {
	Pawn,
	Rook,
	Knight,
	Bishop,
	Queen,
	King,
};

export class Piece {
	white: boolean;
	piecetype: PieceType;
	position: [number, number];
	alive: boolean;

	constructor(white: boolean, piecetype: PieceType, position: [number, number]) {
		this.white = white;
		this.piecetype = piecetype;
		this.position = position;
		this.alive = true;
    }
}