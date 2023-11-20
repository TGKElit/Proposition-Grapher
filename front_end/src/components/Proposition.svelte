<script lang="ts">
    import { onMount } from 'svelte';
    import { proposition_display, lexer, parser, inverse_parser } from '../functions/functions';
    import type { formalization } from '../functions/types';
    export let loggedIn: boolean;
    export let searching_for_argument: boolean;
    export let proposition_id: string;
    let lexical_description: string;
    let truth_score: number;
    let formalization_editor = false;
    let formalization_string: string = "";
    let formalization_valid: string = "";
    let formalization_invalid: string = "";
    let formalization_list: formalization[] = [];
    let params = new URLSearchParams(location.search);


    onMount(() => proposition_id = "" + params.get("id"))
    

    $: [formalization_valid, formalization_invalid] = proposition_display(lexer(formalization_string));

    const fetch_proposition = () => {
        fetch("/api/proposition?id=" + proposition_id)
        .then(response => response.json())
        .then(data => {
            lexical_description = data.lexical_description;
            truth_score = data.truth_score;
        }).catch(error => {
            console.log(error);
            return [];
        })

        fetch("/api/formalization-list?id=" + proposition_id)
        .then(response => response.json())
        .then(data => {
            formalization_list = data.sort((a:formalization, b:formalization) => b.correctness_score - a.correctness_score);;
        }).catch(error => {
            console.log(error);
            return [];
        })
    }

    onMount(async () => {
        fetch_proposition();
    });

    const vote = (vote: boolean, id = proposition_id, votee = "proposition") => {
        let body = {
            vote: vote,
            votee_id: id
        }
        switch (votee) {
            case "proposition":

                fetch("/api/truth", {
                    method: "POST",
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify(body)
                }).then(response => response.json())
                .then(data => {
                    fetch_proposition();
                });
                break;
            case "formalization":
                
                fetch("/api/correctness", {
                    method: "POST",
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify(body)
                }).then(response => response.json())
                .then(data => {
                    fetch_proposition();
                });
                break;
        }
    }

    const suggest_formalization = (formalization_string: string) => {
        let body = {
            formalization_string: JSON.stringify(parser(lexer(formalization_string))),
            proposition_id: proposition_id
        }
        console.log(body);
        fetch("/api/formalization", {
            method: "POST",
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(body)
        })
    }
    
</script>
<section>
    
    
    <div class="row">
        {#if lexical_description}
        <p>{lexical_description}</p>
        {/if}
        {#if truth_score || truth_score === 0.0 }
        <p>Truth Score: {Math.round(truth_score*100)}%</p>
        {/if}
    
        {#if loggedIn}
        <button on:click={() => vote(true)}>Up</button>
        <button on:click={() => vote(false)}>Down</button>
        {/if}
    </div>

    {#if loggedIn}
    <div class="row">
        <button on:click={() => formalization_editor = !formalization_editor}>Suggest formalization</button>
            {#if formalization_editor}
        <label for="formalization-string"></label>
        <input name="formalization-string" bind:value={formalization_string}/>
                {#if {formalization_string}}
        <div class="formalization-display">
            <p>{formalization_valid}</p>
            <p class="invalid">{formalization_invalid}</p>
        </div>
                {/if}
        <button on:click={() => {suggest_formalization(formalization_string)}}>Publish</button>
            {/if}
        <button on:click={() => {searching_for_argument = true}}>Add argument</button>
    </div>
    {/if}
    <div class="row">
        <div class="formalizations">
    {#each formalization_list as formalization}
            <div>
                <p>{proposition_display(inverse_parser(JSON.parse(formalization.formalization_string)))[0]}</p>
        {#if formalization.correctness_score || formalization.correctness_score === 0.0 }
                <p>Correctness Score: {Math.round(formalization.correctness_score*100)}%</p>
        {/if}
        {#if loggedIn}
                <button on:click={() => vote(true, formalization.id, "formalization")}>Up</button>
                <button on:click={() => vote(false, formalization.id, "formalization")}>Down</button>
        {/if}
            </div>
    {/each}
        </div>
    </div>
</section>

<style>
    .row {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;
    }

    section * {
        margin: 1rem;
    }

    .invalid {
        color: red;
    }

    .formalization-display {
        display: flex;
        flex-direction: row;
    }
    .formalization-display * {
        margin: 0;
    }

    .formalizations {
        display: flex;
        flex-direction: column;
    }

    .formalizations * {
        display: flex;
        flex-direction: row;
    }
</style>