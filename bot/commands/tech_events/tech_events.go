package tech_events

import (
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
	CreateComponents() []discordgo.MessageComponent
	// set local package cache
	SetCache()
	Components() []commands.Component
}

// TechEventCommand the tech-event command
type TechEventCommand struct {
	// packages that have opted into this command
	Modules []TechEvent
}

// Components implements commands.Command
func (TechEventCommand) Components() []commands.Component {
	return []commands.Component{}
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
		mod.Components()[0].ComponentHandler(sess, i)
		err := mod.FetchEvents()
		if err != nil {
			return "", err
		}
		reply := mod.ConstructReply()
		_, err = sess.ChannelMessageSendComplex(i.ChannelID, &discordgo.MessageSend{
			Content: reply,
			TTS:     false,
			Components: []discordgo.MessageComponent{
				discordgo.ActionsRow{
					Components: mod.CreateComponents(),
				},
			},
		})
		if err != nil {
			return "", err
		}
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
