<script lang="ts">
    import PropositionThumbnail from "./PropositionThumbnail.svelte";
    import type {nodes, nodeData, relation} from "../functions/types";
    
    
    export let nodeObject: nodes;
    export let relations: relation[];
    export let visitedNodes = new Set<string>();
    

    let nodesToQueue: nodeData[] = [{node: nodeObject, x_offset: 0, y_offset: 0, steps_from_center: 0}];
    let queue: nodeData[] = [];
    while (nodesToQueue.length > 0) {
        let currentLevelNodes = nodesToQueue;
        nodesToQueue = [];
        currentLevelNodes.forEach(currentNode => {
            if (!visitedNodes.has(currentNode.node.node.id)) {
                visitedNodes.add(currentNode.node.node.id);
                queue = [...queue, ...currentLevelNodes]
                currentNode.node.premises.forEach(premise => {
                    if (!visitedNodes.has(premise.node.id)) {
                        let angle = Math.random() * 2 * Math.PI;
                        let steps_from_center = currentNode.steps_from_center + 1;
                        let x_offset = currentNode.x_offset + Math.pow(1-steps_from_center/4, 0.5) * Math.cos(angle) * window.innerWidth / 70;
                        let y_offset = currentNode.y_offset + Math.pow(1-steps_from_center/4, 0.5) * Math.sin(angle) * window.innerHeight / 70;
                        nodesToQueue = [...nodesToQueue, {
                            node: premise,
                            x_offset: x_offset,
                            y_offset: y_offset,
                            steps_from_center: steps_from_center
                        }]
                    }
                });
                currentNode.node.conclusions.forEach(conclusion => {
                    if (!visitedNodes.has(conclusion.node.id)) {
                        let angle = Math.random() * 2 * Math.PI;
                        let steps_from_center = currentNode.steps_from_center + 1;
                        let x_offset = currentNode.x_offset + Math.pow(1-steps_from_center/4, 0.5) * Math.cos(angle) * window.innerWidth / 70;
                        let y_offset = currentNode.y_offset + Math.pow(1-steps_from_center/4, 0.5) * Math.sin(angle) * window.innerHeight / 70;
                        nodesToQueue = [...nodesToQueue, {
                            node: conclusion,
                            x_offset: x_offset,
                            y_offset: y_offset,
                            steps_from_center: steps_from_center
                        }]
                    }
                });
            }
        });
    };
</script>

{#each queue as node}
<PropositionThumbnail nodeObject={node.node} x_offset={node.x_offset} y_offset={node.y_offset} steps_from_center={node.steps_from_center}/>
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