<script lang="ts">
    import { onMount } from 'svelte';
    import { proposition_display, lexer, parser, inverse_parser, fetch_formalization_list, fetch_proposition } from '../functions/functions';
    import type { formalization } from '../functions/types';
    import { navigate } from 'svelte-routing';
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
    let post_formalization_response: string = "";
    let test: string = "successful";

    onMount(() => {
        proposition_id = "" + params.get("id");
    });
    

    $: [formalization_valid, formalization_invalid] = proposition_display(lexer(formalization_string));

    onMount(async () => {
        [lexical_description, truth_score, formalization_list] = await fetch_proposition(proposition_id);
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
                .then(async (data) => {
                    [lexical_description, truth_score, formalization_list] = await fetch_proposition(proposition_id);
                }).catch(error => {
                    console.log("Vote proposition error: " + error);
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
                .then(async (data) => {
                    formalization_list = await fetch_formalization_list(proposition_id);
                }).catch(error => {
                    console.log("Vote formalization error: " + error);
                });
                break;
        }
    }

    const suggest_formalization = (formalization_string: string) => {
        post_formalization_response = "";
        try {
            let body = {
                formalization_string: JSON.stringify(parser(lexer(formalization_string))),
                proposition_id: proposition_id
            }
            fetch("/api/formalization", {
                method: "POST",
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(body)
            }).then(response => response.json())
            .then(async (data) => {
                post_formalization_response = data;
                [lexical_description, truth_score, formalization_list] = await fetch_proposition(proposition_id);
                test = "successful";
            }).catch(error => {
                test = "unsuccessful";
                post_formalization_response = error;
                return [];
            })
        } catch (error) {
            test = "unsuccessful";
            error instanceof Error ? post_formalization_response = error.message : post_formalization_response = "Unknown error"; 
        }
    }
    
</script>
<section>
    <button class="close-button" on:click={() => navigate("/?id=" + proposition_id)}>X</button>
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
        <p class="{test}">{post_formalization_response}</p>
            {/if}
        <button on:click={() => {searching_for_argument = !searching_for_argument}}>{searching_for_argument ? "Cancel" : "Add argument"}</button>
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

    .successful {
        color: green;
    }

    .unsuccessful {
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