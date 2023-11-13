<script lang="ts">
    import { onMount } from 'svelte';
    import { Link } from 'svelte-routing';
    let lexical_description: String;
    let id: String;
    
    onMount(async () => {
        console.log("Mounting");
        await fetch("/api/graph?depth=3", {
            method: "GET",
            headers: {
                'Content-Type': 'application/json'
            },
        })
        .then(response => response.json())
        .then(data => {
            lexical_description = data.node.lexical_description;
            id = data.node.id;
            console.log(data);
        }).catch(error => {
            console.log(error);
            return [];
        })
    });
</script>

<section>
    <Link to="/proposition?id={id}">
        <p>{lexical_description}</p>
    </Link>
</section>

<style>
    section {
        border-radius: 1rem;
    }
</style>