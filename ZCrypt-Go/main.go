package main

import (
	"ZCrypt/internal/CrypticEngine"
	"crypto/rand"
	"fmt"
	"io"
	"os"
)

func main() {
	FilePath := os.Args[1]

	record := CrypticEngine.CrypticRecord{
		SchemaVersion: 999,
		UserID:        "45567889",
		TemplateID:    "753468",
		TemplateType:  "Fusion",
		TemplateVer:   1,
	}

	Data, err := os.ReadFile(FilePath)
	if err != nil {
		fmt.Println("check the damned file ffs", err)
		os.Exit(1)
	}

	KEK := make([]byte, CrypticEngine.DEKSize)
	if _, err := io.ReadFull(rand.Reader, KEK); err != nil {
		fmt.Println(err)
	}

	record.Ciphertext = CrypticEngine.Encrypt(&record, KEK, Data)

	fmt.Println("Nounce: ", record.TemplateNonce)
	fmt.Println("Ciphertext: ", record.Ciphertext)
	fmt.Println("DEK: ", record.DEK)
	fmt.Println("WrapNounce: ", record.WrapNonce)

	fmt.Println(string(CrypticEngine.Decrypt(&record, KEK)))
}
