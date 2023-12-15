class customHeader extends HTMLElement {
    constructor() {
      super()
    }

    connectedCallback(){
        this.innerHTML = `
            <footer>
                footer
            </footer>
        `
    }
  }

  customElements.define('custom-header', customHeader)