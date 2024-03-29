package meetup

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"regexp"
	"strconv"
	"strings"
	"time"

	"github.com/bwmarrin/discordgo"
	"github.com/shawnyu5/networking_bot/commands"
	"github.com/shawnyu5/networking_bot/utils"
)

// Events Events response from the meetup api
type Events struct {
	PageInfo struct {
		EndCursor   string `json:"endCursor"`
		HasNextPage bool   `json:"hasNextPage"`
	} `json:"page_info"`
	Nodes []struct {
		Currency    string `json:"currency"`
		DateTime    string `json:"dateTime"`
		Description string `json:"description"`
		Duration    string `json:"duration"`
		EndTime     string `json:"endTime"`
		EventType   string `json:"eventType"`
		EventURL    string `json:"eventUrl"`
		Going       int64  `json:"going"`
		ID          string `json:"id"`
		IsAttending bool   `json:"isAttending"`
		RsvpState   string `json:"rsvpState"`
		Timezone    string `json:"timezone"`
		Title       string `json:"title"`
	} `json:"nodes"`
}

type QueryString struct {
	// the search query the user entered
	Query   string
	Page    int
	PerPage string
}

// `/meetup` command
type Meetup struct {
	Query QueryString
}

// State local persistent state
type State struct {
	// Query for the API
	Query QueryString
	// events fetched from the meetup api
	events Events
}

var Cache State

// var state State
var cursor string

var nextPageComponentID = "meetup next page"
var previousPageComponentID = "meetup previous page"

func (m Meetup) Components() []commands.Component {
	return []commands.Component{
		{
			ComponentID:      nextPageComponentID,
			ComponentHandler: m.HandleNextPageButton,
		},
		{
			ComponentID:      previousPageComponentID,
			ComponentHandler: m.HandlePreviousPageButton,
		},
	}
}

