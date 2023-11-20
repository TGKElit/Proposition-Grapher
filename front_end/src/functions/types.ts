export type nodes = {
    node: {
        id: string,
        lexical_description: string,
        profile_id: string,
        truth_score: number
    },
    premises: nodes[],
    conclusions: nodes[]
};
export type relation = {
    id: string,
    premise_id: string,
    conclusion_id: string,
    correlation_score: number
}

export enum connective {
    and = "And",
    or = "Or",
    implies = "Implies",
    iff = "Iff"
}

export type atom = "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z";

export type proposition = {
    uuid: string | null
    negated: boolean
    proposition: atom | {
        left_operand: proposition
        main_connective: connective
        right_operand: proposition
    }
}

export type formalization = {
    id: string,
    correctness_score: number,
    formalization_string: string
}