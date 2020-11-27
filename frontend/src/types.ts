export enum PieceType {
	Pawn,
	Rook,
	Knight,
	Bishop,
	Queen,
	King,
};

export function piecetype_from_string(str: string): PieceType {
	switch (str) {
		case "Pawn": return PieceType.Pawn;
		case "Rook": return PieceType.Rook;
		case "Knight": return PieceType.Knight;
		case "Bishop": return PieceType.Bishop;
		case "Queen": return PieceType.Queen;
		case "King": return PieceType.King;
		default:
			return null;
	}
}

export class Piece {
	white: boolean;
	piecetype: PieceType;
	position: [number, number];
	alive: boolean;

	constructor(white: boolean, piecetype: PieceType, position: [number, number], alive = true) {
		this.white = white;
		this.piecetype = piecetype;
		this.position = position;
		this.alive = alive;
    }
}