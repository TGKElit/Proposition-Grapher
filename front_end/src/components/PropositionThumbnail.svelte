<script lang="ts">

    import { Link, navigate } from "svelte-routing";
    import type { nodes } from '../functions/types';

    export let nodeObject: nodes;
    export let x_offset: number;
    export let y_offset: number;
    export let steps_from_center: number;
    

    let scale_down = (1 - steps_from_center / 7);
</script>

<button style="--x-offset: {x_offset}px; --y-offset: {y_offset}px; --scale-down: {scale_down}" on:click={() => {navigate("/proposition?id=" + nodeObject.node.id)}}>
        <p>{nodeObject.node.lexical_description}</p>
</button>

<style>
    button {
        border-radius: 1rem;
        min-height: calc(6rem * var(--scale-down));
        max-height: calc(6rem * var(--scale-down));
        position: absolute;
        left: calc(50% + var(--x-offset));
        top: calc(50% + var(--y-offset) + 4rem);
        translate: -50% -50%;
        background-color: lightgray;
        overflow: hidden;
        z-index: 1;
    }

    p {
        overflow: hidden;
        width: 100%;
        max-height: 100%;
        margin: 0;
        text-align: center;
        font-size: calc(1rem * var(--scale-down));
    }
</style>