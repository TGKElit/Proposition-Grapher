<script lang="ts">

import { Route, Router, Link } from "svelte-routing";
import Home from "./components/Home.svelte";
import NewProposition from "./components/NewProposition.svelte";
import Proposition from "./components/Proposition.svelte";
import { onMount } from "svelte";
import Search from "./components/Search.svelte";
  import Relation from "./components/Relation.svelte";

let response = "";
let email: string;
let username: string;
let password: string;
let loggedIn = false;
let page: view;
let searching_for_argument = false;
let proposition_id: string = "";
type view = "login" | "register";
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
        loggedIn = data;
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

onMount(() => {
  fetch("/api/auth-status", {
        method: "GET",
        headers: {
          'Content-Type': 'application/json'
        },
      })
      .then(response => response.json())
      .then(data => {
        response = data;
        loggedIn = data;
      }).catch(error => {
          console.log(error);
          return [];
      });
})

</script>

<main>
  <Router>
    <nav>
      <Link to="/">Home</Link>
      <Search bind:searching_for_argument bind:proposition_id/>
      {#if loggedIn}
      <Link to="/new-proposition">New Proposition</Link>
      <button on:click={() => {callApi(email, username, password, "logout")}}>Logout</button>
      {:else}
      <Link to="/login" on:click={() => page = "login"}>Login</Link>
      <Link to="/login" on:click={() => page = "register"}>Register</Link>
      {/if}
    </nav>
  
    <Route path="/" component={Home} />
    <Route path="/login">
      <section>
        <label for="email">Email:</label>
        <input type="text" name="email" bind:value={email}>
        {#if page == "register"}
        <label for="username">Username:</label>
        <input type="text" name="username" bind:value={username}>
        {/if}
        <label for="password">Password:</label>
        <input type="password" name="password" bind:value={password}>
        {#if page == "register"}
        <Link to="/" on:click={() => {callApi(email, username, password, "register")}}>Register</Link>
        {:else}
        <Link to="/" on:click={() => {callApi(email, username, password, "login")}}>Login</Link>
        {/if}
      </section>
    </Route>
    <Route path="/new-proposition" component={NewProposition} />
    <Route path="/proposition" >
      <Proposition {loggedIn} bind:searching_for_argument bind:proposition_id={proposition_id}/>
    </Route>
    <Route path="/relation" component={Relation}/>
  </Router>

</main>
  
<style>
  nav {
    position: relative;
    height: 4rem;
    width: 100vw;
    top: 0;
    left: 0;
    background-color: #eee;
    display: flex;
    justify-content: space-around;
    z-index: 2;
  }

</style>
