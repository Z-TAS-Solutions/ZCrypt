package main

import (
	"ZCrypt/internal/CrypticEngine"
	"encoding/json"
	"flag"
	"fmt"
	"os"
)

func main() {

	KEK := []byte("Q7mZp2Hk9Vt4sFb1Rj8NcW3yLd6XgA0g")

	encryptFlag := flag.String("e", "", "encrypt mode")
	decryptFlag := flag.String("d", "", "decrypt mode")

	flag.Parse()

	if *encryptFlag != "" {

		record := CrypticEngine.CrypticRecord{
			SchemaVersion: 999,
			UserID:        "45567889",
			TemplateID:    "753468",
			TemplateType:  "Fusion",
			TemplateVer:   1,
		}

		FilePath := *encryptFlag

		Data, err := os.ReadFile(FilePath)
		if err != nil {
			fmt.Println("check the damned file ffs", err)
			os.Exit(1)
		}

		record.Ciphertext = CrypticEngine.Encrypt(&record, KEK, Data)

		fmt.Println("Nounce: ", record.TemplateNonce)
		fmt.Println("Ciphertext: ", record.Ciphertext)
		fmt.Println("DEK: ", record.DEK)
		fmt.Println("WrapNounce: ", record.WrapNonce)

		Output, err := json.MarshalIndent(record, "", " ")
		if err != nil {
			fmt.Println("Serialization Failure : ", err)
			os.Exit(1)

		}

		OutPath := FilePath + ".json"

		if err := os.WriteFile(OutPath, Output, 0600); err != nil {
			fmt.Println("write error:", err)
			os.Exit(1)
		}

	}

	if *decryptFlag != "" {

		FilePath := *decryptFlag

		Data, err := os.ReadFile(FilePath)
		if err != nil {
			fmt.Println("check the damned file ffs", err)
			os.Exit(1)
		}

		var record CrypticEngine.CrypticRecord

		json.Unmarshal(Data, &record)

		fmt.Println("Nounce: ", record.TemplateNonce)
		fmt.Println("Ciphertext: ", record.Ciphertext)
		fmt.Println("DEK: ", record.DEK)
		fmt.Println("WrapNounce: ", record.WrapNonce)

		fmt.Println(string(CrypticEngine.Decrypt(&record, KEK)))

	}

}
