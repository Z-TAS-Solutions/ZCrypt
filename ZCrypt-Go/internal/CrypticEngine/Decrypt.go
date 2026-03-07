package CrypticEngine

import (
	"crypto/aes"
	"crypto/cipher"
	"fmt"
)

func Decrypt(record *CrypticRecord, KEK []byte) []byte {
	fmt.Println("Demeow !")

	AAD := BuildAAD(record.SchemaVersion, record.UserID, record.TemplateID, record.TemplateType, record.TemplateVer)

	DEK, err := GCMOpen(KEK, record.WrapNonce, record.DEK, AAD)
	if err != nil {
		fmt.Println(err)
	}

	Data, err := GCMOpen(DEK, record.TemplateNonce, record.Ciphertext, AAD)
	if err != nil {
		fmt.Println(err)
	}

	return Data

}

func GCMOpen(Key, Nonce, CipherText, AAD []byte) ([]byte, error) {
	ZCipher, err := aes.NewCipher(Key)
	if err != nil {
		return nil, err
	}
	gcm, err := cipher.NewGCM(ZCipher)
	if err != nil {
		return nil, err
	}
	return gcm.Open(nil, Nonce, CipherText, AAD)
}
