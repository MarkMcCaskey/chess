<script lang="ts">
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

    function handleDragStart(event) {
        // TODO: show valid squares that can be moved to (probaly by firing events)
        console.log("STARTED DRAGGING! " + event.dataTransfer.dropEffect);
        event.dataTransfer.effectAllowed = "move";
        event.dataTransfer.setData("text", data);
        console.log(event);
    }

    function handleDrop(event) {
        // TODO: show valid squares that can be moved to (probaly by firing events)
        console.log("DROPPED!");
        //event.preventDefault();
        //var data = event.dataTransfer.getData("text");
        //console.log(data);
        console.log(event);
    }

    function handleDragEnter(event) {
        // TODO: show valid squares that can be moved to (probaly by firing events)
        console.log("DRAG ENTER!");
        event.dataTransfer.effectAllowed = "move";
        console.log(event);
    }


    export let data: [PieceType, boolean];
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
    <div class="empty" on:dragenter={handleDragEnter}>
        {#if data != null}
        <div class="piece" on:dragstart={handleDragStart} on:drop={handleDrop}>
            <img src="assets/{map_piece_type_to_svg(data[0], data[1])}" alt="TODO">
        </div>
        {/if}
    </div>
</main>