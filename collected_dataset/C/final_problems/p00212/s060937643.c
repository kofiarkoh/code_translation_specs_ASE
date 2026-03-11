#include <stdio.h>
#include <string.h>

#define NODE (100)
#define TICKET (10)
#define INF (100000000)

int U[NODE];
int dist[TICKET + 1][NODE];
int cost[NODE][NODE];
int c, n, s, d;
int V;

int min(int a, int b)
{
    return (a > b ? b : a);
}

void dijkstra(int s)
{
	int i, j, k;
	int v;
	
	for (i = 0; i < V; i++){
		U[i] = 1;
	}
    
    for (i = 0; i <= c; i++){
        for (j = 0; j < V; j++){
            dist[i][j] = INF;
        }
    }
    
    
    for (i = 0; i <= c; i++){
        dist[i][s] = 0;
    }
    for (k = 0; k <= c; k++){
        memset(U, 0, sizeof(U));
    	for (i = 0; i < V; i++){
    		v = -1;
		    for (j = 0; j < V; j++){
			    if (U[j] == 0 && (v == -1 || dist[k][v] > dist[k][j])){
				    v = j;
    			}
	    	}
		    
    		if (v == -1){
	    		break;
		    }
		    
    		U[v] = 1;
            
		    for (j = 0; j < V; j++){
                if (U[j] == 0){
                    if (k == 0){
                        dist[k][j] = min(dist[k][j], dist[k][v] + cost[v][j]);
                    }
                    else {
                        dist[k][j] = min(dist[k][j], min(dist[k][v] + cost[v][j], dist[k - 1][v] + cost[v][j] / 2));
                    }
                }
		    }
	    }
    }
}

int main(void)
{
	int i, j;
	int from, to, m;
	
	while (1){
		scanf("%d%d%d%d%d", &c, &V, &n, &s, &d);
		
		if (c + V + n + s + d == 0){
			break;
		}
		
		for (i = 0; i < V; i++){
			for (j = 0; j < V; j++){
				cost[i][j] = INF;
			}
		}
		
		while (n-- != 0){
			scanf("%d%d%d", &from, &to, &m);
			cost[from - 1][to - 1] = cost[to - 1][from - 1] = m;
		}
        
		dijkstra(s - 1);
        
        printf("%d\n", dist[c][d - 1]);
	}
	return (0);
}