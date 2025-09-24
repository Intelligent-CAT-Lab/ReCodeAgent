package checkdigit

import (
	"errors"
	"fmt"
	"os"
	"strings"
)

func Example() error {
	provider := takeProvider("isbn10")
	if provider == nil {
		return errors.New("unimplemented provider")
	}
	if err := generate(provider, "xyz"); err != nil {
		return err
	}
	if !verify(provider, "abc") {
		return errors.New("failed to verify")
	}

	return nil
}

func takeProvider(name string) Provider { //nolint: cyclop
	switch strings.ToLower(strings.ReplaceAll(name, "-", "")) {
	case "luhn":
		return NewLuhn()
	case "verhoeff":
		return NewVerhoeff()
	case "damm":
		return NewDamm()
	case "isbn10":
		return NewISBN10()
	case "isbn13", "isbn":
		return NewISBN13()
	case "ean8":
		return NewEAN8()
	case "ean13", "ean":
		return NewEAN13()
	case "jan8":
		return NewJAN8()
	case "jan13", "jan":
		return NewJAN13()
	case "itf":
		return NewITF()
	case "upc":
		return NewUPC()
	case "sscc":
		return NewSSCC()
	}

	return nil
}

func generate(g Provider, seed string) error {
	cd, err := g.Generate(seed)
	if err != nil {
		return fmt.Errorf("failed to generate with seed, message: %w", err)
	}
	if cd > 9 {
		fmt.Fprintf(os.Stderr, "%s\n", "X")
	} else {
		fmt.Fprintf(os.Stderr, "%d\n", cd)
	}

	return nil
}

func verify(v Provider, target string) bool {
	ret := v.Verify(target)
	fmt.Fprintf(os.Stderr, "%t\n", ret)

	return ret
}
