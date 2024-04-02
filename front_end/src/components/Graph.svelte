<script lang="ts">
    import PropositionThumbnail from "./PropositionThumbnail.svelte";
    import type {nodes, nodeData, relation, relationData} from "../functions/types";
    import { arctan } from "../functions/functions";
    import { navigate } from "svelte-routing";
    
    
    export let nodeObject: nodes;
    export let relations: relation[];
    let visitedNodes = new Set<string>();
    

    let nodesToQueue: nodeData[] = [{node: nodeObject, x_offset: 0, y_offset: 0, steps_from_center: 0}];
    let queue: nodeData[] = [];
    while (nodesToQueue.length > 0) {
        let currentLevelNodes = nodesToQueue;
        nodesToQueue = [];
        currentLevelNodes.forEach(currentNode => {
            if (!visitedNodes.has(currentNode.node.node.id)) {
                visitedNodes.add(currentNode.node.node.id);
                queue = [...queue, currentNode]
                currentNode.node.premises.forEach(premise => {
                    if (!visitedNodes.has(premise.node.id)) {
                        let angle = Math.random() * 2 * Math.PI;
                        let steps_from_center = currentNode.steps_from_center + 1;
                        let x_offset = currentNode.x_offset + Math.pow(1-steps_from_center/4, 0.4) * Math.cos(angle) * window.innerWidth / 4;
                        let y_offset = currentNode.y_offset + Math.pow(1-steps_from_center/4, 0.4) * Math.sin(angle) * window.innerHeight / 4;
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
                        let x_offset = currentNode.x_offset + Math.pow(1-steps_from_center/4, 0.4) * Math.cos(angle) * window.innerWidth / 4;
                        let y_offset = currentNode.y_offset + Math.pow(1-steps_from_center/4, 0.4) * Math.sin(angle) * window.innerHeight / 4;
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

    let relationData: relationData[] = [];
    if (relations) {
        relations.forEach(relation => {
            let x1: number = 0;
            let y1: number = 0;
            let x2: number = 0;
            let y2: number = 0;
            queue.forEach(node => {
                if (relation.premise_id === node.node.node.id) {
                    x1 = node.x_offset + window.innerWidth/2;
                    y1 = node.y_offset + window.innerHeight/2;
                }
                if (relation.conclusion_id === node.node.node.id) {
                    x2 = node.x_offset + window.innerWidth/2;
                    y2 = node.y_offset + window.innerHeight/2;
                }
            });
            relationData = [...relationData, {relation: relation, x1: x1, y1: y1, x2: x2, y2: y2}];
        });
    }

</script>

{#each queue as node}
<PropositionThumbnail nodeObject={node.node} x_offset={node.x_offset} y_offset={node.y_offset} steps_from_center={node.steps_from_center}/>
{/each}

{#if relations}
<svg>
    {#each relationData as relation}
    <line 
        on:click={() => navigate("/relation?id=" + relation.relation.id)} 
        on:keypress={() => navigate("relation")}
        tabindex=0
        role="link"
        x1={relation.x1}
        y1={relation.y1}
        x2={relation.x2}
        y2={relation.y2}/>
    <line 
        on:click={() => navigate("/relation?id=" + relation.relation.id)} 
        on:keypress={() => navigate("relation")}
        tabindex=0
        role="link"
        x1={(relation.x1 + relation.x2) / 2}
        y1={(relation.y1 + relation.y2)/ 2}
        x2={(relation.x1 + relation.x2) / 2 + Math.cos(-arctan(relation.y2-relation.y1,relation.x2-relation.x1)-1/4*Math.PI)*16}
        y2={(relation.y1 + relation.y2)/ 2 + Math.sin(-arctan(relation.y2-relation.y1,relation.x2-relation.x1)-1/4*Math.PI)*16}
    />
    <line
        on:click={() => navigate("/relation?id=" + relation.relation.id)} 
        on:keypress={() => navigate("relation")}
        tabindex=0
        role="link"
        x1={(relation.x1 + relation.x2) / 2}
        y1={(relation.y1 + relation.y2)/ 2}
        x2={(relation.x1 + relation.x2) / 2 + Math.cos(-arctan(relation.y2-relation.y1,relation.x2-relation.x1)-3/4*Math.PI)*16}
        y2={(relation.y1 + relation.y2)/ 2 + Math.sin(-arctan(relation.y2-relation.y1,relation.x2-relation.x1)-3/4*Math.PI)*16}
    />
    {/each}
</svg>
{/if}


<style>
    svg {
        position: absolute;
        top: 4rem;
        left: 0;
        width: 100%;
        height: calc(100% - 4rem);
        z-index: -1;
    }
    line {
        stroke-width: 6;
        stroke: black;
        position: absolute;
        cursor: pointer;
    }
</style>