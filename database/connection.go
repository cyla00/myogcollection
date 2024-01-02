package database

import (
	"database/sql"
	"fmt"

	_ "github.com/lib/pq"
)

var DB *sql.DB

func PsqlConnection(dbUser string, dbPwd string, dbName string, dbHost string) (*sql.DB, error) {
	connStr := fmt.Sprintf("postgres://%s:%s@%s/%s?sslmode=disable", dbUser, dbPwd, dbHost, dbName)
	return sql.Open("postgres", connStr)
}
