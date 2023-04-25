package tech_events

import (
	"bytes"
	"encoding/gob"

	"github.com/bwmarrin/discordgo"
	"github.com/shawnyu5/networking_bot/commands"
	"github.com/shawnyu5/networking_bot/utils"
)

// TechEvent interface defining the structure of any module that wants to opt into the tech-events command
// T: the type of the query string
type TechEvent interface {
	// retrieve events from the API
	FetchEvents() error
	// constructs the reply message from events
	ConstructReply() string
	// set the package level cache
	SetCache(cache interface{}) interface{}
	// get the package level cache
	GetCache() interface{}
	// clear out the package level cache
	ClearCache()
	// increase the page number in cache
	IncrementPageNumber()
	// decrease the page number in cache
	DecrementPageNumber()
}

// TechEventCommand the tech-event command
type TechEventCommand struct {
	// packages that have opted into this command
	Modules []TechEvent
}

// type State struct {
// // all sent messages
// // map of command : commandMessage
// Messages map[string]*discordgo.Message
// // messages containing pagination button
// PaginationMessages []*discordgo.Message
// }

// map of all package level caches
// hash of struct : cache
var cacheMap = make(map[string]interface{})

// map of all messages send
// hash of struct : message
var messageMap = make(map[string]*discordgo.Message)

// messages containing pagination button
var paginationMessages = make([]*discordgo.Message, 0)

// var state = State{
// Messages: make(map[string]*discordgo.Message),
// }
var nextPageComponentID = "tech events next page"
var previousPageComponentID = "tech events previous page"

// Components implements commands.Command
func (t TechEventCommand) Components() []commands.Component {
	return []commands.Component{
		{
			ComponentID:      nextPageComponentID,
			ComponentHandler: t.HandleNextPageButton,
		},
		{
			ComponentID:      previousPageComponentID,
			ComponentHandler: t.HandlePreviousPageButton,
		},
	}
}

