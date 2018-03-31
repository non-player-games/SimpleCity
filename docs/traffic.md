# Traffic Discussion Note

## Recap of last week

To continue our discussion from what we have so far:

* Able to set a grid
* Able to display a grid of tiles
* Able to send messages between the _system_ and the _client_

## Goal of this week

We need to figure out a way to design the game better by the ability of gaining
money (resource) and spend money.

## Brainstorm of the goal - connectivity

To generatre money, we need to bable to collect _taxes_ from the RCI zones.
However, this involves designing what I called the simulation cycles (e.g. able to
increate population or able to compute power to see if zone has the supply).
This brings up the topic of how do we determine a tile is connected to the other
as we need to determine if a residential zone has a place to work and has a place
to shop.

In other word, we need a way to determine if a zone tile is connected to other
tiles. Graph data structure becomes the first thing I have in mind about
_connectivity_ so I work backward about what data should we store as _node_ and
what data we store as _edge_.

As discussion goes on, we soon realized that *road* tile is necessary for the
SimCity. Rather, the road tile is the first class unit along with RCI tiles. In
example, user cannot even define any RCI zone if the tile is not connected to a
road tile.

Thus, our next focus should be on defining road tiles and RCI zones around the
road tiles.

## Terminology

* Zone = Tile = single tile on the grid
* Area = A group ot zones/tiles

## Operation to check if two areas are connected

An operation to convert from a zone grid

## TODOs

* Define a road from UI & System
* Functionalal requirements to build RCI zone only adjacent to a road tile

* Validation when buying a RCI zone (check if is is connect to a road)
    * problem: validation logic once for client and once for system
* UI connect roads together line of roads

* Algorithm to define connected tiles (define cluster of things)
    * Group same tiles and draw them together
* Research on validation logic
* After grouping RCI tiles, we can find a way to de-group RCI tiles into bunch
  of 1x1, 2x2, 3x3 tiles as low, medium, high density tiles

## Scratch pads

### ZoneTypes

0: None
1: Road
2: R
3: C
4: I

### Road Scenario

```
9 0 1 0 9
9 0 1 0 9
9 0 1 1 0
9 0 0 1 0
0 1 1 1 0
```

-> A bunch of area

an area of 1:
{
    tiles: [
        { x: 0, y: 0, type: 1 }
        { x: 1, y: 0, type: 1 }
        { x: 0, y: 1, type: 1 }
        { x: 1, y: 1, type: 1 }
    ]
}

Graph:
Nodes -> Area
Edges -> Road
