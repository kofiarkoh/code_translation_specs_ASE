#include<stdio.h>
#include<stdlib.h>
#include<string.h>
int N,M,T[5005][2],i,j,x1,x2,y1,y2,R;
int in(int a,int b,int c){return a<=b&&b<=c;}
int cmp(const void *a,const void *b){return ((int *)a)[0]>((int*)b)[0];}
int main()
{
	scanf("%d%d",&N,&M);
	for(i=0;i<N;i++)
		scanf("%d%d",&T[i][0],&T[i][1]);
	qsort(T,N,sizeof(int[2]),cmp);
	for(i=0;i<M;i++)
	{
		scanf("%d%d%d%d",&x1,&y1,&x2,&y2);
		for(j=R=0;T[j][0]<=x2&&j<N;j++)
			if(in(x1,T[j][0],x2)&&in(y1,T[j][1],y2))
				R++;
		printf("%d\n",R);
	}
	return 0;
}