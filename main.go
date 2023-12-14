package main

import (
	"log"

	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/template/html/v2"
)

func main() {
	engine := html.New("./templates", ".html")
	app := fiber.New(fiber.Config{AppName: "MyogCollection v0.0.1", Views: engine})

	app.Static("/static", "./static")
	app.Static("/", "./projects")

	app.Get("/", func(ctx *fiber.Ctx) error {
		return ctx.Render("index", fiber.Map{})
	})

	app.Get("/registration", func(ctx *fiber.Ctx) error {
		return ctx.Render("registration", fiber.Map{})
	})

	app.Get("/login", func(ctx *fiber.Ctx) error {
		return ctx.Render("login", fiber.Map{})
	})

	log.Fatal(app.Listen(":3000"))
}
