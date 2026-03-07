package main

import (
	"ZCrypt/internal/CrypticEngine"
	"fmt"
)

func main() {
	fmt.Println("meow !!")

	record := CrypticEngine.CrypticRecord{
		SchemaVersion: 999,
		UserID:        "45567889",
		TemplateID:    "753468",
		TemplateType:  "Fusion",
		TemplateVer:   1,

		DEK: []byte("..whatifthekeyisalsohellowthere!"),
	}

	text := []byte("hellow there !")

	record.Ciphertext = CrypticEngine.Encrypt(record, text)

	fmt.Println(record.Ciphertext)
}
