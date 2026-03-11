#include <stdio.h>
#include <stdlib.h>
#define inf (int)(1e9)

int main(){
	int N, d, *m, **c, *nowval, *now, i, j, maxval, then_max, minval;
	while(1){
		scanf("%d%d", &N, &d);
		if(N == 0 && d == 0){
			return 0;
		}
		m = (int *)malloc(sizeof(int) * N);
		c = (int **)malloc(sizeof(int *) * N);
		nowval = (int *)malloc(sizeof(int) * N);
		now = (int *)malloc(sizeof(int) * N);
		for(i = 0; i < N; i++){
			scanf("%d", &m[i]);
			c[i] = (int *)malloc(sizeof(int) * m[i]);
			nowval[i] = 0;
			for(j = 0; j < m[i]; j++){
				scanf("%d", &c[i][j]);
				nowval[i] += c[i][j];
			}
			now[i] = 0;
		}
		while(1){
/*			printf("now\n");
			for(i = 0; i < N; i++){
				printf("%d ", now[i]);
			}
			printf("\n");
			printf("nowval\n");
			for(i = 0; i < N; i++){
				printf("%d ", nowval[i]);
			}
			printf("\n");
*/			maxval = -1;
			for(i = 0; i < N; i++){
				if(maxval < nowval[i]){
					maxval = nowval[i];
					then_max = i;
				}
				else if(maxval == nowval[i]){
					if(c[i][now[i]] < c[then_max][now[then_max]]){
						then_max = i;
					}
				}
			}
			if(maxval == 0){
				printf("Yes\n");
				break;
			}
			nowval[then_max] -= c[then_max][now[then_max]];
			now[then_max]++;
			maxval = -1;
			minval = inf;
			for(i = 0; i < N; i++){
				if(maxval < nowval[i]){
					maxval = nowval[i];
				}
				if(minval > nowval[i]){
					minval = nowval[i];
				}
			}
//			printf("(max, min) = (%d, %d)\n", maxval, minval);
			if(maxval - minval > d){
				printf("No\n");
				break;
			}
		}
	}
}
