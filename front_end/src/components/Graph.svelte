<script lang="ts">
    import PropositionThumbnail from "./PropositionThumbnail.svelte";
    import type {node, nodeData, relation, relationData} from "../functions/types";
    import { arctan, updateRelationData } from "../functions/functions";
    import { navigate } from "svelte-routing";


    export let nodeObject: node;
    export let relations: relation[];
    let visitedNodes = new Set<string>();
    let nodesToQueue: nodeData[] = [{node: nodeObject, x_offset: 0, y_offset: 0, steps_from_center: 0}];
    let queue: nodeData[] = [];
    let relationData: relationData[] = [];

    while (nodesToQueue.length > 0) {
        let currentLevelNodes = nodesToQueue;
        nodesToQueue = [];
        currentLevelNodes.forEach(currentNode => {
            if (!visitedNodes.has(currentNode.node.this.id)) {
                visitedNodes.add(currentNode.node.this.id);
                queue = [...queue, currentNode]
                currentNode.node.premises.forEach(premise => {
                    if (!visitedNodes.has(premise.this.id)) {
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
                    if (!visitedNodes.has(conclusion.this.id)) {
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
    
    

    const simulation = (simulationLength: number) => {
        let energy: number = 0;
        
        for (let index = 1; index <= simulationLength; index++) {
            energy = 0;
            queue.forEach(node => {

                if (node.steps_from_center !== 0) {
                    let velocity: [number, number] = [0, 0];
                    //push away nodes from each other
                    queue.forEach(forceNode => {
                        if (forceNode !== node) {
                            let distance: number = Math.sqrt(Math.pow(forceNode.x_offset-node.x_offset, 2) + Math.pow(forceNode.y_offset-node.y_offset, 2));
                            distance = distance === 0 ? 1 : distance;
                            let direction: [number, number] = [(node.x_offset-forceNode.x_offset)/distance, (node.y_offset-forceNode.y_offset)/distance];
                            let speed: number = Math.abs(nodeRepulsion/Math.pow(distance, nodeRepulsionTaper)/(node.steps_from_center+forceNode.steps_from_center));
                            velocity[0] += speed * direction[0];
                            velocity[1] += speed * direction[1];
                            energy += speed;
                        }
                    })

                    // keep edges certain length                
                    relations.forEach(relation => {
                        relationData.forEach(relationDatum => {
                            if (relationDatum.relation === relation) {
                                let x1 = relationDatum.x1;
                                let x2 = relationDatum.x2;
                                let y1 = relationDatum.y1;
                                let y2 = relationDatum.y2;
                                let distance: number = Math.sqrt(Math.pow(x1-x2, 2) + Math.pow(y1-y2, 2));
                                distance = distance === 0 ? 1 : distance;
                                if (node.node.this.id === relation.premise_id || node.node.this.id === relation.conclusion_id) {
                                    let direction: [number, number] = (node.node.this.id === relation.premise_id ? [(x2-x1)/distance, (y2-y1)/distance] : [(x1-x2)/distance, (y1-y2)/distance]);
                                    let speed: number = (distance-edgeLength/(Math.pow(relationDatum.steps_from_center, 1.2))) * edgeStrength;
                                    velocity[0] += speed * direction[0];
                                    velocity[1] += speed * direction[1];
                                    energy += Math.abs(speed);
                                }
                            }
                        });
                    })                  
        
                    // push away nodes from window border
                    velocity[0] += - borderRepulsion * node.x_offset / Math.max(Math.pow(((window.innerWidth * simulationLength / index / 2 + node.x_offset) * (window.innerWidth * simulationLength / index / 2 - node.x_offset)), borderRepulsionTaper), 1);
                    velocity[1] += - borderRepulsion * node.y_offset / Math.max(Math.pow(((window.innerHeight * simulationLength / index / 2 + node.y_offset) * (window.innerHeight * simulationLength / index / 2 - node.y_offset)), borderRepulsionTaper), 1);
                    
                    // cap velocity
                    velocity[0] = Math.abs(velocity[0]) > window.innerWidth / 4 ? Math.sign(velocity[0]) * window.innerWidth / 4 : velocity[0];
                    velocity[1] = Math.abs(velocity[1]) > window.innerHeight / 4 ? Math.sign(velocity[1]) * window.innerHeight / 4 : velocity[1];
                    
                    // apply velocity
                    node.x_offset += velocity[0];
                    node.y_offset += velocity[1];
                    
                    // cap offset
                    node.x_offset = Math.abs(node.x_offset) >= window.innerWidth / 2 ? Math.sign(node.x_offset) * window.innerWidth / 2 : node.x_offset;
                    node.y_offset = Math.abs(node.y_offset) >= window.innerHeight / 2 ? Math.sign(node.y_offset) * window.innerHeight / 2 : node.y_offset;
                    
                }
            });
            relationData = updateRelationData(relations, queue);
            queue = queue;
        }
        finalEnergy = energy;
    }

    const simulatedAnnealing = () => {
        let currentQueue = queue;
        let bestSolutionQueue = queue;
        let bestSolutionEnergy = 1000;
        for (let temperature = startTemperature; temperature > 0; temperature--) {
            let previousRelationData = relationData;
            let previousQueue = currentQueue;
            let previousEnergy = finalEnergy;
            currentQueue.forEach(node => {
                if (node.steps_from_center !== 0) {
                    node.x_offset += (Math.random()-0.5)*200;
                    node.y_offset += (Math.random()-0.5)*200;
                }
            });
            queue = currentQueue;
            simulation(500);
            if (finalEnergy < bestSolutionEnergy) {
                bestSolutionQueue = currentQueue;
                bestSolutionEnergy = finalEnergy;
            }
            let changeProbability = finalEnergy < previousEnergy ? 1 : Math.exp((finalEnergy - previousEnergy)/temperature);
            if (Math.random() > changeProbability) {
                currentQueue = previousQueue;
                relationData = previousRelationData;
            }
        }
        queue = bestSolutionQueue;
        simulation(500);
    }

    let startTemperature = 100;
    let edgeLength = 300;
    let edgeStrength = 0.1;
    let nodeRepulsion = 200000000;
    let nodeRepulsionTaper = 3;
    let borderRepulsion = 1600000000;
    let borderRepulsionTaper = 2;

    let finalEnergy: number = 0;
    $: finalEnergy;

    simulatedAnnealing();

</script>



{#each queue as node}
<PropositionThumbnail nodeObject={node.node} x_offset={node.x_offset} y_offset={node.y_offset} steps_from_center={node.steps_from_center}/>
{/each}

{#if relations}
<svg>
    {#each relationData as relation}
    <line
        style="--stroke-width: {7 - relation.steps_from_center}"
        on:click={() => navigate("/relation?id=" + relation.relation.id)} 
        on:keypress={() => navigate("relation")}
        tabindex=0
        role="link"
        x1={relation.x1}
        y1={relation.y1}
        x2={relation.x2}
        y2={relation.y2}/>
    <line
        style="--stroke-width: {7 - relation.steps_from_center}"
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
        style="--stroke-width: {7 - relation.steps_from_center}" 
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
        stroke-width: var(--stroke-width);
        stroke: black;
        position: absolute;
        cursor: pointer;
    }
</style>