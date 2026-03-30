#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_NAME_LENGTH 11 // 10 characters + 1 for null terminator

typedef struct Ingredient {
    char name[MAX_NAME_LENGTH];
    int cost;
    struct Ingredient** connections; // Adjacency list for connections
    int connection_count;
    int visited; // For DFS traversal
    int min_cost; // Minimum cost in the connected component
} Ingredient;

typedef struct Graph {
    Ingredient* ingredients;
    int ingredient_count;
} Graph;

// Function prototypes
Graph* create_graph(int n);
void add_ingredient(Graph* graph, int index, const char* name, int cost);
void add_connection(Graph* graph, const char* name1, const char* name2);
void dfs(Ingredient* ingredient);
int calculate_total_min_cost(Graph* graph);
void free_graph(Graph* graph);

int main() {
    int n; // Number of ingredients
    scanf("%d", &n);
    
    Graph* graph = create_graph(n);
    
    for (int i = 0; i < n; i++) {
        char name[MAX_NAME_LENGTH];
        int cost;
        scanf("%s %d", name, &cost);
        add_ingredient(graph, i, name, cost);
    }
    
    int m; // Number of connections
    scanf("%d", &m);
    
    for (int i = 0; i < m; i++) {
        char name1[MAX_NAME_LENGTH], name2[MAX_NAME_LENGTH];
        scanf("%s %s", name1, name2);
        add_connection(graph, name1, name2);
    }
    
    int total_min_cost = calculate_total_min_cost(graph);
    printf("%d\n", total_min_cost);
    
    free_graph(graph);
    return 0;
}

Graph* create_graph(int n) {
    Graph* graph = (Graph*)malloc(sizeof(Graph));
    graph->ingredients = (Ingredient*)malloc(n * sizeof(Ingredient));
    graph->ingredient_count = n;
    
    for (int i = 0; i < n; i++) {
        graph->ingredients[i].connections = NULL;
        graph->ingredients[i].connection_count = 0;
        graph->ingredients[i].visited = 0;
        graph->ingredients[i].min_cost = -1; // Initialize to -1 to indicate unvisited
    }
    
    return graph;
}

void add_ingredient(Graph* graph, int index, const char* name, int cost) {
    strncpy(graph->ingredients[index].name, name, MAX_NAME_LENGTH);
    graph->ingredients[index].cost = cost;
}

void add_connection(Graph* graph, const char* name1, const char* name2) {
    Ingredient* ing1 = NULL;
    Ingredient* ing2 = NULL;
    
    // Find the ingredients by name
    for (int i = 0; i < graph->ingredient_count; i++) {
        if (strcmp(graph->ingredients[i].name, name1) == 0) {
            ing1 = &graph->ingredients[i];
        }
        if (strcmp(graph->ingredients[i].name, name2) == 0) {
            ing2 = &graph->ingredients[i];
        }
    }
    
    // If either ingredient is not found, behavior is undefined (as per spec)
    if (ing1 && ing2) {
        // Add connection in both directions
        ing1->connections = realloc(ing1->connections, (ing1->connection_count + 1) * sizeof(Ingredient*));
        ing1->connections[ing1->connection_count++] = ing2;
        
        ing2->connections = realloc(ing2->connections, (ing2->connection_count + 1) * sizeof(Ingredient*));
        ing2->connections[ing2->connection_count++] = ing1;
    }
}

void dfs(Ingredient* ingredient) {
    ingredient->visited = 1; // Mark as visited
    if (ingredient->min_cost == -1 || ingredient->cost < ingredient->min_cost) {
        ingredient->min_cost = ingredient->cost; // Update minimum cost
    }
    
    for (int i = 0; i < ingredient->connection_count; i++) {
        Ingredient* neighbor = ingredient->connections[i];
        if (!neighbor->visited) {
            dfs(neighbor); // Recursive DFS
            if (neighbor->min_cost < ingredient->min_cost) {
                ingredient->min_cost = neighbor->min_cost; // Update minimum cost from neighbor
            }
        }
    }
}

int calculate_total_min_cost(Graph* graph) {
    int total_cost = 0;
    
    for (int i = 0; i < graph->ingredient_count; i++) {
        if (!graph->ingredients[i].visited) {
            dfs(&graph->ingredients[i]); // Perform DFS for unvisited ingredients
        }
        total_cost += graph->ingredients[i].min_cost; // Aggregate minimum costs
    }
    
    return total_cost;
}

void free_graph(Graph* graph) {
    for (int i = 0; i < graph->ingredient_count; i++) {
        free(graph->ingredients[i].connections); // Free connections
    }
    free(graph->ingredients); // Free ingredients array
    free(graph); // Free graph structure
}
