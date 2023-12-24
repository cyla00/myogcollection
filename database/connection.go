package database

import (
	"database/sql"
	"errors"
	"fmt"

	_ "github.com/lib/pq"
)

func PsqlConnection(dbUser string, dbPwd string, dbName string, dbHost string) (*sql.DB, error) {
	connStr := fmt.Sprintf("postgres://%s:%s@%s/%s", dbUser, dbPwd, dbHost, dbName)
	db, conn_err := sql.Open("postgres", connStr)
	if conn_err != nil {
		return nil, errors.New("database connection error")
	}
	return db, nil
}
