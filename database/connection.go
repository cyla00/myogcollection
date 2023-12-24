package database

import (
	"database/sql"
	"fmt"

	_ "github.com/lib/pq"
)

func PsqlConnection(dbUser string, dbPwd string, dbName string, dbHost string) (*sql.DB, error) {
	connStr := fmt.Sprintf("postgres://%s:%s@%s/%s?sslmode=disable", dbUser, dbPwd, dbHost, dbName)
	db, conn_err := sql.Open("postgres", connStr)
	if conn_err != nil {
		return nil, conn_err
	}
	println("database connected")
	return db, nil
}
