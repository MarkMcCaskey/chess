<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import {PieceType, Piece} from './types';

    function map_piece_type_to_svg(piecetype: PieceType, white: boolean): string {
        if(piecetype == PieceType.Pawn) {
            return white ? "Chess_plt45.svg" : "Chess_pdt45.svg";
        } else if (piecetype == PieceType.Rook) {
            return white ? "Chess_rlt45.svg" : "Chess_rdt45.svg";
        } else if (piecetype == PieceType.Knight) {
            return white ? "Chess_nlt45.svg" : "Chess_ndt45.svg";
        } else if (piecetype == PieceType.Bishop) {
            return white ? "Chess_blt45.svg" : "Chess_bdt45.svg";
        } else if (piecetype == PieceType.Queen) {
            return white ? "Chess_qlt45.svg" : "Chess_qdt45.svg";
        } else if (piecetype == PieceType.King) {
            return white ? "Chess_klt45.svg" : "Chess_kdt45.svg";
        }
    }

    function movePiece(src_loc: [number, number], dest_loc: [number, number]) {
	    dispatch('pieceMove', {
            src_loc: src_loc,
            dest_loc: dest_loc,
		});
    }

    function handleDragStart(event) {
        // TODO: show valid squares that can be moved to (probaly by firing events)
        console.log("STARTED DRAGGING! " + event.dataTransfer.dropEffect);
        event.dataTransfer.effectAllowed = "move";
        event.dataTransfer.setData("text", JSON.stringify(data.position));
        console.log(event);
    }

    function handleDragOver(event) {
         console.log("dragOver");
        event.preventDefault();
    }

    function handleDrop(event) {
        // TODO: show valid squares that can be moved to (probaly by firing events)
        console.log("DROPPED at " + x + ", " + y + "!");
        event.preventDefault();
        var data: [number, number] = JSON.parse(event.dataTransfer.getData("text"));
        console.log(data);
        console.log(event);
        movePiece(data, [x, y]);
    }

    function handleDragEnter(event) {
        // TODO: show valid squares that can be moved to (probaly by firing events)
        console.log("DRAG ENTER!");
        event.dataTransfer.effectAllowed = "move";
        console.log(event);
    }

    const dispatch = createEventDispatcher();

    export let data: Piece;
    export let x: number;
    export let y: number;
</script>

<style>
    .piece {
        display: flex;
        justify-content: center;
        width: 100%;
        height: 100%;
    }
    .empty {
        width: 70px;
        height: 70px;
    }
</style>

<main>
    <div class="empty" on:dragenter={handleDragEnter} on:drop={handleDrop} on:dragover={handleDragOver}>
        {#if data != null}
        <div class="piece" on:dragstart={handleDragStart} draggable="true">
            <img src="assets/{map_piece_type_to_svg(data.piecetype, data.white)}" alt="TODO">
        </div>
        {/if}
    </div>
</main>