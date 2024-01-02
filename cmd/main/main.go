package main

import (
	"fmt"
	"log"
	"myog/api"
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
	uploadPatternHandler := http.HandlerFunc(api.PatternUploadRoute)
	deletePatternHandler := http.HandlerFunc(api.PatternDeleteRoute)
	modifyPatternHandler := http.HandlerFunc(api.PatternModifyRoute)
	commentPatternHandler := http.HandlerFunc(api.PatternCommentRoute)
	modifyUserHandler := http.HandlerFunc(api.UserModifyRoute)
	deleteUserHandler := http.HandlerFunc(api.UserDeleteRoute)

	http.HandleFunc("/api/"+os.Getenv("API_VERSION")+"/login", api.LoginRoute)
	http.HandleFunc("/api/"+os.Getenv("API_VERSION")+"/registration", api.RegistrationRoute)
	http.Handle("/api/"+os.Getenv("API_VERSION")+"/upload-pattern", middleware.AuthMiddleware(uploadPatternHandler))
	http.Handle("/api/"+os.Getenv("API_VERSION")+"/delete-pattern", middleware.AuthMiddleware(deletePatternHandler))
	http.Handle("/api/"+os.Getenv("API_VERSION")+"/modify-pattern", middleware.AuthMiddleware(modifyPatternHandler))

	http.Handle("/api/"+os.Getenv("API_VERSION")+"/comment-pattern", middleware.AuthMiddleware(commentPatternHandler))
	http.Handle("/api/"+os.Getenv("API_VERSION")+"/modify-user", middleware.AuthMiddleware(modifyUserHandler))
	http.Handle("/api/"+os.Getenv("API_VERSION")+"/delete-user", middleware.AuthMiddleware(deleteUserHandler))

	fmt.Printf("running on localhost:%s", os.Getenv("SERVER_PORT"))
	log.Fatal(http.ListenAndServe(":"+os.Getenv("SERVER_PORT"), nil))
}
