## Problem definition and info
- Eulerian path (traverse a graph passing through every edge once)
- Hamiltonian path (traverse a graph visiting every node once)
- Hamiltonian cycle, same as a Hamiltonian path but starting and ending at the same point
- Travelling salesman problem, find the Hamiltonian path/cycle with the lowest cost on a weighted directed graph
- TSP is NP-complete

## Time complexity
- The greedy algorithm has a complexity of O(n!)
- The Held-Karp algorithm uses dynamic programming to reach O(n^{2}2^{n})
  - Every sub-path along an optimal path is itself an optimal path, so we compute many smaller subpaths and sum them

## Heuristics
- Nearest neighbour
  - Find the next closest city that you haven't visited yet
- Christofides
  - Find MST for the network, join up nodes with an odd degree, find an eulerian path, convert back to TSP path
  - Guaranteed maximum 1.5 times the optimal weight
  - This requires the graphs relations to form a "metric space" (AB <= AC + CB). Luckily this is true in the real world
- 2, 3, k, v -opt
  - Delete 2 edges and put back 2 edges that make a better tour
  - Or 3 edges
  - Or k edges
  - Or a number of edges that changes as the process goes on (The best known method is Lin-Kernighan method)
- Markov chains
- Ant colony optimisation

## Metaheuristics
These methods are more robust in the sense that they can be more easily adapted to deal with a variety of side constraints. As such, the application of metaheuristic techniques is often preferred for large-scale applications with complicating constraints and decision sets.
- Genetic algorithms
- Tabu search
- Simulated annealing

## Papers
- History of the problem
  - https://www.math.uwaterloo.ca/tsp/history/biblio/1930.html
  - https://www.theorsociety.com/about-or/or-methods/heuristics/a-brief-history-of-the-travelling-salesman-problem/
  - https://opentransportationjournal.com/VOLUME/15/PAGE/93/FULLTEXT/
- Original paper
  - Menger’s Ergebnisse — a biographical introduction
  - https://doi.org/10.1007/978-3-7091-6470-9_2
  - HOW DO I ACCESS A PAPER BY DOI?
- An Effective Heuristic Algorithm for the Traveling-Salesman Problem
  - https://pubsonline.informs.org/doi/10.1287/opre.21.2.498
- A Stochastic Model of the Dynamic Vehicle Allocation Problem
  - https://pubsonline.informs.org/doi/abs/10.1287/trsc.20.2.117
  - WHAT DOES STOCHASTIC MEAN?
- An operational planning model for the dynamic vehicle allocation problem with uncertain demands
  - https://www.sciencedirect.com/science/article/abs/pii/0191261587900051
- The vehicle allocation problem: Alternative formulation and branch-and-price method
  - https://www.sciencedirect.com/science/article/abs/pii/S0305054822000764
- DYNAMIC VEHICLE ROUTING FOR DEMAND RESPONSIVE TRANSPORTATION SYSTEMS
  - https://core.ac.uk/download/pdf/302963211.pdf
- Potential Benefits of Demand Responsive Transport in Rural Areas
  - https://www.mdpi.com/2071-1050/14/6/3252/pdf

## Test data
http://akira.ruc.dk/~keld/research/LKH/

## Potential further research
Azure quantum https://www.thequblog.com/posts/traveling-salesperson-qio-maps/
AWS quantum https://github.com/aws/amazon-braket-examples/blob/main/examples/quantum_annealing/Dwave_TravelingSalesmanProblem/Dwave_TravelingSalesmanProblem.ipynb
AWS deep learning https://aws.amazon.com/blogs/opensource/solving-the-traveling-salesperson-problem-with-deep-reinforcement-learning-on-amazon-sagemaker/

## Bibliography
- Traveling Salesperson Problem. Brilliant.org. Retrieved 02:18, October 21, 2022, from https://brilliant.org/wiki/traveling-salesperson-problem/
- UK government benefits of DRT https://www.gov.uk/government/publications/demand-responsive-transport-local-authority-toolkit/demand-responsive-transport-local-authority-toolkit


It's not really a metric graph because of traffic jams etc.


https://stackoverflow.com/questions/49837125/confusion-about-np-hard-and-np-complete-in-traveling-salesman-problems