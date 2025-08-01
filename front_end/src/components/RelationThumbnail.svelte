<script>
  import { navigate } from "svelte-routing";
  import { arctan } from "../functions/functions";
  import { tweened } from "svelte/motion";

  export let relation;

    const x1 = tweened(relation.x1);
    const x2 = tweened(relation.x2);
    const y1 = tweened(relation.y1);
    const y2 = tweened(relation.y2);

    $: relation, x1.set(relation.x1), x2.set(relation.x2), y1.set(relation.y1), y2.set(relation.y2);

</script>

<line
    style="--stroke-width: {7 - relation.steps_from_center}"
    on:click={() => navigate("/relation?id=" + relation.relation.id)} 
    on:keypress={() => navigate("relation")}
    tabindex=0
    role="link"
    x1={$x1}
    y1={$y1}
    x2={$x2}
    y2={$y2}
/>
<line
    style="--stroke-width: {7 - relation.steps_from_center}"
    on:click={() => navigate("/relation?id=" + relation.relation.id)} 
    on:keypress={() => navigate("relation")}
    tabindex=0
    role="link"
    x1={($x1 + $x2) / 2}
    y1={($y1 + $y2)/ 2}
    x2={($x1 + $x2) / 2 + Math.cos(-arctan($y2-$y1,$x2-$x1)-1/4*Math.PI)*16}
    y2={($y1 + $y2)/ 2 + Math.sin(-arctan($y2-$y1,$x2-$x1)-1/4*Math.PI)*16}
/>
<line
    style="--stroke-width: {7 - relation.steps_from_center}" 
    on:click={() => navigate("/relation?id=" + relation.relation.id)} 
    on:keypress={() => navigate("relation")}
    tabindex=0
    role="link"
    x1={($x1 + $x2) / 2}
    y1={($y1 + $y2)/ 2}
    x2={($x1 + $x2) / 2 + Math.cos(-arctan($y2-$y1,$x2-$x1)-3/4*Math.PI)*16}
    y2={($y1 + $y2)/ 2 + Math.sin(-arctan($y2-$y1,$x2-$x1)-3/4*Math.PI)*16}
/>

<style>
    line {
        stroke-width: var(--stroke-width);
        stroke: black;
        position: absolute;
        cursor: pointer;
    }
</style>