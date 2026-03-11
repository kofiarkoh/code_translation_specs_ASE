#include <stdio.h>
#define INF 1000000

int cost[100][100];
int mincost[100];
char used[100];
int V;

int min(int a, int b){
	if (a > b){
		return (b);
	}
	return (a);
}

int prim(void)
{
	int i;
	int res;
	int u, v;
	
	for (i = 0; i < V; i++){
		mincost[i] = INF;
		used[i] = 0;
	}
	
	mincost[0] = 0;
	res = 0;
	
	while (1){
		v = -1;
		for (u = 0; u < V; u++){
			if (!used[u] && (v == -1 || mincost[u] < mincost[v])){
				v = u;
			}
		}
		
		if (v == -1){
			break;
		}
		used[v] = 1;
		res += mincost[v];
		
		for (u = 0; u < V; u++){
			mincost[u] = min(mincost[u], cost[v][u]);
		}
	}
	return (res);
}

int main(void)
{
	int num, from, to, dist;
	int i,j;
	
	while (1){
		scanf("%d", &V);
		if (V == 0){
			break;
		}
		scanf("%d", &num);
		
		for (i = 0; i < num; i++){
			for (j = 0; j < num; j++){
				cost[i][j] = INF;
			}
		}
		
		for (i = 0; i < num; i++){
			scanf("%d,%d,%d", &from, &to, &dist);
			cost[from][to] = cost[to][from] = (dist / 100) - 1;
		}
		
		printf("%d\n", prim());
	}
	return (0);
}