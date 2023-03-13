package tech_events

import (
	"github.com/bwmarrin/discordgo"
	"github.com/shawnyu5/networking_bot/commands/meetup"
)

// TechEvent interface defining the structure of any module that wants to opt into the tech_events command
// T: the type of the events returned by the API
type TechEvent[T any] interface {
	// retrieve events from the API
	fetchEvents(opts ...string) (T, error)
	// handles the next page button
	handleNextPageButton(sess *discordgo.Session, i *discordgo.InteractionCreate) (string, error)
	// handles the previous page button
	handlePreviousPageButton(sess *discordgo.Session, i *discordgo.InteractionCreate) (string, error)
	// constructs the reply message from events
	constructReply(events T) string
}

func hello() {
	var x TechEvent = []TechEvent{meetup.Meetup}
}
