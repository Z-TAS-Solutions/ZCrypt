package CrypticEngine

import (
	"strconv"
	"strings"
)

func BuildAAD(schemaVersion uint16, userID, templateID, templateType string, templateVer uint16) []byte {
	var AAD strings.Builder
	AAD.WriteString("sv=")
	AAD.WriteString(strconv.FormatUint(uint64(schemaVersion), 10))
	AAD.WriteString("|uid=")
	AAD.WriteString(userID)
	AAD.WriteString("|tid=")
	AAD.WriteString(templateID)
	AAD.WriteString("|type=")
	AAD.WriteString(templateType)
	AAD.WriteString("|tver=")
	AAD.WriteString(strconv.FormatUint(uint64(templateVer), 10))
	return []byte(AAD.String())
}
