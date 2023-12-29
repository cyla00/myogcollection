package types

import "time"

type User struct {
	Id        string
	Username  string
	Email     string
	Password  string
	CreatedAt time.Time
	Active    bool
}

func (user User) PasswordHash(passInput string) (string, error) {
	return "string element", nil
}
