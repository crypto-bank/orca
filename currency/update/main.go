// Downloads a list of coins from coinmarketcap.com
// and constructs `symbol.rs` list of currency symbols

package main

import (
	"encoding/json"
	"html/template"
	"log"
	"net/http"
	"sort"
	"strings"
	"unicode"

	"os"
)

// Coin - Coin data.
type Coin struct {
	ID               string `json:"id"`
	Name             string `json:"name"`
	Symbol           string `json:"symbol"`
	Rank             string `json:"rank"`
	PriceUsd         string `json:"price_usd"`
	PriceBtc         string `json:"price_btc"`
	Two4HVolumeUsd   string `json:"24h_volume_usd"`
	MarketCapUsd     string `json:"market_cap_usd"`
	AvailableSupply  string `json:"available_supply"`
	TotalSupply      string `json:"total_supply"`
	PercentChange1H  string `json:"percent_change_1h"`
	PercentChange24H string `json:"percent_change_24h"`
	PercentChange7D  string `json:"percent_change_7d"`
	LastUpdated      string `json:"last_updated"`
}

func main() {
	resp, err := http.Get("https://api.coinmarketcap.com/v1/ticker/")
	if err != nil {
		log.Fatal(err)
	}
	defer resp.Body.Close()

	var coins []*Coin
	if err := json.NewDecoder(resp.Body).Decode(&coins); err != nil {
		log.Fatal(err)
	}

	// Leave only serious coins
	coins = onlySeriousCoins(coins)

	// Sort coins by symbol
	sort.Sort(bySymbol(coins))

	f, err := os.Create("src/symbol.rs")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	t := template.Must(template.ParseGlob("src/*.tmpl"))
	if err := t.Execute(f, coins); err != nil {
		log.Fatal(err)
	}

}

// No serious coin has a number in front of a symbol
// serious coins also are aware of existing use of a symbol
func onlySeriousCoins(coins []*Coin) (res []*Coin) {
	counts := make(map[string]int)
	for _, coin := range coins {
		counts[coin.Symbol]++
	}
	for _, coin := range coins {
		if strings.Contains(coin.Symbol, "@") {
			log.Printf("Dumb symbol %q", coin.Symbol)
			continue
		}
		// First character in symbol
		r := rune(coin.Symbol[0])
		if !unicode.IsLetter(r) {
			log.Printf("Dumb symbol %q", coin.Symbol)
			continue
		}
		// Ignore coin symbol if more than one
		if counts[coin.Symbol] > 1 {
			log.Printf("Doubled symbol %q", coin.Symbol)
			continue
		}
		res = append(res, coin)
	}
	return res
}

type bySymbol []*Coin

func (a bySymbol) Len() int           { return len(a) }
func (a bySymbol) Swap(i, j int)      { a[i], a[j] = a[j], a[i] }
func (a bySymbol) Less(i, j int) bool { return a[i].Symbol < a[j].Symbol }
