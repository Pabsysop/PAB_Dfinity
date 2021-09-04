import React from "react"
import ReactDOM from "react-dom"

import App from "./src/views/App"

/**
 * @dfinity/agent requires this. Can be removed once it's fixed
 */
window.global = window

ReactDOM.render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
    document.getElementById("app"),
)
