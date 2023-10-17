<script lang="ts">
  import Counter from './lib/Counter.svelte'

 let response = "";
 let inputValue: String;
  const callApi = (name: String) => {
    let query = "";
    if (name) {
      query = "?name=" + name;
    } 
    fetch("/api/hello" + query)
      .then(response => response.json())
      .then(data => {
        response = data;
      }).catch(error => {
        console.log(error);
        return [];
      });
    
  }
</script>

<main>
  
  <h1>Proposition Grapher</h1>

  <input type="text" bind:value={inputValue}>
  <button on:click={() => {callApi(inputValue)}}></button>

  <p>{response}</p>

</main>

<style>
  
</style>
