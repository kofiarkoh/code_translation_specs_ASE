#include<stdio.h>
#include<limits.h>
int main(){
	int n, m;
	int i, j;
	while(scanf("%d%d", &n, &m), n) {
		int cost[100][100] = {0};
		int determined[100] = {0}, undeterminednodes;
		int mincost[100] = {0};
		int costsum = 0;
		int minnode, minnodecost;

		// initialize cost table
		for(i = 0; i < m; i ++) {
			for(j = 0; j < m; j ++) {
				cost[i][j] = INT_MAX;
			}
		}

		// load
		for(i = 0; i < m; i ++) {
			int a, b, d;
			scanf("%d,%d,%d", &a, &b, &d);
			cost[a][b] = cost[b][a] = d/100-1;
		}

		// node 0 is determined
		determined[0] = 1;
		undeterminednodes = m-1;

		// initialize mincost
		for(i = 0; i < m; i ++) {
			if(!determined[i]) {
				mincost[i] = cost[0][i];
			} else {
				mincost[i] = INT_MAX;
			}
		}

		while(undeterminednodes > 0) {
			// select the nearest undetermined node
			minnodecost = INT_MAX;
			for(i = 0; i < m; i ++) {
				if(mincost[i] < minnodecost) {
					minnode = i;
					minnodecost = mincost[i];
				}
			}

			// the node is determined
			determined[minnode] = 1;
			costsum += minnodecost;
			undeterminednodes --;
			mincost[minnode] = INT_MAX;

			// update mincost
			for(i = 0; i < m; i ++) {
				if(!determined[i]) {
					if(cost[minnode][i] < mincost[i])
						mincost[i] = cost[minnode][i];
					}
			}
		}
		printf("%d\n", costsum);
	}
	return 0;
}