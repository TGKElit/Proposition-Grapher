<script lang="ts">

    import { Link, navigate } from "svelte-routing";
    import type { nodes } from '../functions/types';

    export let nodeObject: nodes;
    export let x_offset: number;
    export let y_offset: number;
    export let steps_from_center: number;
    
    
    //let distance_from_center: number = Math.sqrt(Math.pow(x_offset, 2) + Math.pow(y_offset, 2));

    let scale_down = (1 - steps_from_center / 5);
</script>

<button style="--x-offset: {x_offset}rem; --y-offset: {y_offset}rem; --scale-down: {scale_down}" on:click={() => {navigate("/proposition?id=" + nodeObject.node.id)}}>
        <p>{nodeObject.node.lexical_description}</p>
</button>

<style>
    button {
        border-radius: 1rem;
        max-width: calc(20rem * var(--scale-down));
        min-height: calc(6rem * var(--scale-down));
        max-height: calc(6rem * var(--scale-down));
        position: absolute;
        left: calc(50% + var(--x-offset));
        top: calc(50% + var(--y-offset));
        translate: -50% -50%;
        background-color: lightgray;
        overflow: hidden;
    }

    p {
        overflow: hidden;
        width: 100%;
        max-height: 100%;
        margin: 0;
        text-align: center;
    }
</style>