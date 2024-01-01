package api

import (
	"fmt"
	"html"
	"net/http"
)

func LoginRoute(res http.ResponseWriter, req *http.Request) {
	fmt.Fprintf(res, "Hello, %q", html.EscapeString(req.URL.Path))
}
