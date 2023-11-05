<script lang="ts">
  import { Link, navigate } from "svelte-routing";

let proposition_id: String;
let description: String;
const postProposition = (description: String) => {
    let body = {
        lexical_description: description
    }

    fetch("/api/proposition", {
        method: "POST",
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(body)
    })
    .then(response => response.json())
    .then(data => {
        proposition_id = data;
        navigate("/proposition?id=" + proposition_id);
        console.log(data);
    }).catch(error => {
        console.log(error);
        return [];
    })
}
</script>

<section>
    <label for="description">Description:</label>
    <textarea name="description" bind:value={description}/>
    
    <button on:click={() => {postProposition(description)}}>Post</button>
</section>

<style>

</style>