#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>

typedef struct Road {
    int destination;
    int distance;
    struct Road* next;
} Road;

typedef struct City {
    char* name;
    Road* roads;
    int hasLPG;
} City;

typedef struct Graph {
    City* cities;
    int numCities;
    int fuelCapacity;
} Graph;

typedef struct Node {
    int cityIndex;
    int distance;
    int fuelRemaining;
    struct Node* next;
} Node;

typedef struct PriorityQueue {
    Node* head;
} PriorityQueue;

Graph* createGraph(int numCities, int fuelCapacity) {
    Graph* graph = (Graph*)malloc(sizeof(Graph));
    graph->cities = (City*)malloc(numCities * sizeof(City));
    graph->numCities = numCities;
    graph->fuelCapacity = fuelCapacity;
    for (int i = 0; i < numCities; i++) {
        graph->cities[i].name = NULL;
        graph->cities[i].roads = NULL;
        graph->cities[i].hasLPG = 0;
    }
    return graph;
}

void addCity(Graph* graph, int index, const char* name) {
    graph->cities[index].name = strdup(name);
}

void addRoad(Graph* graph, int from, int to, int distance) {
    Road* newRoad = (Road*)malloc(sizeof(Road));
    newRoad->destination = to;
    newRoad->distance = distance;
    newRoad->next = graph->cities[from].roads;
    graph->cities[from].roads = newRoad;
}

void setLPGStation(Graph* graph, int cityIndex) {
    graph->cities[cityIndex].hasLPG = 1;
}

PriorityQueue* createPriorityQueue() {
    PriorityQueue* pq = (PriorityQueue*)malloc(sizeof(PriorityQueue));
    pq->head = NULL;
    return pq;
}

void enqueue(PriorityQueue* pq, int cityIndex, int distance, int fuelRemaining) {
    Node* newNode = (Node*)malloc(sizeof(Node));
    newNode->cityIndex = cityIndex;
    newNode->distance = distance;
    newNode->fuelRemaining = fuelRemaining;
    newNode->next = NULL;

    if (!pq->head || pq->head->distance > distance) {
        newNode->next = pq->head;
        pq->head = newNode;
    } else {
        Node* current = pq->head;
        while (current->next && current->next->distance <= distance) {
            current = current->next;
        }
        newNode->next = current->next;
        current->next = newNode;
    }
}

Node* dequeue(PriorityQueue* pq) {
    if (!pq->head) return NULL;
    Node* temp = pq->head;
    pq->head = pq->head->next;
    return temp;
}

int findShortestPath(Graph* graph, int startCity, int goalCity) {
    PriorityQueue* pq = createPriorityQueue();
    enqueue(pq, startCity, 0, graph->fuelCapacity);

    int* distances = (int*)malloc(graph->numCities * sizeof(int));
    int* visited = (int*)calloc(graph->numCities, sizeof(int));
    for (int i = 0; i < graph->numCities; i++) {
        distances[i] = INT_MAX;
    }
    distances[startCity] = 0;

    while (pq->head) {
        Node* current = dequeue(pq);
        int cityIndex = current->cityIndex;
        int currentDistance = current->distance;
        int fuelRemaining = current->fuelRemaining;
        free(current);

        if (visited[cityIndex]) continue;
        visited[cityIndex] = 1;

        if (cityIndex == goalCity) {
            free(distances);
            free(visited);
            free(pq);
            return currentDistance;
        }

        Road* road = graph->cities[cityIndex].roads;
        while (road) {
            int nextCity = road->destination;
            int distanceToNext = road->distance;

            if (fuelRemaining >= distanceToNext) {
                int newDistance = currentDistance + distanceToNext;
                int newFuelRemaining = fuelRemaining - distanceToNext;

                if (newDistance < distances[nextCity]) {
                    distances[nextCity] = newDistance;
                    enqueue(pq, nextCity, newDistance, newFuelRemaining);
                }
            }
            road = road->next;
        }
    }

    free(distances);
    free(visited);
    free(pq);
    return -1; // No valid path found
}

void freeGraph(Graph* graph) {
    for (int i = 0; i < graph->numCities; i++) {
        free(graph->cities[i].name);
        Road* road = graph->cities[i].roads;
        while (road) {
            Road* temp = road;
            road = road->next;
            free(temp);
        }
    }
    free(graph->cities);
    free(graph);
}

int main() {
    // Example usage
    int numCities = 5;
    int fuelCapacity = 10;
    Graph* graph = createGraph(numCities, fuelCapacity);

    addCity(graph, 0, "CityA");
    addCity(graph, 1, "CityB");
    addCity(graph, 2, "CityC");
    addCity(graph, 3, "CityD");
    addCity(graph, 4, "CityE");

    addRoad(graph, 0, 1, 3);
    addRoad(graph, 1, 2, 4);
    addRoad(graph, 0, 2, 5);
    addRoad(graph, 2, 3, 2);
    addRoad(graph, 3, 4, 1);

    setLPGStation(graph, 1); // CityB has an LPG station

    int result = findShortestPath(graph, 0, 4);
    printf("Minimum distance: %d\n", result);

    freeGraph(graph);
    return 0;
}
