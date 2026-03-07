package CrypticEngine

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"fmt"
	"io"
)

func Encrypt(record *CrypticRecord, KEK, Data []byte) []byte {

	fmt.Println("meow !!")
	AAD := BuildAAD(record.SchemaVersion, record.UserID, record.TemplateID, record.TemplateType, record.TemplateVer)

	DEK := make([]byte, DEKSize)
	if _, err := io.ReadFull(rand.Reader, DEK); err != nil {
		return nil
	}

	record.TemplateNonce = make([]byte, NonceSizeGCM)

	if _, err := io.ReadFull(rand.Reader, record.TemplateNonce); err != nil {
		fmt.Println(err)
	}

	var err error
	record.Ciphertext, err = GCMSeal(DEK, record.TemplateNonce, Data, AAD)

	if err != nil {
		fmt.Println(err)
	}

	record.WrapNonce = make([]byte, NonceSizeGCM)

	if _, err := io.ReadFull(rand.Reader, record.WrapNonce); err != nil {
		fmt.Println(err)
	}

	record.DEK, err = GCMSeal(KEK, record.WrapNonce, DEK, AAD)

	if err != nil {
		fmt.Println(err)
	}

	return record.Ciphertext
}

func GCMSeal(Key, Nonce, Data, AAD []byte) ([]byte, error) {
	ZCipher, err := aes.NewCipher(Key)
	if err != nil {
		return nil, err
	}
	gcm, err := cipher.NewGCM(ZCipher)
	if err != nil {
		return nil, err
	}
	return gcm.Seal(nil, Nonce, Data, AAD), nil
}
