package tech_events

import (
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
	// set local package cache
	SetCache()
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

var state State
var nextPageComponentID = "next page"
var previousPageComponentID = "next page"

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
	// sess.ChannelMessageSendComplex(i.ChannelID, &discordgo.MessageSend{
	// // Components: []discordgo.MessageComponent{
	// // t.Components(),
	// // },
	// })
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