// HandleNextPageButton handle when the next page button is clicked
func (m Meetup) HandleNextPageButton(sess *discordgo.Session, i *discordgo.InteractionCreate) (string, error) {
	if !Cache.events.PageInfo.HasNextPage {
		return "No more pages", nil
	}
	m.IncrementPageNumber()
	err := m.FetchEvents()
	if err != nil {
		return "", err
	}

	reply := m.ConstructReply()
	disableNextButton := !Cache.events.PageInfo.HasNextPage

	err = sess.InteractionRespond(i.Interaction, &discordgo.InteractionResponse{
		Type: discordgo.InteractionResponseUpdateMessage,
		Data: &discordgo.InteractionResponseData{
			Content: reply,
			Components: []discordgo.MessageComponent{
				discordgo.ActionsRow{
					Components: []discordgo.MessageComponent{
						createPreviousPageButton(false),
						createNextPageButton(disableNextButton),
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

// HandlePreviousPageButton handle when the previous page button is clicked
func (m Meetup) HandlePreviousPageButton(sess *discordgo.Session, i *discordgo.InteractionCreate) (string, error) {
	m.DecrementPageNumber()
	err := m.FetchEvents()
	if err != nil {
		return "", err
	}

	reply := m.ConstructReply()

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
		Disabled: disabled,
		CustomID: nextPageComponentID,
	}

}

// createPreviousPageButton create a previous page button
// disabled: if the button should be disabled
func createPreviousPageButton(disabled bool) discordgo.Button {
	return discordgo.Button{
		Label:    "⬅️",
		Style:    discordgo.PrimaryButton,
		Disabled: disabled,
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

func (m Meetup) Handler(sess *discordgo.Session, i *discordgo.InteractionCreate) (string, error) {
	utils.DeferReply(sess, i.Interaction)
	userOptions := utils.ParseUserOptions(sess, i)
	Cache.Query.Query = userOptions["query"].StringValue()
	Cache.Query.Page = 1
	Cache.Query.PerPage = "4"

	err := m.FetchEvents()
	if err != nil {
		return "", err
	}

	// construct a reply from api body
	reply := m.ConstructReply()

	_, err = sess.InteractionResponseEdit(i.Interaction, &discordgo.WebhookEdit{
		Content: &reply,
		Components: &[]discordgo.MessageComponent{
			discordgo.ActionsRow{
				Components: []discordgo.MessageComponent{
					createPreviousPageButton(true),
					createNextPageButton(false),
				},
			},
		},
	})
	if err != nil {
		return "", err
	}

	time.AfterFunc(5*time.Minute, func() {
		reply := m.ConstructReply()

		_, err = sess.InteractionResponseEdit(i.Interaction, &discordgo.WebhookEdit{
			Content: &reply,
			Components: &[]discordgo.MessageComponent{
				discordgo.ActionsRow{
					Components: []discordgo.MessageComponent{
						createPreviousPageButton(true),
						createNextPageButton(true),
					},
				},
			},
		})
		if err != nil {
			log.Println(err)
			return
		}
		log.Println("meetups buttons disabled")
	})

	return "list of events sent", nil
}

// ConstructReply construct a reply with data from the API
// returns: a string to be sent as a reply
func (m Meetup) ConstructReply() string {
	response := ""
	for _, event := range Cache.events.Nodes {
		description := strings.ReplaceAll(event.Description, "\n", " ")
		// truncate description to 100 characters
		if len(description) > 250 {
			// wrap all links with <>, to avid embed preview
			description = description[:250]

			httpRegex := regexp.MustCompile(`(https://\S+)`)
			description = httpRegex.ReplaceAllString(description, "<$1>")
			description = fmt.Sprintf("%s...", description)

			// remove all `*`
			description = strings.ReplaceAll(description, "*", "")
		}

		// 2023-03-17T18:30-04:00
		// trim away the date
		date := strings.Split(event.DateTime, "T")[0]
		startTime := strings.SplitAfter(event.DateTime, "T")[1]
		// remove everything after `-`
		startTime = strings.Split(startTime, "-")[0]

		// endTime is in the same format is start time
		endTime := strings.SplitAfter(event.EndTime, "T")[1]
		endTime = strings.Split(endTime, "-")[0]

		eventDate := fmt.Sprintf("%s, %s - %s", date, startTime, endTime)

		response += fmt.Sprintf("**title**: %s(%d ppl)\n**description**: %s\n**date**: %s\n**URL**: <%s>\n\n", event.Title, event.Going, description, eventDate, event.EventURL)
	}
	return response
}

// FetchEvents get events from the meetup api. Store events in Meetup.Events. Reads the query params from the cache
// returns: errors if any
func (m Meetup) FetchEvents() error {
	// if no query, fetch events from Meetup struct
	if Cache.Query.Query == "" {
		Cache.Query.Query = m.Query.Query
		Cache.Query.Page = m.Query.Page
		Cache.Query.PerPage = m.Query.PerPage
	}
	// fmt.Printf("FetchEvents Cache: %+v\n", Cache.Query) // __AUTO_GENERATED_PRINT_VAR__
	config := utils.LoadConfig()

	req, err := http.NewRequest(http.MethodGet, fmt.Sprintf("%s/meetup/search", config.ApiUrl), nil)
	if err != nil {
		return err
	}

	q := req.URL.Query()
	q.Add("query", Cache.Query.Query)
	q.Add("page", strconv.Itoa(Cache.Query.Page))
	q.Add("per_page", Cache.Query.PerPage)
	req.URL.RawQuery = q.Encode()

	res, err := http.DefaultClient.Do(req)
	if err != nil {
		return err
	}

	b, err := ioutil.ReadAll(res.Body)
	if err != nil {
		return err
	}
	var body Events
	json.Unmarshal(b, &body)
	Cache.events = body
	// TODO: remove this
	cursor = body.PageInfo.EndCursor
	return nil

}

// GetCache implements tech_events.TechEvent
func (Meetup) GetCache() interface{} {
	return Cache
}

// SetCache implements tech_events.TechEvent
func (Meetup) SetCache(cache interface{}) interface{} {
	Cache = cache.(State)
	return Cache
}

// DecrementPageNumber implements tech_events.TechEvent
func (Meetup) DecrementPageNumber() {
	Cache.Query.Page--
}

// IncrementPageNumber implements tech_events.TechEvent
func (Meetup) IncrementPageNumber() {
	Cache.Query.Page++
}

// ClearCache implements tech_events.TechEvent
func (Meetup) ClearCache() {
	empty := State{}
	Cache = empty
}

// func (m Meetup) CreateComponents() []discordgo.MessageComponent {
// return []discordgo.MessageComponent{
// m.createPreviousPageButton(false),
// m.createNextPageButton(false),
// }
// }

// SetCache load data from Events into state cache
// func (m Meetup) SetCache() {
// state.query.Page = m.QueryString.Page
// state.query.PerPage = m.QueryString.PerPage
// state.query.Query = m.QueryString.Query
// }
