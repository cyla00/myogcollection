class customHeader extends HTMLElement {
    constructor() {
      super()
    }

    connectedCallback(){
        this.innerHTML = `
            <header class="bg">
                <h1 class="text-blue">header</h1>
                <button hx-get="/clicked">get</button>
            </header>
        `
    }
  }

  customElements.define('custom-header', customHeader)