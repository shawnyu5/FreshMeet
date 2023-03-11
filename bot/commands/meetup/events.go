package meetup

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	"strconv"
	"strings"

	"github.com/bwmarrin/discordgo"
	"github.com/shawnyu5/networking_bot/commands"
	"github.com/shawnyu5/networking_bot/utils"
)

// Events Events response from the meetup api
type Events []struct {
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
			Going       int    `json:"going"`
		} `json:"result"`
	} `json:"node"`
}

// /meetup command
type Meetup struct{}

// State local persistent state
type State struct {
	// events from the meetup api
	events Events
	// the search query the user entered
	query string
	// the current page number the user is on
	page int
}

var state State

var nextPageComponentID = "next page"
var previousPageComponentID = "previous page"

func (Meetup) Components() []commands.Component {
	return []commands.Component{
		{
			ComponentID:      nextPageComponentID,
			ComponentHandler: handleNextPageButton,
		},
		{
			ComponentID:      previousPageComponentID,
			ComponentHandler: handlePreviousPageButton,
		},
	}
}

// handleNextPageButton handle when the next page button is clicked
func handleNextPageButton(sess *discordgo.Session, i *discordgo.InteractionCreate) (string, error) {
	state.page++
	events, err := getEvents(state.query, state.page, 4)
	if err != nil {
		return "", err
	}

	reply := constructReply(events)
	println(len(reply))

	err = sess.InteractionRespond(i.Interaction, &discordgo.InteractionResponse{
		Type: discordgo.InteractionResponseUpdateMessage,
		Data: &discordgo.InteractionResponseData{
			Content: reply,
			Components: []discordgo.MessageComponent{
				discordgo.ActionsRow{
					Components: []discordgo.MessageComponent{
						createPreviousPageButton(false),
						createNextPageButton(false),
					},
				},
			},
		},
	})
	if err != nil {
		return "", err
	}

	return "Next page event updated", nil
}

// handlePreviousPageButton handle when the previous page button is clicked
func handlePreviousPageButton(sess *discordgo.Session, i *discordgo.InteractionCreate) (string, error) {
	state.page--
	events, err := getEvents(state.query, state.page, 4)
	if err != nil {
		return "", err
	}

	reply := constructReply(events)

	err = sess.InteractionRespond(i.Interaction, &discordgo.InteractionResponse{
		Type: discordgo.InteractionResponseUpdateMessage,
		Data: &discordgo.InteractionResponseData{
			Content: reply,
			Components: []discordgo.MessageComponent{
				discordgo.ActionsRow{
					Components: []discordgo.MessageComponent{
						createPreviousPageButton(false),
						createNextPageButton(false),
					},
				},
			},
		},
	})
	if err != nil {
		return "", err
	}

	return "previous page event updated", nil

}

// createNextPageButton create next page button
// disabled: if the button should be disabled
func createNextPageButton(disabled bool) discordgo.Button {
	return discordgo.Button{
		Label:    "➡️",
		Style:    discordgo.PrimaryButton,
		Disabled: false,
		CustomID: nextPageComponentID,
	}

}

// createPreviousPageButton create a previous page button
// disabled: if the button should be disabled
func createPreviousPageButton(disabled bool) discordgo.Button {
	return discordgo.Button{
		Label:    "⬅️",
		Style:    discordgo.PrimaryButton,
		Disabled: false,
		CustomID: previousPageComponentID,
	}

}
func (Meetup) Def() *discordgo.ApplicationCommand {
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
				Required:    true,
			},
		},
	}
	return obj
}

func (Meetup) Handler(sess *discordgo.Session, i *discordgo.InteractionCreate) (string, error) {
	utils.DeferReply(sess, i.Interaction)
	userOptions := utils.ParseUserOptions(sess, i)
	state.query = userOptions["query"].StringValue()
	state.page = 1

	events, err := getEvents(state.query, state.page, 4)
	if err != nil {
		return "", err
	}

	// construct a reply from api body
	reply := constructReply(events)

	_, err = sess.InteractionResponseEdit(i.Interaction, &discordgo.WebhookEdit{
		Content: &reply,
		Components: &[]discordgo.MessageComponent{
			discordgo.ActionsRow{
				Components: []discordgo.MessageComponent{
					createPreviousPageButton(false),
					createNextPageButton(false),
				},
			},
		},
	})
	if err != nil {
		return "", err
	}
	return "list of events sent", nil
}

// constructReply construct a reply with data from the API
// events: events from the meetup api.
// returns: a string to be sent as a reply
func constructReply(events Events) string {
	response := ""
	for _, event := range events {
		description := strings.ReplaceAll(event.Node.Result.Description, "\n", " ")
		// truncate description to 100 characters
		if len(description) > 250 {
			description = fmt.Sprintf("%s...", description[:250])
			// remove all `*`
			description = strings.ReplaceAll(description, "*", "")
		}

		// 2023-03-17T18:30-04:00
		// trim away the date
		date := strings.Split(event.Node.Result.DateTime, "T")[0]
		startTime := strings.SplitAfter(event.Node.Result.DateTime, "T")[1]
		// remove everything after `-`
		startTime = strings.Split(startTime, "-")[0]

		// endTime is in the same format is start time
		endTime := strings.SplitAfter(event.Node.Result.EndTime, "T")[1]
		endTime = strings.Split(endTime, "-")[0]

		eventDate := fmt.Sprintf("%s %s - %s", date, startTime, endTime)

		response += fmt.Sprintf("**title**: %s(%d)\n**description**: %s\n**date**: %s\n**URL**: <%s>\n\n", event.Node.Result.Title, event.Node.Result.Going, description, eventDate, event.Node.Result.EventURL)
	}
	return response
}

// getEvents get events from the meetup api
// query: search query
// page: page number
// perPage: number of results per page
// returns: events from the meetup api
func getEvents(query string, page int, perPage int) (Events, error) {
	config := utils.LoadConfig()

	req, err := http.NewRequest(http.MethodGet, fmt.Sprintf("%s/meetup/search", config.ApiUrl), nil)
	if err != nil {
		return Events{}, err
	}

	q := req.URL.Query()
	q.Add("query", query)
	q.Add("page", strconv.Itoa(page))
	q.Add("per_page", strconv.Itoa(perPage))
	req.URL.RawQuery = q.Encode()

	res, err := http.DefaultClient.Do(req)
	if err != nil {
		return Events{}, err
	}

	b, err := ioutil.ReadAll(res.Body)
	if err != nil {
		return Events{}, err
	}
	var body Events
	json.Unmarshal(b, &body)
	return body, nil

}
