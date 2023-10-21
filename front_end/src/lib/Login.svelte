<script lang="ts">
    import callApi from "../utils/api";


    let response = "hmm";
    let email: string;
    let username: string;
    let password: string;
</script>

<section>
    <label for="email">Email:</label>
    <input type="text" name="email" bind:value={email}>
    <label for="username">Username:</label>
    <input type="text" name="username" bind:value={username}>
    <label for="password">Password:</label>
    <input type="password" name="password" bind:value={password}>
    <button on:click={() => {response = callApi(email, username, password, "register")}}>Register</button>
    <button on:click={() => {response = callApi(email, username, password, "login")}}>Login</button>
</section>
<p>{response}</p>

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