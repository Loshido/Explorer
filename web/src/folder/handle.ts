const main = document.querySelector('main') as HTMLElement | null
const header = document.querySelector('header') as HTMLElement | null
if(!main) throw new Error("Mauvaise page")
if(!header) throw new Error("Mauvaise page")

function path(nom: string, href: string) {
    const a = document.createElement('a')
    a.className = "py-1 px-2 hover:bg-black/15"
    a.innerText = nom
    a.href = href

    if(header.children.length === 1) {
        header.appendChild(a)
    } else {
        header.append(
            '/',
            a
        )
    }
}
function file(nom: string, href: string) {
    const a = document.createElement('a')
    a.href = href;
    a.className = "flex gap-1 py-2 px-2.5 hover:bg-black/10"
    a.innerHTML = `
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" 
            stroke="currentColor" stroke-width="2" stroke-linecap="round" 
            stroke-linejoin="round">
            <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"/>
            <path d="M14 2v4a2 2 0 0 0 2 2h4"/>
        </svg>
        <p>
            ${ nom }
        </p>
    `

    main.appendChild(a)
}
function folder(nom: string, href: string) {
    const a = document.createElement('a')
    a.href = href;
    a.className = "flex gap-1 py-2 px-2.5 hover:bg-black/10 font-medium"
    a.innerHTML = `
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" 
            stroke="currentColor" stroke-width="2" stroke-linecap="round" 
            stroke-linejoin="round">
            <path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 
            2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/>
        </svg>
        <p>
            ${ nom }
        </p>
    `

    main.appendChild(a)
}

const url = location.pathname
url
    .split('/')
    .filter(path => path.length > 0)
    .map((path, i, array) => [path, '/' + array.slice(0, i + 1).join('/')])
    .forEach(([nom, href]) => {
        path(nom, href)
    })

const setup = async () => {
    const response = await fetch('/_payload' + url, {
        credentials: 'include'
    })
    if(!response.ok) return

    const data = await response.json() as {
        path: string,
        files: [boolean, string][]
    }

    data.files.forEach(entity => {
        if(entity[0]) {
            file(
                entity[1].split('/').filter(e => e.length > 0).at(-1), 
                '/' + entity[1]
            )
        } else {
            folder(
                entity[1].split('/').filter(e => e.length > 0).at(-1), 
                '/' + entity[1]
            )
        }
    })
}

setup()