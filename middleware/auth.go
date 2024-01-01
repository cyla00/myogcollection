package middleware

import (
	"log"
	"net/http"
)

func AuthMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(res http.ResponseWriter, req *http.Request) {
		// Code that run before
		log.Print("Executing middlewareOne before")
		next.ServeHTTP(res, req) // Breaks the flow if not used
		// Code that run after
		log.Print("Executing middlewareOne after")
	})
}
