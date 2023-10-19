<script lang="ts">

let response = "";
let email: string;
let username: string;
let password: string;

const callApi = (email: string, username: string, password: string) => {
    fetch("/api/register", {
      method: "POST",
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        "email": email,
        "username": username,
        "password": password
      })
    })
    .then(response => response.json())
    .then(data => {
      response = data;
      console.log(response);
    }).catch(error => {
        console.log(error);
        return [];
    });
  
}
</script>

<main>
  <h1>Proposition Grapher</h1>
  <section>
    <label for="email">Email:</label>
    <input type="text" name="email" bind:value={email}>
    <label for="username">Username:</label>
    <input type="text" name="username" bind:value={username}>
    <label for="password">Password:</label>
    <input type="password" name="password" bind:value={password}>
    <button on:click={() => {callApi(email, username, password)}}>Register</button>
  </section>

  <p>{response}</p>

</main>
  
<style>
  section {
    height: 40vh;
    padding: 1rem;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    justify-content: space-between;
    background-color: #EEE;
  }
</style>
