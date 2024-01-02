package api

import (
	"fmt"
	"html"
	"net/http"

	_ "github.com/lib/pq"
)

func LoginRoute(res http.ResponseWriter, req *http.Request) {
	if req.Method != "POST" {
		res.WriteHeader(http.StatusMethodNotAllowed)
		return
	}

	rows, err := DB.Query("SELECT * FROM users")
	if err != nil {
		fmt.Println("", err)
	}
	fmt.Println("", rows)

	fmt.Fprintf(res, "Hello, %q", html.EscapeString(req.URL.Path))
}
