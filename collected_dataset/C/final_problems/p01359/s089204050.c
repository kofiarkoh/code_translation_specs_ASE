#include<stdio.h>
typedef struct D{int f,e;char n[20];}D;
int N,Q,i,q;
D Y[1005];
int main()
{
	for(;scanf("%d%d",&N,&Q),N;)
	{
		for(i=0;i<N;Y[i].f=Y[i].e-Y[i].f+1,i++)
			scanf("%s%d%d",Y[i].n,&Y[i].f,&Y[i].e);
		for(;Q--;)
		{
			scanf("%d",&q);
			for(i=0;i<N;i++)
				if(Y[i].f<=q&&q<=Y[i].e)
				{
					printf("%s %d\n",Y[i].n,q-Y[i].f+1);
					break;
				}
			if(i==N)
				puts("Unknown");
		}
	}
	return 0;
}