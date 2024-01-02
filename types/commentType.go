package types

import "time"

type Comments struct {
	Id         string    // json commentId
	Owner_id   string    // json commentOwnerId
	Pattern_id string    // json commentPatternId
	Comment    string    // json commentValue
	Created_at time.Time // json commentCreationDate
}
