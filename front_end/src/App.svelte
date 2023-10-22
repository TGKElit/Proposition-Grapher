<script lang="ts">

let response = "";
let email: string;
let username: string;
let password: string;
let bgColor = "#EEE";

type endpoint = "register" | "login" | "logout";

const callApi = (email: string, username: string, password: string, endpoint: endpoint) => {
    let body;
    switch (endpoint) {
      case "register":
        body = {
          "email": email,
          "username": username,
          "password": password
        };
        break;
      case "login":
        body = {
          "email": email,
          "password": password
        };
        break;
      case "logout":
        body = {}
      break;
    }
    console.log(body);
    fetch("/api/" + endpoint, {
      method: "POST",
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(body)
    })
    .then(response => response.json())
    .then(data => {
      response = data;
      console.log(response);
      fetch("/api/auth-status", {
      method: "GET",
      headers: {
        'Content-Type': 'application/json'
      },
    })
      .then(response => response.json())
      .then(data => {
        response = data;
        if (response) {
          bgColor = "#2E2";
        } else {
          bgColor = "#E22"
        }
        console.log(response);
      }).catch(error => {
          console.log(error);
          return [];
      });
    }).catch(error => {
        console.log(error);
        return [];
    })
    
    
}

</script>

<main>
  <h1>Proposition Grapher</h1>
  <section style='--bgColor:{bgColor}'>
    <label for="email">Email:</label>
    <input type="text" name="email" bind:value={email}>
    <label for="username">Username:</label>
    <input type="text" name="username" bind:value={username}>
    <label for="password">Password:</label>
    <input type="password" name="password" bind:value={password}>
    <button on:click={() => {callApi(email, username, password, "register")}}>Register</button>
    <button on:click={() => {callApi(email, username, password, "login")}}>Login</button>
    <button on:click={() => {callApi(email, username, password, "logout")}}>Logout</button>
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
    background-color: var(--bgColor);
  }
</style>
