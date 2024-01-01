package main

import (
	"fmt"
	"html"
	"log"
	"myog/database"
	"myog/middleware"
	"myog/templates/index"
	"myog/templates/login"
	"myog/templates/signup"
	"net/http"
	"os"

	"github.com/a-h/templ"
	"github.com/joho/godotenv"
	_ "github.com/lib/pq"
)

func main() {

	err := godotenv.Load()
	if err != nil {
		log.Fatal("Error loading .env file")
	}

	_, db_conn_err := database.PsqlConnection(os.Getenv("DB_USER"), os.Getenv("DB_PWD"), os.Getenv("DB_NAME"), os.Getenv("DB_HOST"))
	if db_conn_err != nil {
		log.Fatal(db_conn_err)
	}

	http.Handle("/patterns/", http.StripPrefix("/", http.FileServer(http.Dir("./patterns"))))
	http.Handle("/images/", http.StripPrefix("/", http.FileServer(http.Dir("./images"))))

	http.Handle("/", templ.Handler(index.IndexPage()))
	http.Handle("/signup", templ.Handler(signup.SignupPage()))
	http.Handle("/login", templ.Handler(login.LoginPage()))

	// API
	http.Handle("/api/"+os.Getenv("API_VERSION")+"/login", fooHandler)
	http.Handle("/api/"+os.Getenv("API_VERSION")+"/registration", fooHandler)
	http.Handle("/api/"+os.Getenv("API_VERSION")+"/upload-pattern", middleware.AuthMiddleware(fooHandler))
	http.Handle("/api/"+os.Getenv("API_VERSION")+"/delete-pattern", middleware.AuthMiddleware(fooHandler))
	http.Handle("/api/"+os.Getenv("API_VERSION")+"/modify-pattern", middleware.AuthMiddleware(fooHandler))
	http.Handle("/api/"+os.Getenv("API_VERSION")+"/comment-pattern", middleware.AuthMiddleware(fooHandler))
	http.Handle("/api/"+os.Getenv("API_VERSION")+"/modify-user", middleware.AuthMiddleware(fooHandler))
	http.Handle("/api/"+os.Getenv("API_VERSION")+"/delete-user", middleware.AuthMiddleware(fooHandler))

	http.HandleFunc("/bar", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "Hello, %q", html.EscapeString(r.URL.Path))
	})

	fmt.Printf("running on localhost:%s", os.Getenv("SERVER_PORT"))
	log.Fatal(http.ListenAndServe(":"+os.Getenv("SERVER_PORT"), nil))
}
