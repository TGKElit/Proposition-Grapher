<script lang="ts">
    import { onMount } from 'svelte';
    import Graph from './Graph.svelte';
    import type { node, relation } from '../functions/types';
    import { navigate } from 'svelte-routing';

    let nodeObject: node;
    let relations: relation[] = [];
    let params = new URLSearchParams(location.search);
    let depth = 3;

    onMount(() => {
        params.get("id") ? get_graph(depth, "" + params.get("id")) : get_graph(depth, ""); 
    });

    const get_graph = (depth:number, id: string, newNode: node | void) => {

        let center: string = "";
        if (id) {
            center = "&id=" + id;
        }
        if (!newNode) {
            fetch("/api/graph?depth=" + depth + center, {
                method: "GET",
                headers: {
                    'Content-Type': 'application/json'
                },
            })
            .then(response => response.json())
            .then(data => {
                nodeObject = data.node;
                relations = data.relations;
            }).catch(error => {
                console.log(error);
                return [];
            })
        } else {
            let nodeList: node["this"][] = [nodeObject.this];
            let premiseConclusionList: node[] = nodeObject.premises.concat(nodeObject.conclusions);
            for (let i = 0; i < depth; i++) {
                let newPremiseConclusionList: node[] = [];
                premiseConclusionList.forEach(node => {
                    if (!nodeList.some(element => element.id === node.this.id)) {
                        nodeList.push(node.this);
                        newPremiseConclusionList.push(...node.premises.concat(node.conclusions));
                    }
                });
                premiseConclusionList = newPremiseConclusionList;
            }

            let newNodeObject: node = {this: newNode.this, premises: [], conclusions: []};
            let nodeObjectList: node[] = [newNodeObject];
            let newRelations: relation[] = [];
            for (let i = 0; i < depth; i++) {
                nodeObjectList.forEach(node => {
                    relations.filter(relation => relation.premise_id === node.this.id).forEach(relation => {
                        node.conclusions.push(...nodeList
                            .filter(node => node.id === relation.conclusion_id)
                            .map(item => ({this: item, premises: [], conclusions: []}))
                        );
                        !newRelations.some(element => element.id === relation.id) && newRelations.push(relation);
                    });
                    relations.filter(relation => relation.conclusion_id === node.this.id).forEach(relation => {
                        node.premises.push(...nodeList
                            .filter(node => node.id === relation.premise_id)
                            .map(item => ({this: item, premises: [], conclusions: []}))
                        );
                        !newRelations.some(element => element.id === relation.id) && newRelations.push(relation);
                    });
                });
                let newNodeObjectList: node[] = [];
                nodeObjectList.forEach(node => {
                    newNodeObjectList.push(...node.premises.concat(node.conclusions));
                });
                nodeObjectList = newNodeObjectList;
            }
            relations = newRelations;
            nodeObject = newNodeObject;
        }
    }

    const handle_thumbnail_click = (newNodeObject: node) => {
        newNodeObject === nodeObject ? navigate("/proposition?id=" + nodeObject.this.id) : get_graph(depth, newNodeObject.this.id, newNodeObject);
    }

</script>

<section>
    {#if nodeObject}
    <Graph bind:nodeObject={nodeObject} relations={relations} handle_thumbnail_click={handle_thumbnail_click}/>
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