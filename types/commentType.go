package types

import "time"

type Comments struct {
	Id         string
	Owner_id   string
	Pattern_id string
	Comment    string
	Created_at time.Time
}
