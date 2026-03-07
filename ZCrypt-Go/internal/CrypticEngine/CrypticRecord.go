package CrypticEngine

const (
	NonceSizeGCM = 12
	DEKSize      = 32
)

type CrypticRecord struct {
	SchemaVersion uint16

	UserID       string
	TemplateID   string
	TemplateType string
	TemplateVer  uint16

	DEK           []byte
	TemplateNonce []byte
	WrapNonce     []byte

	Ciphertext []byte
}
