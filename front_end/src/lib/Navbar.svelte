<script lang="ts">
  import callApi from "../utils/api";
  let authText: string = "Logout";

</script>

<nav>
  <h1>Proposition Grapher</h1>
    <button on:click={() => callApi("", "", "", "logout")}>{authText}</button>
</nav>


<style>
  
</style>