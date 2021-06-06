package main

import (
	"log"

	"github.com/youshy/Rust-vs-Go/Go/app"
)

func main() {
	a := app.App{}
	err := a.Initialize()
	if err != nil {
		log.Fatal(err)
	}
	a.Run()
}
