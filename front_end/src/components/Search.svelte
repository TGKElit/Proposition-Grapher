<script lang="ts">
    import { Link } from "svelte-routing";
    import proposition from "./Proposition.svelte";
    import { add_argument } from "../functions/functions";
    export let searching_for_argument: boolean;
    export let proposition_id: string;
    let search_query: string;
    let search_result: proposition[];


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
    }

</script>

<section class="search">
    <input type="text" bind:value={search_query}>
    <button on:click={() => search(search_query)}>Search</button>
</section>
{#if search_result}
<section class="search-result">
    {#each search_result as proposition}
    <div class="search-item">
        <p>{proposition.lexical_description}</p>
        <div>
            <Link to="/proposition?id={proposition.id}">GOTO</Link>
        {#if searching_for_argument}
            <button on:click={() => add_argument( proposition_id, proposition.id)}>Add as argument</button>
        {/if}
        </div>
    </div>
    {/each}
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
    }
</style>