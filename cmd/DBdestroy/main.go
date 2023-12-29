package main

import (
	"database/sql"
	"fmt"
	"log"
	"os"

	"github.com/joho/godotenv"
	_ "github.com/lib/pq"
)

func main() {
	err := godotenv.Load()
	if err != nil {
		log.Fatal("Error loading .env file")
	}

	connStr := fmt.Sprintf("postgres://%s:%s@%s/%s?sslmode=disable", os.Getenv("DB_USER"), os.Getenv("DB_PWD"), os.Getenv("DB_HOST"), os.Getenv("DB_NAME"))
	db, err := sql.Open("postgres", connStr)
	if err != nil {
		log.Fatal(err)
	}

	drop_users_err := db.QueryRow("DROP TABLE users")
	if drop_users_err != nil {
		fmt.Println("", drop_users_err.Err())
	}

	drop_patterns_err := db.QueryRow("DROP TABLE patterns")
	if drop_patterns_err != nil {
		fmt.Println("", drop_patterns_err.Err())
	}

	drop_comments_err := db.QueryRow("DROP TABLE comments")
	if drop_comments_err != nil {
		fmt.Println("", drop_comments_err.Err())
	}

	println("database destroyed")
}
