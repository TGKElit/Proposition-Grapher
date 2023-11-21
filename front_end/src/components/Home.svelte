<script lang="ts">
    import { onMount } from 'svelte';
    import Graph from './Graph.svelte';
    import type {nodes, relation} from '../functions/types';

    let nodeObject: nodes;
    let relations: relation[] = [];

    onMount(async () => {
        await fetch("/api/graph?depth=3", {
            method: "GET",
            headers: {
                'Content-Type': 'application/json'
            },
        })
        .then(response => response.json())
        .then(data => {
            nodeObject = data.nodes;
            relations = data.relations;
            console.log(data);
        }).catch(error => {
            console.log(error);
            return [];
        })
        
    });
</script>

<section>
    {#if nodeObject}
    <Graph nodeObject={nodeObject} relations={relations}/>
    {/if}
</section>

<style>
    section {
        width: 100vw;
        height: calc(100vh - 4rem);
        background-color: unset;
        margin: 0;
    }
    
</style>