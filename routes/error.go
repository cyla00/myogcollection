package routes

import (
	"myog/templates/errorPage"
	"net/http"

	"github.com/a-h/templ"
)

func ErrorRoute(res http.ResponseWriter, req *http.Request, status int) {
	res.WriteHeader(status)
	if status == http.StatusNotFound {
		templ.Handler(errorPage.ErrPage())
	}
}
