const input = document.querySelector('input') as HTMLInputElement | null
if(!input) throw new Error('Mauvaise page')

input.addEventListener('input', () => {
    if(input.value.at(5) !== '-' && input.value.length >= 5) {
        input.value = input.value.slice(0, 5) + '-' + input.value.slice(6)
    }
    if(input.value.at(11) !== '-' && input.value.length >= 11) {
        input.value = input.value.slice(0, 11) + '-' + input.value.slice(12)
    }
})

input.addEventListener('keydown', async (event) => {
    if(event.key === "Backspace" && (input.value.length === 12 || input.value.length === 6)) {
        event.preventDefault()
        input.value = input.value.slice(0, -2)
    }
    if(event.key === "Enter" && input.value.length === 17) {
        const response = await fetch('/login', {
            method: "POST",
            body: input.value,
        })

        if(response.ok) {
            input.classList.add("success")

            setTimeout(() => {
                input.classList.remove("success")
                location.reload()
            }, 500);
        } else {
            input.classList.add("error")

            setTimeout(() => {
                input.classList.remove("error")
            }, 500);
        }
    }
})