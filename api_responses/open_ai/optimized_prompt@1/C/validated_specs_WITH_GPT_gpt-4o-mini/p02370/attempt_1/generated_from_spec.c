#include <stdio.h>
#include <stdlib.h>

typedef struct Node {
    int vertex;
    struct Node* next;
} Node;

typedef struct Graph {
    int numVertices;
    int* inDegree; // Array to store the in-degree of each vertex
    Node** adjList; // Adjacency list
} Graph;

// Function to create a new graph
Graph* createGraph(int vertices) {
    Graph* graph = (Graph*)malloc(sizeof(Graph));
    graph->numVertices = vertices;
    graph->inDegree = (int*)calloc(vertices, sizeof(int));
    graph->adjList = (Node**)malloc(vertices * sizeof(Node*));
    
    for (int i = 0; i < vertices; i++) {
        graph->adjList[i] = NULL; // Initialize adjacency list
    }
    
    return graph;
}

// Function to add an edge to the graph
void addEdge(Graph* graph, int src, int dest) {
    Node* newNode = (Node*)malloc(sizeof(Node));
    newNode->vertex = dest;
    newNode->next = graph->adjList[src];
    graph->adjList[src] = newNode;
    graph->inDegree[dest]++; // Increment in-degree of destination vertex
}

// Function to perform topological sort
void topologicalSort(Graph* graph) {
    int* queue = (int*)malloc(graph->numVertices * sizeof(int));
    int front = 0, rear = 0;

    // Enqueue all vertices with in-degree 0
    for (int i = 0; i < graph->numVertices; i++) {
        if (graph->inDegree[i] == 0) {
            queue[rear++] = i;
        }
    }

    // Process the queue
    while (front < rear) {
        int current = queue[front++]; // Dequeue
        printf("%d\n", current); // Output the vertex

        // Decrease the in-degree of adjacent vertices
        Node* temp = graph->adjList[current];
        while (temp) {
            int adjVertex = temp->vertex;
            graph->inDegree[adjVertex]--;
            // If in-degree becomes 0, enqueue it
            if (graph->inDegree[adjVertex] == 0) {
                queue[rear++] = adjVertex;
            }
            temp = temp->next;
        }
    }

    // Free allocated memory
    free(queue);
    for (int i = 0; i < graph->numVertices; i++) {
        Node* temp = graph->adjList[i];
        while (temp) {
            Node* toFree = temp;
            temp = temp->next;
            free(toFree);
        }
    }
    free(graph->adjList);
    free(graph->inDegree);
    free(graph);
}

int main() {
    int vertices, edges;
    scanf("%d %d", &vertices, &edges); // Read number of vertices and edges

    Graph* graph = createGraph(vertices);

    for (int i = 0; i < edges; i++) {
        int src, dest;
        scanf("%d %d", &src, &dest); // Read each edge
        addEdge(graph, src, dest);
    }

    topologicalSort(graph); // Perform topological sort

    return 0;
}
