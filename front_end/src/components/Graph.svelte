<script lang="ts">
    import PropositionThumbnail from "./PropositionThumbnail.svelte";
    import type {nodes, nodeData, relation, relationData} from "../functions/types";
    import { arctan, updateRelationData } from "../functions/functions";
    import { navigate } from "svelte-routing";

    export let nodeObject: nodes;
    export let relations: relation[];
    let visitedNodes = new Set<string>();
    let nodesToQueue: nodeData[] = [{node: nodeObject, x_offset: 0, y_offset: 0, steps_from_center: 0}];
    let queue: nodeData[] = [];
    let relationData: relationData[] = [];

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

    relationData = updateRelationData(relations, queue);

    window.addEventListener('resize', () => relationData = updateRelationData(relations, queue));
    
    
    for (let index = 1; index < 1000; index++) {
        queue.forEach(node => {
            if (node.steps_from_center !== 0) {
                let velocity: [number, number] = [0, 0];
                queue.forEach(forceNode => {
                    if (forceNode !== node) {
                        let distance: number = Math.sqrt(Math.pow(forceNode.x_offset-node.x_offset, 2) + Math.pow(forceNode.y_offset-node.y_offset, 2));
                        let direction: [number, number] = [(node.x_offset-forceNode.x_offset)/distance, (node.y_offset-forceNode.y_offset)/distance];
                        let speed: number = Math.abs(200000/Math.pow(distance, 2));
                        velocity[0] += speed * direction[0] + Math.random()*100/index;
                        velocity[1] += speed * direction[1] + Math.random()*100/index;
                        if (node.steps_from_center === 1 && forceNode.steps_from_center === 0) {
                            console.log("distance:" + distance + " speed: " + speed + " direction: " + direction);
                        }
                    }
                })
                relations.forEach(relation => {
                    if (node.node.node.id === relation.premise_id || node.node.node.id === relation.conclusion_id) {
                        relationData.forEach(relationDatum => {
                            if (relationDatum.relation === relation) {
                                let x1 = relationDatum.x1;
                                let x2 = relationDatum.x2;
                                let y1 = relationDatum.y1;
                                let y2 = relationDatum.y2;
                                let distance: number = Math.sqrt(Math.pow(x1-x2, 2) + Math.pow(y1-y2, 2));
                                let direction: [number, number] = (node.node.node.id === relation.premise_id ? [(x2-x1)/distance, (y2-y1)/distance] : [(x1-x2)/distance, (y1-y2)/distance]);
                                let speed: number = (distance-200)/10;
                                velocity[0] += speed * direction[0] * window.innerWidth / 1000;
                                velocity[1] += speed * direction[1] * window.innerHeight / 1000;
                                //console.log("distance:" + distance + " speed: " + speed + " direction: " + direction + " velocity: " + velocity);
                            }
                        })
                    }
                })
                node.x_offset += velocity[0];
                node.y_offset += velocity[1];
            }
        });
        relationData = updateRelationData(relations, queue);
        //console.log(index);
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
        z-index: 0;
    }
    line {
        stroke-width: 6;
        stroke: black;
        position: absolute;
        cursor: pointer;
    }
</style>