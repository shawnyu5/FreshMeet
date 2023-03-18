# Networking accumulator API

A back end API to fetch networking events

## Routes

`/meetup/search?<query>&<page><per_page>` - search for events on meetup.

- `query`: the search query
- `page`: page number to display
- `per_page`: number of nodes to return

```json
{
  "page_info": {
    "hasNextPage": true,
    "endCursor": "MjA="
  },

  "nodes": [
    {
      "id": "291352101",
      "title": "Toronto's Biggest Professional Networking Mixer -Entrepreneur Tech & GameChanger",
      "dateTime": "2023-04-03T18:30-04:00",
      "endTime": "2023-04-03T20:30-04:00",
      "description": "***[Click Here To Confirm Your RSVP On EventBrite](https://bit.ly/3YdV0te)***\nOr\nClick Link Here - https://bit.ly/3YdV0te\n\nMAKE SURE TO RSVP ON EVENTBRITE TO ATTEND\n\nHighlights From Previous Events - Media Link Below\nhttps://youtu.be/9epSNzTL2tc\n\n***[Click Here To Confirm Your RSVP On EventBrite](https://bit.ly/3YdV0te)***\nOr\nClick Link Here - https://bit.ly/3YdV0te\n\nThis is...",
      "duration": "PT2H",
      "timezone": "America/Toronto",
      "eventType": "physical",
      "currency": "USD",
      "eventUrl": "https://www.meetup.com/toronto_tech/events/291352101"
    }
  ]
}
```
