<script lang="ts">
    import { onMount } from 'svelte';
    export let loggedIn: boolean;
    let lexical_description: String;
    let truth_score: number;
    let formalization_editor = false;
    let formalization_string: String = "";
    let formalization_valid: String = "";
    let formalization_invalid: String = "";
    let params = new URLSearchParams(window.location.search)
    let proposition_id = params.get("id");


    interface token {
        
        connective: connective,
        atom: atom,
    }
    enum connective {
        and = "and",
        or = "or",
        implies = "implies",
        iff = "iff"
    }
    
    type atom = "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z";

    type proposition = {
        uuid: String | null
        negated: boolean
        proposition: atom | {
            left_operand: proposition
            main_connective: connective
            right_operand: proposition
        } | string
    }

    const is_enclosed = (tokens: { token: String; type: string; }[]): boolean => {
        if(tokens[0].token === "(" && tokens[tokens.length-1].token === ")") {
            let parenthesis_balance = 0;
            for (let n = 0; n < tokens.length-1; n++) {
                switch (tokens[n].token) {
                        case "(":
                            parenthesis_balance++;
                            break;
                        case ")": 
                            parenthesis_balance--;
                            break;
                }
                if (parenthesis_balance === 0) {
                    return false;
                }
            }
            return true;
        }
        return false;
    }

    const parser = (tokens: { token: String; type: string; }[]): proposition => {
        let proposition: proposition;
        let uuid = null;
        let negated = false;
        let parenthesis_balance = 0;
        let main_connective: connective | null = null;
        let connective_position = 0;
        
        if (is_enclosed(tokens)) {
            console.log("enclosed");
            tokens.splice(0,1);
            tokens.splice(-1,1);
        }
            
        if (tokens[0].token === "!") {
            console.log("ptential_negation");
            console.log(tokens.slice(1,tokens.length));
            if (is_enclosed(tokens.slice(1,tokens.length))) {
                console.log("negation");
                negated = true;
                tokens.splice(0,1);
                tokens.splice(0,1);
                tokens.splice(-1,1);
            } else if (tokens[1].type === "atom" && tokens.length <= 3) {
                negated = true;
            }
        }
        for (let n = 0; n < tokens.length; n++) {
            switch (tokens[n].type) {
                case "reserved":
                    switch (tokens[n].token) {
                        case "(":
                            parenthesis_balance++;
                            break;
                        case ")": 
                            parenthesis_balance--;
                            break;
                        case "&":
                            if (parenthesis_balance === 0) {
                                main_connective = connective.and;
                                connective_position = n;
                            }
                            break;
                        case "|":
                            if (parenthesis_balance === 0) {
                                main_connective = connective.or;
                                connective_position = n;
                            }
                            break;
                        case "->":
                            if (parenthesis_balance === 0) {
                                main_connective = connective.implies;
                                connective_position = n;
                            }
                            break;
                        case "<->":
                            if (parenthesis_balance === 0) {
                                main_connective = connective.iff;
                                connective_position = n;
                            }
                            break;
                    }
                    break;
                case "atom":
                    if (parenthesis_balance === 0 && (tokens.length <= 2 || (negated && tokens.length <= 3))) {
                        if (tokens.length > n+1) {
                            if (tokens[n+1].type === "uuid") {
                                uuid = tokens[n+1].token;
                            }
                        }
                        return proposition = {
                            uuid: uuid,
                            negated: negated,
                            proposition: tokens[n].token as atom
                        }
                    }
                    break;
            }
            if(main_connective !== null) {
                return proposition = {
                    uuid: uuid,
                    negated: negated,
                    proposition: {
                        left_operand: parser(tokens.slice(0, connective_position)),
                        main_connective: main_connective,
                        right_operand: parser(tokens.slice(connective_position+1, tokens.length))
                    }
                }
            }
        }
        throw new Error("Incorecctly formatted formalization");
        
        
    }

    const token_expressions = [
    { pattern: /^[ \n\t]+/, type: null },
    { pattern: /^\(/, type: "reserved" },
    { pattern: /^\)/, type: "reserved" },
    { pattern: /^\!/, type: "reserved" },
    { pattern: /^&/, type: "reserved" },
    { pattern: /^\|/, type: "reserved" },
    { pattern: /^<->/, type: "reserved" },
    { pattern: /^->/, type: "reserved" },
    { pattern: /^:[1-9a-fA-F]{8}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{12}/, type: "uuid" },
    { pattern: /^[A-Z]/, type: "atom" }
];

    const lexer = (string: String) => {
        let tokens = [];
        while (string.length > 0) {
            let match;
            let token;
            for (let n = 0; n < token_expressions.length; n++) {
                let {pattern, type} = token_expressions[n];
                match = string.match(pattern);
                if (match) {
                    token = match[0];
                    string = string.slice(token.length);
                    if (type) {
                        tokens.push({token, type});
                    }
                    break;
                }
            }
            if (!token) {
                token = string;
                let type = "invalid";
                tokens.push({token, type});
                break;
            }
        }
        return tokens;
    }

    const proposition_display = (tokens: { token: String; type: string; }[]) => {
        let valid_string = "";
        let invalid_string = "";
        tokens.forEach(token => {
            switch (token.type) {
                case "atom":
                    valid_string += token.token;
                    break;
                case "reserved":
                    switch (token.token) {
                        case "(":
                            valid_string += "(";
                            break;
                        case ")":
                            valid_string += ")";
                            break;
                        case "->": 
                            valid_string += '\u2192';
                            break;
                        case "&":
                            valid_string += '\u2227';
                            break;
                        case "|":
                            valid_string += '\u2228';
                            break;
                        case "<->":
                            valid_string += '\u2194';
                            break;
                        case "!":
                            valid_string += '\u00AC'
                            break;
                    }
                    break;
                case "invalid":
                    invalid_string += token.token;
                default:
                    break;
            }
        });
        return [valid_string, invalid_string];
    }

    $: [formalization_valid, formalization_invalid] = proposition_display(lexer(formalization_string));

    
    const to_proposition = (string: String) => {
        let tokens = lexer(string);

    }

    const fetch_proposition = () => {
        fetch("/api/proposition?id=" + proposition_id)
        .then(response => response.json())
        .then(data => {
            lexical_description = data.lexical_description;
            truth_score = data.truth_score;
            console.log(data);
        }).catch(error => {
            console.log(error);
            return [];
        })
    }

    onMount(async () => {
        
        await fetch_proposition();
    });

    const vote = (vote: boolean) => {
        let body = {
            vote: vote,
            votee_id: proposition_id
        }
        fetch("/api/truth", {
            method: "POST",
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(body)
        }).then(response => response.json())
        .then(data => {
            console.log(data);
            fetch_proposition();
        });
    }


    const suggest_formalization = () => {
        let body = {
            formalization_string: formalization_string,
            proposition_id: proposition_id
        }
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
    {#if lexical_description}
    <p>{lexical_description}</p>
    {/if}
    {#if truth_score || truth_score === 0.0 }
    <p>Truth Score: {Math.round(truth_score*100)}%</p>
    {/if}

    {#if loggedIn}
    <button on:click={() => vote(true)}>Up</button>
    <button on:click={() => vote(false)}>Down</button>

    <button on:click={() => formalization_editor = true}>Suggest formalization</button>
        {#if formalization_editor}
    <label for="formalization-string"></label>
    <textarea name="formalization-string" bind:value={formalization_string}/>
    {#if {formalization_string}}
    <div class="formalization-display">
        <p>{formalization_valid}</p>
        <p class="invalid">{formalization_invalid}</p>
    </div>
        
    {/if}
    <button on:click={() => {console.log(parser(lexer(formalization_string)))}}>Publish</button>
        {/if}
    {/if}

</section>   


<style>
    section {
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
</style>