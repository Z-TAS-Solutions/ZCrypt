package CrypticEngine

import (
	"crypto/aes"
	"crypto/cipher"
	"fmt"
)

func Decrypt(Key []byte, record CrypticRecord, CipherText []byte) []byte {
	fmt.Println("Demeow !")

	AAD := BuildAAD(record.SchemaVersion, record.UserID, record.TemplateID, record.TemplateType, record.TemplateVer)

	Cipher, err := aes.NewCipher(Key)
	if err != nil {
		fmt.Println(err)
	}

	gcm, err := cipher.NewGCM(Cipher)
	if err != nil {
		fmt.Println(err)
	}

	nonceSize := gcm.NonceSize()
	if len(CipherText) < nonceSize {
		fmt.Println(err)
	}

	nonce, CipherText := CipherText[:nonceSize], CipherText[nonceSize:]
	plaintext, err := gcm.Open(nil, nonce, CipherText, AAD)

	if err != nil {
		fmt.Println(err)
	}
	return plaintext
}
