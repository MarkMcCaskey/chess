<script lang="ts">
	import { onMount } from 'svelte';
	import RenderPiece from './RenderPiece.svelte';
	import {PieceType, Piece, piecetype_from_string} from './types';


	let boardState = [
		// set up white's pawns
		new Piece(true, PieceType.Pawn,    [1, 2]), new Piece(true, PieceType.Pawn,    [2, 2]),
		new Piece(true, PieceType.Pawn,    [3, 2]), new Piece(true, PieceType.Pawn,    [4, 2]),
		new Piece(true, PieceType.Pawn,    [5, 2]), new Piece(true, PieceType.Pawn,    [6, 2]),
		new Piece(true, PieceType.Pawn,    [7, 2]), new Piece(true, PieceType.Pawn,    [8, 2]),
		// set up white's back row
		new Piece(true, PieceType.Rook,    [1, 1]), new Piece(true, PieceType.Rook,    [8, 1]),
		new Piece(true, PieceType.Knight,  [2, 1]), new Piece(true, PieceType.Knight,  [7, 1]),
		new Piece(true, PieceType.Bishop,  [3, 1]), new Piece(true, PieceType.Bishop,  [6, 1]),
		new Piece(true, PieceType.Queen,   [4, 1]), new Piece(true, PieceType.King,    [5, 1]),
		// set up black's pawns
		new Piece(false, PieceType.Pawn,   [1, 7]), new Piece(false, PieceType.Pawn,   [2, 7]),
		new Piece(false, PieceType.Pawn,   [3, 7]), new Piece(false, PieceType.Pawn,   [4, 7]),
		new Piece(false, PieceType.Pawn,   [5, 7]), new Piece(false, PieceType.Pawn,   [6, 7]),
		new Piece(false, PieceType.Pawn,   [7, 7]), new Piece(false, PieceType.Pawn,   [8, 7]),
		// set up blacks's back row
		new Piece(false, PieceType.Rook,   [1, 8]), new Piece(false, PieceType.Rook,   [8, 8]),
		new Piece(false, PieceType.Knight, [2, 8]), new Piece(false, PieceType.Knight, [7, 8]),
		new Piece(false, PieceType.Bishop, [3, 8]), new Piece(false, PieceType.Bishop, [6, 8]),
		new Piece(false, PieceType.Queen,  [4, 8]), new Piece(false, PieceType.King,   [5, 8]),
	];
	// map from board position x, y to index in boardState
	let pieceMap = new Array(8).fill(null).map(() => new Array(8).fill(null));
	boardState.forEach((piece, i) => {
		pieceMap[8 - piece.position[1]][piece.position[0] - 1] = i + 1;
	});
	function get_piece_at(x: number, y: number): number {
		return pieceMap[x][y];
	}
	function get_piece_to_render(boardStateIndex: number): Piece {
		let piece = boardState[boardStateIndex];
		if (typeof piece == 'undefined') {
			return null;
		}
		if (!piece.alive) {
			return null;
		}
		return piece;
	}

	function updateBoardState(boardStateJson: Object) {
		console.log(boardStateJson);
		let newBoardState = [];
		for (var i = 0; i < boardStateJson.pieces.length; ++i) {
			let json_piece = boardStateJson.pieces[i];
			let piecetype = piecetype_from_string(json_piece.piecetype);

			let piece = new Piece(json_piece.white, piecetype, json_piece.position, json_piece.alive);
			newBoardState.push(piece);
		}
		boardState = newBoardState;
		pieceMap = new Array(8).fill(null).map(() => new Array(8).fill(null));
		boardState.forEach((piece, i) => {
			if (piece.position != null) {
				pieceMap[8 - piece.position[1]][piece.position[0] - 1] = i;
			}
		});
	}

	function handleMessage(message) {
		console.log("Handling message from the server! " + message);
		let msg = JSON.parse(message.data);
		if (msg.hasOwnProperty("Welcome")) {
			window.player_id = msg.Welcome.id_token;
			console.log("Player ID: " + window.player_id);
		} else if (msg.hasOwnProperty("BoardState")) {
			updateBoardState(msg.BoardState);
		} else {
			console.error("Unrecognized message from the server!");
			console.log(message);
		}
	}

	function handlePieceMove(message) {
		console.log("Handling piece move: " + message);
		let msg = JSON.stringify({"MovePiece": {
					"id_token": window.player_id,
					"prev_location": message.detail.src_loc,
					"location": message.detail.dest_loc
				}});
				console.log("Sending message: " + msg);
		window.websocket.send(msg);
	}

	onMount(async () => {
		let ws = new WebSocket("ws://localhost:8080");
		ws.onmessage = handleMessage;
		ws.onclose = (msg) => console.log("Web socket closed!" + msg);
		ws.onopen = (msg) => {
			console.log("Web socket opened!" + msg);
			ws.send("\"Connect\"");
		 };
		ws.onerror = (msg) => console.error("Web socket closed!" + msg);
		console.log("Connected to server!");
		console.log("Connected to server!");
		window.websocket = ws;
	});
</script>

<!-- shout out to https://codepen.io/jeansarlon/pen/WpZNda for CSS styles -->
<style>
	.board {
 	 	border: 49px solid #462921;
 	 	width: 560px;
 	 	margin: 0 auto;
 	 	display: grid;
 	 	grid-gap: 0;
 	 	grid-template-columns: repeat(8, 70px);
 	 	grid-template-rows: repeat(8, 70px);
 	 	grid-auto-flow: row;
	}
	.square {
 	 	font-size: 150%;
 	 	background-color: #b5915f;
 	 	color: #000;
 	 	text-align: center;
	}
	.square:nth-child(-2n+8), 
	.square:nth-child(8) ~ div:nth-child(-2n+15), 
	.square:nth-child(17) ~ div:nth-child(-2n+24), 
	.square:nth-child(24) ~ div:nth-child(-2n+31), 
	.square:nth-child(33) ~ div:nth-child(-2n+40), 
	.square:nth-child(40) ~ div:nth-child(-2n+47), 
	.square:nth-child(48) ~ div:nth-child(-2n+56), 
	.square:nth-child(56) ~ div:nth-child(-2n+63){
  		background-color: #441a03;
  		color: #fff;
	}
</style>

<main>
	<p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>
	<div class="board">
		{#each pieceMap as columns, i}
			{#each columns as item, j}
				<div class="square">
					<RenderPiece data={get_piece_to_render(item)} y={7-i + 1} x={j + 1} on:pieceMove={handlePieceMove} />
				</div>
			{/each}
		{/each}
	</div>
</main>