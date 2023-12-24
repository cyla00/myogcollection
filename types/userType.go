package types

import "time"

type User struct {
	id        string
	username  string
	email     string
	password  string
	createdAt time.Time
	active    bool
}

func (user User) PasswordHash(passInput string) (string, error) {
	return "string element", nil
}
