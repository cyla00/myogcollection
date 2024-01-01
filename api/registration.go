package api

import (
	"fmt"
	"html"
	"net/http"
)

func RegistrationRoute(res http.ResponseWriter, req *http.Request) {
	fmt.Fprintf(res, "Hello, %q", html.EscapeString(req.URL.Path))
}
