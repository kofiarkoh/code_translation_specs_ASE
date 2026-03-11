#include<stdio.h>
#include<stdlib.h>
#include<string.h>


#define NITEM(arr) (sizeof(arr)/sizeof(arr[0]))


typedef int (*cmp_t)(const void *, const void *);


#define N_MAX 5000
#define K_MAX 5000


int cmpint(const int *a, const int *b)
{
	return *b-*a;
}


int max(int a, int b)
{
	if(a>b) return a;
	return b;
}

int min(int a, int b)
{
	if(a<b) return a;
	return b;
}

int main(void)
{
	int N, K, i, j, k, need, prev_is_need;
	static int a[N_MAX];
	static char forward[N_MAX+1][K_MAX+1];
	static char backward[N_MAX+1][K_MAX+1];
	static char mix[K_MAX+1];

	while(scanf("%d%d", &N, &K)==2)
	{
		for(i=0;i<N;i++)
		{
			scanf("%d", &a[i]);
		}

		qsort(a, N, sizeof(a[0]), (cmp_t)cmpint);

		memset(forward, 0, sizeof(forward));
		memset(backward, 0, sizeof(backward));
		forward[0][0]=backward[N][0]=1;
		for(i=0;i<N;i++)
		{
			memcpy(forward[i+1], forward[i], sizeof(mix));
			for(j=0;j+a[i]<=K;j++)
			{
				forward[i+1][j+a[i]]|=forward[i][j];
			}
		}
		for(i=N-1;i>=0;i--)
		{
			memcpy(backward[i], backward[i+1], sizeof(mix));
			for(j=0;j+a[i]<=K;j++)
			{
				backward[i][j+a[i]]|=backward[i+1][j];
			}
		}
/*
		printf("forward\n");
		for(i=0;i<=N;i++)
		{
		for(j=0;j<=K;j++) printf(" %d", forward[i][j]);
		printf("\n");
		}

		printf("backward\n");
		for(i=0;i<=N;i++)
		{
		for(j=0;j<=K;j++) printf(" %d", backward[i][j]);
		printf("\n");
		}
*/
		need=0;
		prev_is_need=0;
		for(i=0;i<N;i++)
		{
			if(i && a[i]==a[i-1] && prev_is_need)
			{
				need++;
				continue;
			}

			prev_is_need=0;
			memset(mix, 0, sizeof(mix));
			for(j=0;j<K;j++)
			{
				if(forward[i][j]==0) continue;
				for(k=0;k+j<K;k++)
				{
					if(backward[i+1][k])
					{
						mix[k+j]|=backward[i+1][k];
						if(k+j>=K-a[i])
						{
							prev_is_need=1;
							need++;
							break;
						}
					}
				}
				if(prev_is_need) break;
			}
/*
			printf("%3d mix ", i);
			for(k=0;k<=K;k++) printf(" %d", mix[k]);
			printf("\n");
*/
/*
			prev_is_need=0;
			for(k=max(K-a[i], 0);k<K;k++)
			{
				if(mix[k])
				{
//					printf("need\n");
					prev_is_need=1;
					need++;
					break;
				}
			}
*/
		}
		printf("%d\n", N-need);
	}

	return 0;
}
