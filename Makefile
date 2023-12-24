run:
	@go run ./cmd/main/main.go

dev:
	@templ generate
	@go run ./cmd/main/main.go

template:
	@templ generate

database build:
	@go run ./cmd/DBbuild/main.go

database seed:
	@go run ./cmd/DBseed/main.go

database destroy:
	@go run ./cmd/DBdestroy/main.go