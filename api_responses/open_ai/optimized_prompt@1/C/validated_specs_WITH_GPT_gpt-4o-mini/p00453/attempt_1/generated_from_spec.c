#include <stdio.h>
#include <limits.h>

#define MAX_POINTS 100 // Assuming a reasonable upper limit for points

int main() {
    int n, m; // Number of points and maximum segments
    while (1) {
        // Read n and m
        scanf("%d %d", &n, &m);
        
        // Termination condition
        if (n == 0 && m == 0) {
            break;
        }

        // Initialize cost matrix
        int cost[MAX_POINTS][MAX_POINTS];
        for (int i = 0; i < MAX_POINTS; i++) {
            for (int j = 0; j < MAX_POINTS; j++) {
                cost[i][j] = INT_MAX; // High initial value
            }
        }

        // Process each point
        for (int i = 0; i < n; i++) {
            int num_coords;
            scanf("%d", &num_coords); // Read number of coordinates for the point
            
            for (int j = 0; j < num_coords; j++) {
                int x, weight;
                scanf("%d %d", &x, &weight); // Read coordinate and weight
                
                // Update cost matrix (assuming some logic to determine costs)
                // Here we just set the cost for the point to its weight for simplicity
                cost[i][i] = weight; // Cost to traverse to itself is its weight
            }
        }

        // Calculate minimum cost (placeholder logic)
        int min_cost = INT_MAX;
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                if (cost[i][j] < min_cost) {
                    min_cost = cost[i][j]; // Update minimum cost if a lower cost is found
                }
            }
        }

        // Output the minimum cost
        printf("%d\n", min_cost);
    }
    return 0;
}
