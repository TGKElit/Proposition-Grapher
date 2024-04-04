<script lang="ts">
    import { onMount } from "svelte";
    import { navigate } from "svelte-routing";
    let proposition_id: string;
    let params = new URLSearchParams(location.search);

    onMount(() => {
        proposition_id = "" + params.get("id");
        navigate("/proposition?id=" + proposition_id);
    });
    
</script>