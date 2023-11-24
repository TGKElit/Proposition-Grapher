

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

export type nodeData = {
    node: nodes,
    x_offset: number,
    y_offset: number,
    steps_from_center: number
}
export enum validity {
    valid = "Valid",
    invalid = "Invalid",
    antivalid = "Antivalid"
}

export type relation = {
    id: string,
    premise_id: string,
    conclusion_id: string,
    correlation_score: number,
    validity: validity
}

export type relationData = {
    relation: relation,
    x1: number,
    y1: number,
    x2: number,
    y2: number
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