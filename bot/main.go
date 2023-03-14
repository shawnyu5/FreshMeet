package main

import (
	"log"
	"os"
	"os/signal"

	"github.com/bwmarrin/discordgo"
	"github.com/shawnyu5/networking_bot/commands"
	"github.com/shawnyu5/networking_bot/commands/meetup"
	"github.com/shawnyu5/networking_bot/commands/tech_events"
	"github.com/shawnyu5/networking_bot/middware"
	utils "github.com/shawnyu5/networking_bot/utils"
)

var c utils.Config
var dg *discordgo.Session

// init reads config.json and sets global config variable
func init() {
	c = utils.LoadConfig()
}

func init() {
	var err error
	if c.Development {
		dg, err = discordgo.New("Bot " + c.TokenDev)
	} else {
		dg, err = discordgo.New("Bot " + c.Token)
	}
	if err != nil {
		log.Fatalf("Invalid bot parameters: %v", err)
	}
}

// a handler function type for slash command and components
type handlerFunc func(sess *discordgo.Session, i *discordgo.InteractionCreate)

var (
	// array of all slash commands in this bot
	allCommands = []commands.Command{
		meetup.Meetup{},
		tech_events.TechEventCommand{
			Modules: []tech_events.TechEvent{
				meetup.Meetup{},
			},
		},
	}

	// array of slash command defs
	slashCommandDefs = utils.GetCmdDefs(allCommands)
	// array of command handlers
	commandHandlers = utils.GetCmdHandler(allCommands)
	// array of component handlers
	componentsHandlers = utils.GetComponentHandler(allCommands)
)

func init() {
	dg.AddHandler(func(sess *discordgo.Session, i *discordgo.InteractionCreate) {
		switch i.Type {
		// handle slash command response and autocomplete requests the same way
		case discordgo.InteractionApplicationCommand, discordgo.InteractionApplicationCommandAutocomplete:
			if handle, ok := commandHandlers[i.ApplicationCommandData().Name]; ok {
				cmdObj := commands.CommandStruct{
					Name:    i.ApplicationCommandData().Name,
					Handler: handle,
				}
				logger := middware.NewLogger(log.New(os.Stdout, "", log.LstdFlags), cmdObj)
				logger.Handler(sess, i)
			} else {
				utils.SendErrorMessage(sess, i, "")
			}
		case discordgo.InteractionMessageComponent:
			if handle, ok := componentsHandlers[i.MessageComponentData().CustomID]; ok {
				cmdObj := commands.CommandStruct{
					Handler: handle,
				}

				logger := middware.NewLogger(log.New(os.Stdout, "", log.LstdFlags), cmdObj)
				logger.Handler(sess, i)
			} else {
				utils.SendErrorMessage(sess, i, "")
			}
		}
	})
}

func main() {
	// dg.Identify.Intents |= discordgo.IntentGuildMessages
	// dg.Identify.Intents |= discordgo.IntentGuildMembers
	dg.AddHandler(func(s *discordgo.Session, r *discordgo.Ready) {
		log.Printf("Logged in as: %v#%v", s.State.User.Username, s.State.User.Discriminator)
	})

	err := dg.Open()

	if err != nil {
		log.Fatalf("Cannot open the session: %v", err)
	}

	registeredCommands := make([]*discordgo.ApplicationCommand, len(slashCommandDefs))

	// remove old commands before adding new ones
	// utils.RemoveCommands(dg, registeredCommands)

	utils.RegisterCommands(dg, slashCommandDefs, registeredCommands)
	dg.AddHandler(func(sess *discordgo.Session, gld *discordgo.GuildCreate) {
		log.Printf("Bot added to new guild: %v", gld.Name)
		utils.RegisterCommands(dg, slashCommandDefs, registeredCommands)
	})

	defer dg.Close()

	stop := make(chan os.Signal, 1)
	signal.Notify(stop, os.Interrupt)
	log.Println("Press Ctrl+C to exit")
	<-stop

	// TODO: commands are not being deleted in my own server
	// only remove commands in production
	if !c.Development {
		utils.RemoveCommands(dg, registeredCommands)
	}

	log.Println("Gracefully shutting down.")
}