// HandleNextPageButton handles when the next page button is clicked
func (t TechEventCommand) HandleNextPageButton(sess *discordgo.Session, i *discordgo.InteractionCreate) (string, error) {
	// update pagination buttons
	messArr := make([]*discordgo.Message, 0)
	for _, mess := range paginationMessages {
		reply := "Loading..."
		updatedMess, err := sess.ChannelMessageEditComplex(&discordgo.MessageEdit{
			Content: &reply,
			Components: []discordgo.MessageComponent{
				discordgo.ActionsRow{
					Components: []discordgo.MessageComponent{
						createPreviousPageButton(false),
						createNextPageButton(false),
					},
				},
			},
			ID:      mess.ID,
			Channel: mess.ChannelID,
		})
		if err != nil {
			return "", err
		}
		messArr = append(messArr, updatedMess)
	}
	paginationMessages = messArr

	for _, mod := range t.Modules {
		// set the cache at the package level
		mod.SetCache(cacheMap[hash(mod)])
		mod.IncrementPageNumber()
		err := mod.FetchEvents()
		if err != nil {
			return "", err
		}

		reply := mod.ConstructReply()

		mess := messageMap[hash(mod)]
		mess, err = sess.ChannelMessageEdit(mess.ChannelID, mess.ID, reply)
		if err != nil {
			return "", err
		}

		messageMap[hash(mod)] = mess
		cacheMap[hash(mod)] = mod.GetCache()
	}

	// TODO: idk why this is not able to update the interaction
	sess.InteractionRespond(i.Interaction, &discordgo.InteractionResponse{
		Type: discordgo.InteractionResponseUpdateMessage,
		Data: &discordgo.InteractionResponseData{
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
	// if err != nil {
	// return "", err
	// }

	messArr = make([]*discordgo.Message, 0)
	for _, mess := range paginationMessages {
		reply := "next page updated"
		updatedMess, err := sess.ChannelMessageEditComplex(&discordgo.MessageEdit{
			Content: &reply,
			Components: []discordgo.MessageComponent{
				discordgo.ActionsRow{
					Components: []discordgo.MessageComponent{
						createPreviousPageButton(false),
						createNextPageButton(false),
					},
				},
			},
			ID:      mess.ID,
			Channel: mess.ChannelID,
		})
		if err != nil {
			return "", err
		}
		messArr = append(messArr, updatedMess)
	}
	paginationMessages = messArr

	return "next page updated", nil
}

// HandleNextPageButton handles when the next page button is clicked
func (t TechEventCommand) HandlePreviousPageButton(sess *discordgo.Session, i *discordgo.InteractionCreate) (string, error) {
	// update pagination buttons
	messArr := make([]*discordgo.Message, 0)
	for _, mess := range paginationMessages {
		reply := "Loading..."
		updatedMess, err := sess.ChannelMessageEditComplex(&discordgo.MessageEdit{
			Content: &reply,
			Components: []discordgo.MessageComponent{
				discordgo.ActionsRow{
					Components: []discordgo.MessageComponent{
						createPreviousPageButton(false),
						createNextPageButton(false),
					},
				},
			},
			ID:      mess.ID,
			Channel: mess.ChannelID,
		})
		if err != nil {
			return "", err
		}
		messArr = append(messArr, updatedMess)
	}
	paginationMessages = messArr

	for _, mod := range t.Modules {
		// set the cache at the package level
		mod.SetCache(cacheMap[hash(mod)])
		mod.DecrementPageNumber()
		err := mod.FetchEvents()
		if err != nil {
			return "", err
		}

		reply := mod.ConstructReply()

		mess := messageMap[hash(mod)]
		mess, err = sess.ChannelMessageEdit(mess.ChannelID, mess.ID, reply)
		if err != nil {
			return "", err
		}

		messageMap[hash(mod)] = mess
		cacheMap[hash(mod)] = mod.GetCache()
	}

	// TODO: idk why this is not able to update the interaction
	sess.InteractionRespond(i.Interaction, &discordgo.InteractionResponse{
		Type: discordgo.InteractionResponseUpdateMessage,
		Data: &discordgo.InteractionResponseData{
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
	// if err != nil {
	// return "", err
	// }

	messArr = make([]*discordgo.Message, 0)
	for _, mess := range paginationMessages {
		reply := "previous page updated"
		updatedMess, err := sess.ChannelMessageEditComplex(&discordgo.MessageEdit{
			Content: &reply,
			Components: []discordgo.MessageComponent{
				discordgo.ActionsRow{
					Components: []discordgo.MessageComponent{
						createPreviousPageButton(false),
						createNextPageButton(false),
					},
				},
			},
			ID:      mess.ID,
			Channel: mess.ChannelID,
		})
		if err != nil {
			return "", err
		}
		messArr = append(messArr, updatedMess)
	}
	paginationMessages = messArr

	return "previous page updated", nil
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
	switch i.Type {
	case discordgo.InteractionApplicationCommand:
		utils.DeferReply(sess, i.Interaction)
		mess, err := sendPaginationButton(sess, i)
		if err != nil {
			return "", err
		}
		paginationMessages = append(paginationMessages, mess)

		for _, mod := range t.Modules {
			err := mod.FetchEvents()
			if err != nil {
				return "", err
			}

			reply := mod.ConstructReply()
			mess, err := sess.ChannelMessageSend(i.ChannelID, reply)
			if err != nil {
				return "", err
			}

			// mod.GetCache()
			// fmt.Printf("Handler cache: %+v\n", cache) // __AUTO_GENERATED_PRINT_VAR__
			cacheMap[hash(mod)] = mod.GetCache()
			messageMap[hash(mod)] = mess
			// clear out the cache to force `FetchEvents` to use data from the parent struct
			mod.ClearCache()

			// send page separator
			_, err = sess.ChannelMessageSend(i.ChannelID, "---------------------")
			if err != nil {
				return "", err
			}
		}

		// respond to the interaction
		res := "all events sent"
		_, err = sess.InteractionResponseEdit(i.Interaction, &discordgo.WebhookEdit{
			Content: &res,
			// Components: &[]discordgo.MessageComponent{
			// discordgo.ActionsRow{
			// Components: []discordgo.MessageComponent{
			// createPreviousPageButton(false),
			// createNextPageButton(false),
			// },
			// },
			// },
		})
		if err != nil {
			return "", err
		}

		mess, err = sendPaginationButton(sess, i)
		if err != nil {
			return "", err
		}
		paginationMessages = append(paginationMessages, mess)

		return "all events sent", nil
	case discordgo.InteractionApplicationCommandAutocomplete:
		return "", nil
	}
	return "", nil
}

// sendPaginationButton send pagination buttons to a channel.
// sess: discordgo session
// i: the interaction that triggered this command
// return: the message that was sent, and errors if any
func sendPaginationButton(sess *discordgo.Session, i *discordgo.InteractionCreate) (*discordgo.Message, error) {
	mess, err := sess.ChannelMessageSendComplex(i.ChannelID, &discordgo.MessageSend{
		Components: []discordgo.MessageComponent{
			discordgo.ActionsRow{
				Components: []discordgo.MessageComponent{
					createPreviousPageButton(true),
					createNextPageButton(false),
				},
			},
		},
	})
	if err != nil {
		return nil, err
	}

	return mess, nil
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
