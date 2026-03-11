#include<stdio.h>
#include<stdlib.h>
int N,r,l,q[10000],H,S,M,T,R;
int I()
{
	scanf("%d:%d:%d",&H,&M,&S);
	return H*3600+S*60+M;
}

int c(const void *a,const void *b)
{
	return *(int*)a-*(int*)b;
}

int main()
{
	for(;scanf("%d",&N),N;)
	{
		r=l=R=0;
		for(;N--;)
		{
			T=I();
			if(r<l&&q[r]<=T)r++;
			else R++;
			T=I();
			q[l++]=T;
			qsort(q+r,l-r,4,c);
		}
		printf("%d\n",R);
	}
	return 0;
}