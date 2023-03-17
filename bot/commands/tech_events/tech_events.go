package tech_events

import (
	"bytes"
	"encoding/gob"
	"reflect"

	"github.com/bwmarrin/discordgo"
	"github.com/shawnyu5/networking_bot/commands"
)

// TechEvent interface defining the structure of any module that wants to opt into the tech-events command
// T: the type of the query string
type TechEvent interface {
	// retrieve events from the API
	FetchEvents() error
	// handles the next page button
	HandleNextPageButton(sess *discordgo.Session, i *discordgo.InteractionCreate) (string, error)
	// handles the previous page button
	HandlePreviousPageButton(sess *discordgo.Session, i *discordgo.InteractionCreate) (string, error)
	// constructs the reply message from events
	ConstructReply() string
	// Returns the components of the package
	// CreateComponents() []discordgo.MessageComponent
	// load necessary values into the package level cache
	// SetCache()
	// Components() []commands.Component
}

// TechEventCommand the tech-event command
type TechEventCommand struct {
	// packages that have opted into this command
	Modules []TechEvent
}
type State struct {
	// all sent messages
	// map of command : commandMessage
	Messages map[string]*discordgo.Message
}

var state State = State{
	Messages: make(map[string]*discordgo.Message),
}
var nextPageComponentID = "next page"
var previousPageComponentID = "previous page"

// Components implements commands.Command
func (t TechEventCommand) Components() []commands.Component {
	return []commands.Component{
		{
			ComponentID:      nextPageComponentID,
			ComponentHandler: t.HandleNextPageButton,
		},
	}
}

// HandleNextPageButton handles when the next page button is clicked
func (t TechEventCommand) HandleNextPageButton(sess *discordgo.Session, i *discordgo.InteractionCreate) (string, error) {
	for _, mod := range t.Modules {
		// fmt.Printf("HandleNextPageButton mod: %+v\n", mod) // __AUTO_GENERATED_PRINT_VAR__
		_, err := mod.HandleNextPageButton(sess, i)
		if err != nil {
			return "", err
		}
	}
	return "next page", nil
}

// Def implements commands.Command
func (TechEventCommand) Def() *discordgo.ApplicationCommand {
	return &discordgo.ApplicationCommand{
		Version:     "1.0.0",
		Name:        "tech-events",
		Description: "fetch tech events from different sources",
	}
}

// Handler implements commands.Command
func (t TechEventCommand) Handler(sess *discordgo.Session, i *discordgo.InteractionCreate) (string, error) {
	for _, mod := range t.Modules {
		// mod.SetCache()
		err := mod.FetchEvents()
		if err != nil {
			return "", err
		}

		reply := mod.ConstructReply()
		mess, err := sess.ChannelMessageSendComplex(i.ChannelID, &discordgo.MessageSend{
			Content: reply,
		})
		if err != nil {
			return "", err
		}
		state.Messages[reflect.TypeOf(mod).String()] = mess
	}
	_, err := sess.ChannelMessageSendComplex(i.ChannelID, &discordgo.MessageSend{
		// Content: "**Pagination**",
		Components: []discordgo.MessageComponent{
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
	sess.InteractionRespond(i.Interaction, &discordgo.InteractionResponse{
		Type: discordgo.InteractionResponseChannelMessageWithSource,
		Data: &discordgo.InteractionResponseData{
			Content:         "events sent",
			Components:      []discordgo.MessageComponent{},
			Embeds:          []*discordgo.MessageEmbed{},
			AllowedMentions: &discordgo.MessageAllowedMentions{},
			Files:           []*discordgo.File{},
			Flags:           0,
			Choices:         []*discordgo.ApplicationCommandOptionChoice{},
			CustomID:        "",
			Title:           "",
		},
	})
	return "all events sent", nil
}

// OptIn add a opt a package into this command
func (t *TechEventCommand) OptIn(pack TechEvent) {
	t.Modules = append(t.Modules, pack)
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

// hash hash a struct
// t: the struct to hash
// return: the hashed representation of the struct
func hash(t TechEvent) string {
	var b bytes.Buffer
	gob.NewEncoder(&b).Encode(t)
	return b.String()
}
