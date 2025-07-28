async function updateSentence() {
    try {
        let response = await fetch("/api/get_random_sentence", {
            method: 'POST',
            headers: {'Content-Type': 'text/plain'},
            body: "",
        });
        
        if (!response.ok) {
            throw new Error(`Response status: ${response.status}`);
        }

        const random_sentence = await response.text();
        console.log(random_sentence);
        const sentence_display = document.getElementById("sentence-box");
        sentence_display.textContent = random_sentence;
    } catch (error) {
    console.error(error.message);
    }
}

updateSentence();

document.getElementById("userInput").addEventListener("keydown", async function(event) {
    if (event.key === "Enter") {
        const userInput = event.target.value;
        try {
            let response = await fetch("/api/submit_user_input", {
                method: 'POST',
                headers: {'Content-Type': 'text/plain'},
                body: userInput,
            });
            if (!response.ok) {
                throw new Error(`Response status: ${response.status}`);
            }
            // Optionally handle the response
            const result = await response.text();
            console.log(result);
        } catch (error) {
            console.error(error.message);
        }
    }
});