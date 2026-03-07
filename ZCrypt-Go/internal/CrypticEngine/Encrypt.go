package CrypticEngine

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"fmt"
	"io"
)

func Encrypt(record CrypticRecord, text []byte) []byte {

	fmt.Println("meow !!")
	AAD := BuildAAD(record.SchemaVersion, record.UserID, record.TemplateID, record.TemplateType, record.TemplateVer)

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

	return gcm.Seal(record.TemplateNonce, record.TemplateNonce, text, AAD)

}
