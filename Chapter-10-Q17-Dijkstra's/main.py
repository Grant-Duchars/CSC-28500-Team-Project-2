'''
Morgan Purcell
November 14th, 2022
Program to find the best path between two nodes on a graph using Dijkstra's search algorithm
'''

#import the math library
import math

#initialize the graph dictionary to hold the nodes within in the graph, edges, and weights
graph = {
    "A": {"C":1, "B":3, "G":2},
    "B": {"A":3},
    "C": {"A":1, "D":3, "E":8},
    "D": {"C":3, "E":2, "F":3},
    "E": {"C":8, "D":2, "F":1},
    "F": {"E":1, "D":3, "G":2},
    "G": {"A":2, "F":2}
}


#define the trackBack function to retrace the previous nodes from the end node to start node
def trackBack(startNode, endNode, prev_node):
    #initializd the current node to the end node
    currNode = endNode
    #initialize a list to hold the start node
    path = [startNode]
    #while the current node is not equal tot he start node
    while currNode != startNode:
        #insert each current node to the path list
        path.insert(1, currNode)
        #set the current node equal to the current nodes previous node
        currNode = prev_node[currNode]
    #print the path taken from the start node to end node
    print("The best route is: ")
    print(' -> '.join(path))



#define the findpath function to find the path and distance from a start node to end node
def findPath(startNode, endNode):
    #declare a dictionary to hold the current shortest distance from a node to the start node (priority queue)
    pq = {}
    #initialize the dict with the keys in the graph dict
    pq = graph.keys()
    #initialize the distances(values) to inf
    pq = dict.fromkeys(pq, math.inf)
    #set the starting nodes distance(value) to 0
    pq[startNode] = 0

    #declare a dictionary to hold each nodes previous node via the shortest path
    prev_node = {}
    #initialize the dict with the keys in the graph dict
    prev_node = graph.keys()
    #initialize the previous node(value) for each node to empty as a placeholder
    prev_node = dict.fromkeys(prev_node, "")

    #declare a dictionary to hold nodes that have been visited
    visited = []

    #while there are nodes in the pq dict
    while pq:
        #set the current node equal to the node with the shortest distance to the start node in the priority queue(pq) (visit the node)
        currNode = min(pq, key=pq.get)

        #iterate through the neighbours(nodes) of the current node
        for neighbour in graph[currNode]:
            #if the neighbour(node) has not already been visited 
            #and this path to the neighbour(node) is shorter than its current path in the priority queue, update the neighbour(node)
            if neighbour not in visited and pq[currNode] + graph[currNode][neighbour] < pq[neighbour]:
                #update the neighbours(nodes) distance from the start node
                pq[neighbour] = graph[currNode][neighbour] + pq[currNode]
                #update the neighbours(nodes) previous node to the current node
                prev_node[neighbour] = currNode

        #append the current node to the visited nodes once all neighbours have been viewed
        visited.append(currNode)

        #if the end node is still in the priority queue, update the shortest distance to it
        if endNode in pq:
            totalDist = pq[endNode]

        #remove currNode from the priority queue
        pq.pop(currNode)

    #print the total distance travelled from the start node to end node
    print("\nThe total distance travelled to", endNode, "was", totalDist)
    
    #call the trackBack function and pass the start node, end node, and previous node dictionary as arguments
    trackBack(startNode, endNode, prev_node)
            


#define the main function
def main():
    #print a message informing the user of the graph nodes
    print("\nThe graph includes the nodes A B C D E F G\n")
    #prompt for input to initialize the start node 
    startNode = input("Enter the start node: ")
    #if the input is not valid, inform the user and call main again
    if startNode not in graph:
        print("\nInvalid entry\n")
        main()

    #prompt for input to initialize the end node 
    endNode = input("Enter the end node: ")
    #if the input is not valid, inform the user and call main again
    if endNode not in graph:
        print("\nInvalid entry\n")
        main()

    #call the findPath function and pass the start and end nodes as arguments
    findPath(startNode, endNode)
    


#call the main function
if __name__ == '__main__':
    main()
