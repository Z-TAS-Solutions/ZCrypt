package main

import (
	"ZCrypt/internal/CrypticEngine"
	"fmt"
	"os"
)

func main() {
	text := []byte(os.Args[1])

	record := CrypticEngine.CrypticRecord{
		SchemaVersion: 999,
		UserID:        "45567889",
		TemplateID:    "753468",
		TemplateType:  "Fusion",
		TemplateVer:   1,

		DEK: []byte("..whatifthekeyisalsohellowthere!"),
	}

	record.Ciphertext = CrypticEngine.Encrypt(record, text)

	fmt.Println(record.Ciphertext)

	fmt.Println(string(CrypticEngine.Decrypt(record.DEK, record, record.Ciphertext)))
}
