package events

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"

	"github.com/bwmarrin/discordgo"
	"github.com/shawnyu5/networking_bot/commands"
	"github.com/shawnyu5/networking_bot/utils"
)

// event A event from the meetup api
type event []struct {
	Node struct {
		ID     string `json:"id"`
		Result struct {
			Currency    string `json:"currency"`
			DateTime    string `json:"dateTime"`
			Description string `json:"description"`
			Duration    string `json:"duration"`
			EndTime     string `json:"endTime"`
			EventType   string `json:"eventType"`
			EventURL    string `json:"eventUrl"`
			ID          string `json:"id"`
			Timezone    string `json:"timezone"`
			Title       string `json:"title"`
		} `json:"result"`
	} `json:"node"`
}

type Events struct{}

func (Events) Components() []commands.Component {
	return []commands.Component{}
}

func (Events) Def() *discordgo.ApplicationCommand {
	obj := &discordgo.ApplicationCommand{
		Version:     "1.0.0",
		Name:        "meetup",
		NSFW:        new(bool),
		Description: "Find events on meetup",
		Options: []*discordgo.ApplicationCommandOption{
			{
				Type:        discordgo.ApplicationCommandOptionString,
				Name:        "query",
				Description: "search query",
				Required:    false,
			},
		},
	}
	return obj
}

func (Events) Handler(sess *discordgo.Session, i *discordgo.InteractionCreate) (string, error) {
	config := utils.LoadConfig()
	// make an http request
	req, err := http.NewRequest(http.MethodGet, fmt.Sprintf("%s/%s", config.ApiUrl, "/search?query=coding"), nil)
	if err != nil {
		return "", err
	}

	res, err := http.DefaultClient.Do(req)
	if err != nil {
		log.Fatal(err)
	}

	// read the response b
	b, err := ioutil.ReadAll(res.Body)
	if err != nil {
		log.Fatal(err)
	}
	// fmt.Printf("Handler b: %v\n", string(b)) // __AUTO_GENERATED_PRINT_VAR__
	var body event
	json.Unmarshal(b, &body)
	fmt.Printf("Handler body: %+v\n", body) // __AUTO_GENERATED_PRINT_VAR__

	err = sess.InteractionRespond(i.Interaction, &discordgo.InteractionResponse{
		Type: discordgo.InteractionResponseChannelMessageWithSource,
		Data: &discordgo.InteractionResponseData{
			Content: body[0].Node.Result.Title,
			// Components:      []discordgo.MessageComponent{},
			// Embeds:          []*discordgo.MessageEmbed{},
			// AllowedMentions: &discordgo.MessageAllowedMentions{},
			// Files:           []*discordgo.File{},
			// Flags:           0,
			// Choices:         []*discordgo.ApplicationCommandOptionChoice{},
			// CustomID:        "",
			// Title:           "",
		},
	})
	if err != nil {
		return "", err
	}
	return "response sent", nil
}
