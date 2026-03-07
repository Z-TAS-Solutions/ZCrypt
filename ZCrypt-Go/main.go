package main

import (
	"ZCrypt/internal/CrypticEngine"
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"fmt"
	"io"
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

	AAD := CrypticEngine.BuildAAD(record.SchemaVersion, record.UserID, record.TemplateID, record.TemplateType, record.TemplateVer)

	text := []byte("hellow there !")

	ZCipher, err := aes.NewCipher(record.DEK)
	if err != nil {
		fmt.Println(err)
	}

	gcm, err := cipher.NewGCM(ZCipher)
	if err != nil {
		fmt.Println(err)
	}

	record.TemplateNonce = make([]byte, gcm.NonceSize())

	if _, err = io.ReadFull(rand.Reader, record.TemplateNonce); err != nil {
		fmt.Println(err)
	}

	record.Ciphertext = gcm.Seal(record.TemplateNonce, record.TemplateNonce, text, AAD)

	fmt.Println(record.Ciphertext)
}
