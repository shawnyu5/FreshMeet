package events

import (
	"github.com/bwmarrin/discordgo"
	"github.com/shawnyu5/networking_bot/commands"
)

type Events struct{}

func (Events) Components() []commands.Command {
	return []commands.Command{}
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

}
