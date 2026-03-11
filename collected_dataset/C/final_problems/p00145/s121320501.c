#include<stdio.h>
#include<string.h>
typedef struct d{int card[100][2],cost;}d;

int N,C[100][2],i,j,k,l,p;
d dp[105][105];
int main()
{
	scanf("%d",&N);
	for(i=0;i<N;i++)
		scanf("%d%d",&C[i][0],&C[i][1]);
	for(i=0;i<N;i++)
	{
		memcpy(dp[0][i].card,C,sizeof(C));
		dp[0][i].cost=0;
	}
	for(i=1;i<N;i++)
		for(j=0;j<N-i;j++)
		{
			d *s=&dp[i][j];
			s->cost=2147483647;
			for(k=0;k<N-i+1;k++)
			{
				d t=dp[i-1][k];
				int nc=t.cost+t.card[j][0]*t.card[j][1]*t.card[j+1][0]*t.card[j+1][1];
				if(s->cost>nc)
				{
					s->cost=nc;
					for(l=p=0;l<N-i+1;)
					{
						if(l==j)
						{
							s->card[p][0]=t.card[l][0];
							s->card[p][1]=t.card[l+1][1];
							l+=2;
							p++;
						}
						else
						{
							s->card[p][0]=t.card[l][0];
							s->card[p][1]=t.card[l][1];
							l++;
							p++;
						}
					}
				}
			}
		}
	printf("%d\n",dp[N-1][0].cost);
	return 0;
}