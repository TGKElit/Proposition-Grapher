<script lang="ts">
    import type { node, nodeData } from '../functions/types';
    import { tweened } from 'svelte/motion';

    export let nodeData: nodeData;
    export let handle_click;
    
    let scale_down = (1 - nodeData.steps_from_center / 5);

    const x_offset = tweened(nodeData.x_offset);
    const y_offset = tweened(nodeData.y_offset);

    $: nodeData, x_offset.set(nodeData.x_offset), y_offset.set(nodeData.y_offset), scale_down = (1 - nodeData.steps_from_center / 5);
</script>

<button
    class={nodeData.steps_from_center === 0 ? "center" : ""}
    style="--x-offset: {$x_offset}px; --y-offset: {$y_offset}px; --scale-down: {scale_down}"
    on:click={() => {handle_click(nodeData.node)}}
>
        <p>{nodeData.node.this.lexical_description}</p>
</button>

<style>
    button {
        padding: unset;
        border-radius: 100%;
        min-width: calc(6rem * var(--scale-down));;
        min-height: calc(6rem * var(--scale-down));
        position: absolute;
        left: calc(50% + var(--x-offset));
        top: calc(50% + var(--y-offset) + 4rem);
        translate: -50% -50%;
        background-color: lightgray;
        overflow: hidden;
        z-index: 1;
    }

    button:hover, .center {
        border-radius: 1rem;
        min-width: 6rem;
        min-height: 6rem;
        padding: 1rem;
    }

    p {
        display: none;
    }
    button:hover p, .center p {
        display: block;
        overflow: hidden;
        width: 100%;
        max-height: 100%;
        /*margin: 0;*/
        text-align: center;
        /*font-size: calc(1rem * var(--scale-down));*/
    }
</style>