#include <stdio.h>
#include <stdlib.h>

typedef struct
{
	int c;
	int m[4];
	int s[4];
} TEAM;

int comparison(const void* a, const void* b);

int main(void)
{
	int n;
	TEAM* team;
	
	while (scanf("%d", &n) * n != 0){
		int i;
		
		team = (TEAM*)calloc(n, sizeof(TEAM));
		for (i = 0; i < n; i++){
			int j;
			
			scanf("%d", &team[i].c);
			for (j = 0; j < 4; j++){
				scanf("%d %d", &team[i].m[j], &team[i].s[j]);
			}
		}
		qsort(team, n, sizeof(TEAM), comparison);
		printf("%d\n", team[0].c);
		printf("%d\n", team[1].c);
		printf("%d\n", team[n - 2].c);
		
		free(team);
	}
	
	return 0;
}

int comparison(const void* a, const void* b)
{
	int sum_a, sum_b;
	int i;
	
	sum_a = sum_b = 0;
	for (i = 0; i < 4; i++){
		sum_a += ((TEAM*)a)->m[i] * 60 + ((TEAM*)a)->s[i];
		sum_b += ((TEAM*)b)->m[i] * 60 + ((TEAM*)b)->s[i];
	}
	if (sum_a > sum_b){
		return 1;
	}
	else if (sum_a < sum_b){
		return -1;
	}
	else {
		return 0;
	}
}