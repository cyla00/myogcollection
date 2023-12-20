run:
	@go run ./main.go

dev:
	@templ generate
	@go run ./main.go

template:
	@templ generate

# database build:
# 	@go run ./build.go

# database seed:
# 	@go run ./seed.go

# database destroy:
# 	@go run ./destroy.go