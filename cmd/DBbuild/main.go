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

	usr_table_err := db.QueryRow(`CREATE TABLE IF NOT EXISTS users (
		id VARCHAR(50) PRIMARY KEY,
		username VARCHAR(100) NOT NULL,
		email VARCHAR(100) NOT NULL,
		password VARCHAR(500) NOT NULL,
		created_at DATE NULL,
		active BOOLEAN DEFAULT false NOT NULL
	)`)
	if usr_table_err != nil {
		fmt.Println("", usr_table_err.Err())
	}

	pattern_table_err := db.QueryRow(`CREATE TABLE IF NOT EXISTS patterns (
		id VARCHAR(50) PRIMARY KEY,
		owner_id VARCHAR(100) NOT NULL,
		pattern_path VARCHAR(100) NOT NULL,
		tags BOOLEAN DEFAULT false NOT NULL,
		created_at DATE NULL
	)`)
	if pattern_table_err != nil {
		fmt.Println("", pattern_table_err.Err())
	}

	comments_table_err := db.QueryRow(`CREATE TABLE IF NOT EXISTS comments (
		id VARCHAR(100) PRIMARY KEY,
		owner_id VARCHAR(100) NOT NULL,
		pattern_id VARCHAR(100) NOT NULL,
		rating FLOAT DEFAULT 0.0 NOT NULL,
		created_at DATE NULL
	)`)
	if comments_table_err != nil {
		fmt.Println("", comments_table_err.Err())
	}

	println("database tables created or present")
}
