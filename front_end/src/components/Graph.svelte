<script lang="ts">
    import PropositionThumbnail from "./PropositionThumbnail.svelte";
    import type {nodes, relation} from "../functions/types";
    
    
    export let nodeObject: nodes;
    export let relations: relation[];
    export let visitedNodes = new Set<string>();
    export let x_offset: number;
    export let y_offset: number; 
    export let steps_from_center: number;
    let angle = Math.random() * 2 * Math.PI;
    if (steps_from_center != 0) {
        x_offset += Math.pow(1-steps_from_center/4, 0.5) * Math.cos(angle) * window.innerWidth / 70
        y_offset += Math.pow(1-steps_from_center/4, 0.5) * Math.sin(angle) * window.innerHeight / 70
    }

    let nodesToQueue: nodes[] = [nodeObject];
    let queue: nodes[] = [];
    /*while (nodesToQueue.length > 0) {
        let currentLevelNodes = nodesToQueue;
        nodesToQueue = [];
        currentLevelNodes.forEach(currentNode => {
            if (!visitedNodes.has(currentNode.node.id)) {
                visitedNodes.add(currentNode.node.id);
                queue = [...queue, ...currentLevelNodes]
                currentNode.premises.forEach(premise => {
                    if (!visitedNodes.has(premise.node.id)) {
                        nodesToQueue = [...nodesToQueue, premise]
                    }
                });
                currentNode.conclusions.forEach(conclusion => {
                    if (!visitedNodes.has(conclusion.node.id)) {
                        nodesToQueue = [...nodesToQueue, conclusion]
                    }
                });
            }
        });
    };*/
</script>



<PropositionThumbnail {nodeObject} {x_offset} {y_offset} {steps_from_center}/>
{@html `<!--${visitedNodes.add(nodeObject.node.id)}-->`}


{#each nodeObject.premises as premise}
    {#if !visitedNodes.has(premise.node.id)}
        
    {@html `<!-- ${visitedNodes.add(premise.node.id)} -->`}
<svelte:self nodeObject={premise} {visitedNodes} {x_offset} {y_offset} steps_from_center={steps_from_center + 1}/>
    {/if}
{/each}

{#each nodeObject.conclusions as conclusion}
    
    {#if !visitedNodes.has(conclusion.node.id)}
        
    {@html `<!-- ${visitedNodes.add(conclusion.node.id)} -->`}
<svelte:self nodeObject={conclusion} {visitedNodes} {x_offset} {y_offset} steps_from_center={steps_from_center + 1}/>
    {/if}
{/each}

{#if relations}
<svg>
    {#each relations as relation}
    <line x1={Math.random()*window.innerWidth} y1={Math.random()*window.innerHeight} x2={Math.random()*window.innerHeight} y2={Math.random()*100}/>
    {/each}
</svg>
{/if}


<style>
    svg {
        position: absolute;
        width: 100%;
        height: calc(100% - 4rem);
        z-index: -1;
        margin: -1rem;
    }
    line {
        stroke-width: 2;
        stroke: black;
    }
</style>