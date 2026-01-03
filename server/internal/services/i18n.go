package services

import (
	"encoding/json"

	"github.com/gofiber/fiber/v2"
	"github.com/nicksnyder/go-i18n/v2/i18n"
	"golang.org/x/text/language"
)

var bundle *i18n.Bundle

func InitI18n(localesPath string) error {
	bundle = i18n.NewBundle(language.English)
	bundle.RegisterUnmarshalFunc("json", json.Unmarshal)

	if _, err := bundle.LoadMessageFile(localesPath + "/en.json"); err != nil {
		return err
	}
	if _, err := bundle.LoadMessageFile(localesPath + "/zh.json"); err != nil {
		return err
	}

	return nil
}

func GetBundle() *i18n.Bundle {
	return bundle
}

// T 从 fiber context 获取 localizer 并翻译
func T(c *fiber.Ctx, msgID string) string {
	loc, ok := c.Locals("localizer").(*i18n.Localizer)
	if !ok {
		return msgID
	}
	msg, err := loc.Localize(&i18n.LocalizeConfig{MessageID: msgID})
	if err != nil {
		return msgID
	}
	return msg
}
