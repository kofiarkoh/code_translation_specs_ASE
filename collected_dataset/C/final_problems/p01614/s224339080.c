#include<stdio.h>
#include<stdlib.h>
typedef struct d{int s,l,p;}d;
d D[400];
int N,M,W,i,j,r[400];
int c(const void *a, const void *b)
{
	d*x=(d*)a;
	d*y=(d*)b;
	return x->p==y->p?x->s>y->s:x->p<y->p;
}

int f(int w)
{
	int d=0,i,r=0;
	for(i=0;i<N;)
	{
		if(D[i].s<=w)
		{
			r+=D[i].p;
			d+=D[i].l-D[i].s;
			w-=D[i].s;
		}
		else
			i++;
	}
	return w<=d?r:-1;
}

int main()
{
	scanf("%d",&N);
	for(i=0;i<N;i++)scanf("%d%d%d",&D[i].s,&D[i].l,&D[i].p);
	
	qsort(D,N,sizeof(d),c);
	scanf("%d",&M);
	for(i=0;i<M;i++)
	{
		scanf("%d",&W);
		r[i]=f(W);
		if(r[i]==-1)
		{
			r[0]=-1;
			break;
		}
	}
	
	for(i=0;i<M;i++)
	{
		printf("%d\n",r[i]);
		if(r[i]==-1)break;
	}
	return 0;
}