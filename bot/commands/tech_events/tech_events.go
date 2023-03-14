package tech_events

import (
	"github.com/bwmarrin/discordgo"
	"github.com/shawnyu5/networking_bot/commands"
)

type Query struct {
}

// TechEvent interface defining the structure of any module that wants to opt into the tech-events command
type TechEvent interface {
	// retrieve events from the API
	FetchEvents(opts ...string) error
	// handles the next page button
	HandleNextPageButton(sess *discordgo.Session, i *discordgo.InteractionCreate) (string, error)
	// handles the previous page button
	HandlePreviousPageButton(sess *discordgo.Session, i *discordgo.InteractionCreate) (string, error)
	// constructs the reply message from events
	ConstructReply() string
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
	for _, pack := range t.Modules {
		q := pack.Queries // array of strings that is our query strings?
		pack.FetchEvents(q)
		err := pack.FetchEvents()
		if err != nil {
			return "", err
		}
		reply := pack.ConstructReply()
		_, err = sess.ChannelMessageSendComplex(i.ChannelID, &discordgo.MessageSend{
			Content: reply,
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
