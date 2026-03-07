package CrypticEngine

type CrypticRecord struct {
	SchemaVersion uint16

	UserID       string
	TemplateID   string
	TemplateType string
	TemplateVer  uint16

	DEK           []byte
	TemplateNonce []byte

	Ciphertext []byte
}
