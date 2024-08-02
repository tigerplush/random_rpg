# random_rpg

## Mission statement
This is the Random RPG project (or rrpg for short). It is supposed to procedurally create immersive worlds and characters for you to read, enjoy and to play with.

## Usage

rrpg is supposed to work like this:
1. Execute `rrpg init` to create a new folder with settings files. These settings files will be pre-filled with settings for a generic fantasy world. You can change and adapt these files to your liking.
2. Execute `rrpg generate` to generate the world from your settings. This will create a map, species, villages and cities, factions etc. and save it to a file. It will also create a wiki for you to browse through.
3. Add or edit "event" files and then run `rrpg update`. If the event is in the past of the current timeline, the simulation will be wound back to the specified time and then regenerated from there. This is your main mechanic to interact with the simulation.
