# Networking accumulator API

A back end API to fetch networking events

## Routes

**DEPRECATED**

~~GET `/meetup/search?<query>&<page><per_page>` - search for events on meetup.~~

~~- `query`: the search query~~
~~- `page`: page number to display~~
~~- `per_page`: number of nodes to return~~

POST `/meetup/search` - search for events on meetup

Json body:

- `query`: the search query
- `page`: page number to display
- `per_page`: number of nodes to return
- `start_date`(optional): start date of events in ISO 8601 format

Returns:

```json
{
  "page_info": {
    "hasNextPage": true,
    "endCursor": "Mg=="
  },
  "nodes": [
    {
      "id": "292273023",
      "title": "Dupont Morning Code",
      "dateTime": "2023-04-01T13:00-04:00",
      "endTime": "2023-04-01T16:00-04:00",
      "description": "[Summary]\nThis meetup is for those who are interested in code, web development, design, cloud, or AI. All levels are welcome, so feel free to bring your laptop and discuss what you are working on or what you are learning. You can share your projects and ideas with other participants freely. There are outlets and WiFi, drinks are about $5 for tea, they take card only.\n\n[Price]\nAttendance is FREE. But you need to buy something from the venue.",
      "duration": "PT3H",
      "timezone": "America/Toronto",
      "eventType": "physical",
      "currency": "USD",
      "eventUrl": "https://www.meetup.com/toronto-tech-stack-exchange/events/292273023",
      "going": 2,
      "isAttending": false,
      "rsvpState": "JOIN_OPEN"
    }
  ]
}
```

GET `/tech-events?<page><per_page>` - return an accumulation of tech events, from various sources

- `page`: page number to display
- `per_page`: number of nodes to return

```json
{
  "page_info": {
    "hasNextPage": true,
    "endCursor": "Mg=="
  },
  "nodes": [
    {
      "id": "292273023",
      "title": "Dupont Morning Code",
      "dateTime": "2023-04-01T13:00-04:00",
      "endTime": "2023-04-01T16:00-04:00",
      "description": "[Summary]\nThis meetup is for those who are interested in code, web development, design, cloud, or AI. All levels are welcome, so feel free to bring your laptop and discuss what you are working on or what you are learning. You can share your projects and ideas with other participants freely. There are outlets and WiFi, drinks are about $5 for tea, they take card only.\n\n[Price]\nAttendance is FREE. But you need to buy something from the venue.",
      "duration": "PT3H",
      "timezone": "America/Toronto",
      "eventType": "physical",
      "currency": "USD",
      "eventUrl": "https://www.meetup.com/toronto-tech-stack-exchange/events/292273023",
      "going": 2,
      "isAttending": false,
      "rsvpState": "JOIN_OPEN"
    }
  ]
}
```
