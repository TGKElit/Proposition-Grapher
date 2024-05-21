<script lang="ts">
    import { Link } from "svelte-routing";
    import proposition from "./Proposition.svelte";
    import { add_argument } from "../functions/functions";
    export let searching_for_argument: boolean;
    export let proposition_id: string;
    let search_query: string;
    let search_result: proposition[];
    let search_box: string = "none";
    let argument_response: string = "";


    const search = (query: string) => {
        fetch("/api/search?query=" + query)
        .then(response => response.json())
        .then(data => {
            search_result = data;
            console.log(data);
        }).catch(error => {
            console.log(error);
            return [];
        })
        search_box = "box";
        console.log(search_box);
    }

    const close = () => {
        search_box = "none";
        console.log(search_box);
    }

</script>

<form class="search" on:submit|preventDefault={() => search(search_query)}>
    <input type="text" bind:value={search_query}>
    <button type="submit">Search</button>
</form>
{#if search_result}
<section class="search-result" style="--search-box: {search_box}">
    {#if search_result.length === 0}
    <p>There are no matches</p>
    {/if}
    <button class="close-button" on:click={() => close()}>X</button>
    {#each search_result as proposition}
    <div class="search-item">
        <p>{proposition.lexical_description}</p>
        <div>
            <Link to="/proposition-redirect?id={proposition.id}">GOTO</Link>
        {#if searching_for_argument && proposition_id !== proposition.id}
            <button on:click={async () => argument_response = await add_argument(proposition_id, proposition.id)}>Add as argument</button>
        {/if}
        </div>
    </div>
    {/each}
    <p>{argument_response}</p>
</section>
{/if}

<style>
    .search-item {
        width: 100%;
        display: flex;
        justify-content: space-between;
        gap: 1rem;
    }
    .search {
        display: flex;
        flex-direction: row;
        margin: 1rem;
        padding: 0;
        align-items: center;
        gap: 1rem;
    }

    .search-result {
        width: fit-content;
        height: fit-content;
        margin: 0;
        position: absolute;
        left: 0;
        top: 4rem;
        display: var(--search-box);
    }

    .close-button {
        left: 1rem;
    }
</style>