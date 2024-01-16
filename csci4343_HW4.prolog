%Description: This game allows the user to move between rooms, pick up items, and unlock doors to escape a house
%This line is needed so you can modify the variables with these predicates
:- dynamic item_at/2, here/1.

%Predicate used to say which room we are currently in
here(entryway).

%series of predicates that tells us how the rooms are connected
move_to(entryway, e, kitchen).
move_to(kitchen, w, entryway).

move_to(entryway, w, dining_room).
move_to(dining_room, e, entryway).

move_to(entryway, s, outside) :- item_at(house_key, in_hand).
move_to(entryway, s, outside) :- write('Cannot exit the house without the key.'), nl, !, fail.

move_to(entryway, u, upstairs_landing) :- item_at(flashlight, in_hand).
move_to(entryway, u, upstairs_landing) :- write('Too dark to go upstairs.'), nl, !, fail.

move_to(upstairs_landing, d, entryway).

move_to(upstairs_landing, n, bedroom).

move_to(bedroom, s, upstairs_landing).
move_to(bedroom, w, bathroom).
move_to(bedroom, e, closet) :- item_at(closet_key, in_hand).
move_to(bedroom, e, closet) :- write('Cannot enter the closet without the key.'), nl, !, fail.

move_to(bathroom, e, bedroom).
move_to(bathroom, u, roof).

move_to(roof, d, bathroom).

move_to(closet, w, bedroom).

%predicates that shows which rooms the items exist in
item_at(flashlight, kitchen).
item_at(closet_key, dining_room).
item_at(house_key, closet).


%the take() predicates allow us to pick up an item.
take(Item) :- 
	item_at(Item, in_hand),
	write('You already have it'),
	nl, !.

take(Item) :-
	here(Place),
	item_at(Item, Place),
	retract(item_at(Item, Place)),
	assert(item_at(Item, in_hand)),
	write('OK.'),
	nl, !.
	
take(_) :-
	write('You do not see that here.'), nl.


%the describe() predicates give the room descriptions
describe(entryway) :- 
	write('You are in the entryway of a house'), nl,
	write('You see a room to the east and west. The door to outside is south and you can go upstairs.').

describe(kitchen) :-
	write('You enter a kitchen.'),nl,
	write('The only exit is back west.').
	
describe(dining_room) :-
	write('You enter a dining room.'),nl,
	write('The only exit is back east.').

describe(outside) :-
	write('You have escaped the house! Congratulations').

describe(upstairs_landing) :-
	write('You are upstairs.'),nl,
	write('There is a bedroom to the north.'), nl,
	write('You can go back downstairs.').

describe(bedroom) :-
	write('You enter a bedroom.'),nl,
	write('There is a closet to the east and a bathroom to the west. The exit is south.').

describe(closet) :-
	write('You are in a closet.'),nl,
	write('The exit is west.').

describe(bathroom) :-
	write('You are in a bathroom.'),nl,
	write('You can climb up through a window. The exit is back east.').

describe(roof) :-
	write('You are on the roof.'),nl,
	write('It is dark and you cannot see anything. You can go back down through the window.').

%The look empty predicate calls a series of other predicates that will show the room descriptions and any objects
look :- here(Place),
		describe(Place),
		nl,
		show_objects(Place),
		nl.

%The show_objects() predicates are used to show any items that exist in the room outside of the description
show_objects(Place) :- 
	item_at(X, Place),
	write('There is a '), write(X), write(' here.'), nl, fail.
	
show_objects(_).


%Predicates used to move to a different room
go(Direction) :-
	here(Curren_Location),
	move_to(Curren_Location, Direction, Next_Location),
	retract(here(Curren_Location)),
	assert(here(Next_Location)), look, !.
	
go(_) :- 
	write('Cannot go there.'), nl.

%Series of shortcut predicates to move in the cardinal directions.
n :- go(n).
s :- go(s).
e :- go(e).
w :- go(w).
u :- go(u).
d :- go(d).

%Help file to show commands
help :- write('------------Help File-------------------'), nl,
		write('|help - Shows the help file'), nl,
		write('|start - starts the game'), nl,
		write('|look - shows the room description of current room'), nl,
		write('|n,s,e,w,u,d - moves in the directions typed'), nl,
		write('|take(X) - allows you to take an item that appears in the room'), nl,
		write('|Make sure you always end your command with a period'), nl,
		write('|halt - end the game.'), nl.

start :- write('Welcome to CSCI 4342 Adventure Game'),nl,
		help, nl,nl, look.
