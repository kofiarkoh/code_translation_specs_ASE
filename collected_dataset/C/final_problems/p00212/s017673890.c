#include <stdio.h>

#define INF 100000000

struct NODE {
    int distance;
    int state;
} vertex[11][101];

int W[101][101];
int c, V, n, s, d;

int min(int a, int b) {
    return (a > b ? b : a);
}

void dijkstra(int start) {
    int i, x, p, step;
    for (i = 0; i <= c; i++) {
	for (x = 1; x <= n; x++) {
	    vertex[i][x].distance = INF; vertex[i][x].state = 0;
	}
	vertex[i][start].distance = 0; 
    }
    for (i = 0; i <= c; i++) {
	for (step = 1; step <= n; step++) {
	    int minimum = INF;
	    for (x = 1; x <= n; x++)
		if (vertex[i][x].state == 0 && vertex[i][x].distance < minimum) {
		    p = x; minimum = vertex[i][x].distance;
		}
	    vertex[i][p].state = 1;
	    for (x = 1 ; x <= n; x++)
		if (vertex[i][x].state == 0) {
		    if (i == 0)
			vertex[i][x].distance = min(vertex[i][x].distance, 
						    vertex[i][p].distance+W[p][x]);
		    else
			vertex[i][x].distance = min(vertex[i][x].distance, 
						    min(vertex[i][p].distance+W[p][x],
							vertex[i-1][p].distance+W[p][x]/2));
		}
	}
    }
}

int main(void) {
    int i, j;
    int from, to, m;
     
    while (1) {
	scanf("%d%d%d%d%d", &c, &V, &n, &s, &d);
        if (c + V + n + s + d == 0) break;
         
        for (i = 0; i <= V; i++)
            for (j = 0; j <= V; j++)
                W[i][j] = INF;
         
	for (i = 0; i < n; i++) {
            scanf("%d%d%d", &from, &to, &m);
            W[from][to] = m;
	    W[to][from] = m;
        }
         
        dijkstra(s);
	printf("%d\n", vertex[c][d].distance);
    }
    return 0;
}