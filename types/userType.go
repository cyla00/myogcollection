package types

import (
	"fmt"
	"regexp"
	"time"

	"golang.org/x/crypto/bcrypt"
)

type User struct {
	Id        string    // json userId
	Username  string    // json username
	Email     string    // json email
	Password  string    // json password
	CreatedAt time.Time // json userCreationDate
	Active    bool      // json userActive
}

func PasswordHash(passInput string) (string, error) {
	password, err := bcrypt.GenerateFromPassword([]byte(passInput), 15)
	if err != nil {
		fmt.Println("", err)
	}
	return string(password[:]), nil
}

func CheckPasswordHash(passInput, hash string) error {
	return bcrypt.CompareHashAndPassword([]byte(hash), []byte(passInput))
}

func EmailFormatValidation(email string) bool {
	emailRegex := regexp.MustCompile(`^[a-z0-9._%+\-]+@[a-z0-9.\-]+\.[a-z]{2,4}$`)
	return emailRegex.MatchString(email)
}
