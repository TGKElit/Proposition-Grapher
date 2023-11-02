<script lang="ts">
    import { onMount } from 'svelte';
    let lexical_description: String;
    let truth_score: Number;
    
    onMount(async () => {
        //console.log(window.location.search)
        let params = new URLSearchParams(window.location.search)
        
        let proposition_id = params.get("id");
        
        await fetch("/api/proposition?id=" + proposition_id, {
            method: "GET",
            headers: {
                'Content-Type': 'text/html'
            }
        })
        .then(response => response.json())
        .then(data => {
            lexical_description = data.lexical_description;
            truth_score = data.truth_score;
            console.log(data);
        }).catch(error => {
            console.log(error);
            return [];
        })
    });
</script>



<section>
    <p>{lexical_description}</p>
    {#if truth_score}
    <p>{truth_score}</p>
    {/if}
    <button>Up</button>
    <button>Down</button>
</section>   


<style>
    section {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
    }
</style>