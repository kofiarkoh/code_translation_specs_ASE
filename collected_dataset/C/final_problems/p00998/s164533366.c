#include<stdio.h>
#include<stdlib.h>
#define min(a,b)(a)<(b)?(a):(b)
#define  MAX_N 1<<17
int N;
int num[MAX_N];
int tree[2*MAX_N-1];

void init(int _n)
{
	int i;
	for(N=1;N<=_n;N*=2);
	for(i=0;i<2*N-1;i++)
		tree[i]=10000005;
}

void update(int p,int a)
{
	p+=N-1;
	tree[p]=a;
	while(p>0)
	{
		p=(p-1)/2;
		tree[p]=min(tree[2*p+1],tree[2*p+2]);
	}
}

void shift(int l,int r)
{
	int i,t;
	t=tree[r+N-1];
	for(i=r-1;i>=l;i--)
		update(i+1,tree[i+N-1]);
	update(l,t);
}


int query(int a,int b,int k,int l,int r)
{
	if(r<=a||b<=l)
		return 10000005;
	if(a<=l&&r<=b)
		return tree[k];
	else
	{
		int t1=query(a,b,2*k+1,l,(l+r)/2);
		int t2=query(a,b,2*k+2,(l+r)/2,r);
		return min(t1,t2);
	}
}

int main()
{
	int n,i,t,q,x,y,z;
	scanf("%d%d",&n,&q);
	init(n);
	for(i=0;i<n;scanf("%d",&t),update(i++,t));
	for(;q--;)
	{
		scanf("%d%d%d",&x,&y,&z);
		switch(x)
		{
		case 0:shift(y,z);break;
		case 1:printf("%d\n",query(y,z+1,0,0,N));break;
		case 2:update(y,z);break;
		}
	}
	return 0;
}