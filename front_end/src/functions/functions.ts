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