#include<stdio.h>
#define MIN(a,b) (((a)<(b))?(a):(b))
#define INF 100000

int m[10][10];

int main(void)
{
	int n,a,b,time,t_num,min,index;
	int i,j,k;

	while (1) {
		scanf("%d", &n);
		if (n == 0)break;
		int sum[10] = { 0 };
		int town[10] = { 0 };
		t_num = 0;
		for (i = 0; i < n; i++) {
			for (j = 0; j < n; j++)
				if (i == j)m[i][j] = 0;
				else
				m[i][j] = INF;
		}

		for (i = 0; i < n; i++) {
			scanf("%d %d %d", &a, &b, &time);
			town[a] = 1,town[b]=1;
			m[a][b] = time;
			m[b][a] = time;
		}

		for (i = 0; i < 10; i++) {
			if (town[i] == 1)t_num++;
		}

		for (i = 0; i < t_num; i++) {
			for (j = 0; j < t_num; j++) 
				for (k = 0; k < t_num; k++) 
					m[j][k] = MIN(m[j][k], m[j][i] + m[i][k]);
		}
		for (i = 0; i < t_num; i++) {
			for (j = 0; j < t_num; j++)
				sum[i] += m[i][j];
		}
		index = 0;
		min = sum[0];
		for (i = 1; i < t_num; i++) {
			if (min > sum[i]) {
				min = sum[i];
				index = i;
			}
		}
		printf("%d %d\n",index,min);
	}
	return 0;
}