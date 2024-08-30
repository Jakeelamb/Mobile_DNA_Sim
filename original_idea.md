
![[TE_dilution.jpg]]

## Goal
1. How likely is Transposable element to land somewhere important?

### Idea
Adapt Conway's Game of Life Simulation:
![[giphy.webp]]
## Map:

*Double stranded DNA with a length comparable to relevant genome sizes*
![[Screenshot from 2023-11-09 11-28-44.png]]

2 x 0.18 Billion cubes in a row - Fly genome size
2 x 3 Billion cubes in a row - Human genome size
2 x (20 to 120) Billion cubes in a row - Salamander genome size
## Players: 

4 types of boxes: 
	Grey = non-coding
		Stationary
	White = exon
		Stationary
	Black = DNAtransposons
		Mobile, Cut and paste
	Red = Retrotransposons
		Mobile, Copy and paste

*Initial proportions will need to be established*
## Rules: 

Each turn, mobile players(Black and Red boxes) can either move or stay stationary
	If a mobile element intersects with a white box, then a point is added
	If a mobile element intersect with another mobile element, box length grows by sum of element lengths.
		If a black box intersects with a red box, result is red box
	*Additionally we can include chance of silencing (black/red box become Grey box)*
Red and Grey boxes can move both directions or across strands if they are nearby

## Score:
1. Game ends: calculate time it took to end (first time mobile element collides with an exon)
2. Points are subtracted: how many negative points accumulated after certain amount of time (Represents negative fitness cost)


## Expectation and Biological Implications:

1. TE mobility will matter less in larger genomes, compared to smaller genomes
		- This is because the number of Exons do not scale proportionately to genome size
		- Non coding regions (grey + black + red boxes) will dilute the critical regions (white boxes)  decreasing the probability that TE's will negatively impact a genome. 
2. It would be interesting to see if the score trend line(# of times a TE lands in an exon) vs genome size smallest to largest genomes
3. Additionally this simulation could help explain patterns of TEs distributions in genomes based on size.


# Progress 

TE_dilutoin_simulation_v1.py
Initial test failed 
Cant seem to prevent the simulation going to fixation.

New idea: Each round, reset the random distribution of 4 types of elements and record the statistics of where the mobile elements move and land and the interactions.
