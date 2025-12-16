# Autumn Timerboard

A simple Discord-based fleet timerboard for EVE Online. 

This timerboard was created with the goal to address the issues of accidentally overlapping fleets as well as providing an easily accessible, single source of truth as to what fleets are upcoming. 

This application is experimental and more-so a prototype than something planned to be supported long term as its functionality will be merged into Bifrost Auth eventually. Regardless, you are free to use it as you see fit but do expect bugs as this application was written quickly and with a few cut corners for the sake of prototyping. If you encounter any bugs, please submit an issue so we can fix them.

## Features

**Ping Format**
- Create a standard ping format defining what information a fleet needs, e.g. `Form-up location`, `Voice comms`, `SRP`, etc

**Fleet Category**
- Select a ping format to be used by the fleet category
- Set the minimum time between fleets to avoid overlap (2 hours for roams or 0 for strategic fleets to demonstrate how they take precedent)
- Limit how far in advance a fleet can be scheduled (e.g. can't schedule a roam anymore than 24 hours in advance)
- Set if a reminder should be sent before fleet form-up (e.g. 1 hour before fleet)
- Choose which Discord roles have access to either view fleet, create a fleet, or manage fleets of that category
- Choose which Discord roles will be pinged when the fleet forms (@everyone or restrict to a specific role)
- Choose which channel(s) the fleet ping will be sent in 

**Multiple Discord Servers**
- Supports multiple Discord servers for one timerboard instance
- Does not currently support relaying fleets between Discords

**Timerboard**
- Provides a per Discord-server timerboard
- Provides a list of all upcoming fleets 
- Filters out fleets that started over an hour ago & hides fleets from categories a user doesn't have view access to

**Discord fleet pings (Pending implementation)**
- Sends pings & post upcoming fleets to configured channel(s) for fleet category
- Sends a ping when a fleet is first created
- Sends a reminder ping prior to form-up (if configured for fleet category)
- Sends a ping when fleet begins form-up
- Silently updates the fleet message when details are updated
- Silently updates fleet message and posts an additional message if a fleet is cancelled or the time is changed (without a ping)
- Periodicially provides a list of upcoming fleets with countdowns in the configured Discord channel - every 30 minutes pushes the list to most recent message for visibilty (deleting the prior posted list to not clutter the channel)

# Deployment

## Prerequsites

### Install Dependencies

- [Docker](https://docs.docker.com/engine/install/)
- [git](https://git-scm.com/install/linux)

### Clone the repository

```bash
git clone https://github.com/autumn-order/bifrost
```

### Create a Discord Developer Application

Create a Discord developer application at <https://discord.com/developers/applications>
- Go to `OAuth2` tab of your application and add a redirect under `Redirects`, set to `https://your-domain.com/api/auth/callback`
- Then, go to `Bot` tab, scroll down, enable the `Server Members Intent` which we need to access server members & roles to handle permissions for who can create timers.

Keep the developer application page open, configure your `.env` as directed below

### Configure Environment Variables

```bash
cp .env.example .env
```

Set the following in `.env`:
- `DOMAIN` (Set to your domain, e.g. `timerboard.autumn-order.com`)
- `DISCORD_REDIRECT_URL` (Set to your application's callback URL e.g. `https://timerboard.autumn-order.com/api/auth/callback`)
Create a Discord dev application at <https://discord.com/developers/applications> and set the following in `.env`
- `DISCORD_CLIENT_ID` (Get from `OAuth2 tab of your Discord developer application)
- `DISCORD_CLIENT_SECRET` (Get from `OAuth2 tab of your Discord developer application)
- `DISCORD_BOT_TOKEN` (Get from `Bot` tab of your Discord developer application - use the `Reset Token` button)

## Running for production

1. Start traefik proxy instance (if you don't have a reverse proxy already)

```bash
sudo docker network create traefik
```

```bash
sudo docker compose -f docker-compose.traefik.yml up -d
```

2. Run the application

```bash
sudo docker compose up -d
```

3. Create an admin login

```bash
sudo docker compose logs timerboard
```

- Find the admin login URL printed to logs which is generated when there are no current admins in the instance.
- `Ctrl + click` the link and then login with Discord, you will then be logged into the application and set as admin

If the link expires or you can't run it, run `sudo docker compose restart` then check logs again to get a new link.

# Development

Generate entities with:
```bash
sea-orm-cli generate entity -o ./entity/src/entities/ --date-time-crate chrono
```
