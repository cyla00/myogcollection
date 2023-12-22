package main

import (
	"log"
	"myog/templates/index"
	"myog/templates/login"
	"myog/templates/signup"
	"net/http"
	"os"
	"fmt"

	"github.com/a-h/templ"
	"github.com/joho/godotenv"
	_ "github.com/lib/pq"
)

func main() {

	err := godotenv.Load()
	if err != nil {
		log.Fatal("Error loading .env file")
	}

	http.Handle("/", templ.Handler(index.IndexPage()))
	http.Handle("/signup", templ.Handler(signup.SignupPage()))
	http.Handle("/login", templ.Handler(login.LoginPage()))

	// db, db_conn_err := database.PsqlConnection(os.Getenv("DB_USER"), os.Getenv("DB_PWD"), os.Getenv("DB_NAME"), os.Getenv("DB_HOST"))
	// if db_conn_err != nil {
	// 	log.Fatal(db_conn_err)
	// }
	build_err := database.Build(db)
	if build_err != nil {
		log.Fatal(build_err)
	}
	fmt.Printf("running on localhost:%s", os.Getenv("SERVER_PORT"))
	log.Fatal(http.ListenAndServe(":"+os.Getenv("SERVER_PORT"), nil))
}
