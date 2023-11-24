import type { atom, proposition } from './types';
import { connective } from './types';

export const add_argument = (proposition_id: string, argument_id: string) => {
    let body = {
        premise_id: argument_id,
        conclusion_id: proposition_id
    }

    fetch("/api/relation", {
        method: "POST",
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(body)
    })
    .then(response => response.json())
    .then(data => {
        console.log(data);
    }).catch(error => {
        console.log(error);
        return [];
    })
}

// parser

const is_enclosed = (tokens: { token: string; type: string; }[]): boolean => {
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

export const parser = (tokens: { token: string; type: string; }[]): proposition => {
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
        if (is_enclosed(tokens.slice(1))) {
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
                            uuid = tokens[n+1].token.slice(1);
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

export const inverse_parser = (proposition: proposition): { token: string; type: string }[] => {
    const tokens: { token: string; type: string }[] = [];

    const add_token = (t: string, type: string) => {
        tokens.push({ token: t, type: type });
    };

    const process_proposition = (proposition: proposition) => {
        if (typeof proposition.proposition === 'string') {
            // Atomic proposition
            if (proposition.negated) {
                add_token("!", "reserved");
            }
            add_token(proposition.proposition, "atom");
            if (proposition.uuid) {
                add_token(proposition.uuid, "uuid");
            }
        } else {
            // Compound proposition
            let connective: string = "";
            add_token("(", "reserved");
            process_proposition(proposition.proposition.left_operand);
            switch (proposition.proposition.main_connective) {
                case "And":
                    connective = "&"
                    break;
                case "Or":
                    connective = "|"
                    break;
                case "Implies":
                    connective = "->"
                    break;
                case "Iff":
                    connective = "<->"
                    break;
            }
            add_token(connective, "reserved");
            process_proposition(proposition.proposition.right_operand);
            add_token(")", "reserved");
        }
    };

    process_proposition(proposition);
    if (tokens.length > 2) {
        tokens.shift();
        tokens.pop();
    }

    return tokens;
};

const token_expressions = [
{ pattern: /^[ \n\t]+/, type: null },
{ pattern: /^\(/, type: "reserved" },
{ pattern: /^\)/, type: "reserved" },
{ pattern: /^\!/, type: "reserved" },
{ pattern: /^&/, type: "reserved" },
{ pattern: /^\|/, type: "reserved" },
{ pattern: /^<->/, type: "reserved" },
{ pattern: /^->/, type: "reserved" },
{ pattern: /^:[0-9a-fA-F]{8}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{12}/, type: "uuid" },
{ pattern: /^[A-Z]/, type: "atom" }
];

export const lexer = (string: string) => {
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

export const proposition_display = (tokens: { token: string; type: string; }[]) => {
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

export const arctan = (x: number, y: number) => {
    let result = Math.atan(y/x);
    if (x < 0) {
        if (y > 0) {
            result += Math.PI;
        } else {
            result -= Math.PI;
        }

    }
    return result;
}
