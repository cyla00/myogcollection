const template = document.createElement("template")
template.innerHtml = `
<header>
            <div>
                <a href="/" class="">index</a>
                <a href="/registration" class="">registration</a>
                <a href="/login" class="">login</a>
            </div>
            
        </header>
`

class Header extends HTMLElement {
    constructor(){
        super()
        const shadow = this.attachShadow({mode: "closed"})
        shadow.append(template.content.cloneNode(true))
    }
}

customElements.define("header-component", Header)