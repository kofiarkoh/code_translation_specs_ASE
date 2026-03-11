#include<stdio.h>
#include<stdlib.h>
#include<string.h>

typedef long long ll;

#define MODU 1000000007LL

ll gcd(ll a, ll b)
{
	ll c;

	while((c=a%b))
	{
		a=b;
		b=c;
	}
	return b;
}


ll nCm(int n, int m)
{
	int work[100];
	int i, j, t, d;
	ll r;

	if(m*2>n) m=n-m;

	for(i=0;i<m;i++) work[i]=n-i;
	for(i=1;i<=m;i++)
	{
		t=i;
		for(j=0;j<m && t>1;j++)
		{
			d=gcd(work[j], t);
			if(d>1)
			{
				work[j]/=d;
				t/=d;
			}
		}
	}

	r=1;
	for(i=0;i<m;i++)
	{
		r=(r*work[i])%MODU;
	}
	return r;
}


typedef struct
{
	int c, w;
}ball_t;

#define N_MAX 200000

ball_t ball[N_MAX+1];
int c_num[N_MAX+1];
int movable[N_MAX+1];


int cmp(const void *pa, const void *pb)
{
	const ball_t *a, *b;

	a=pa;
	b=pb;
	if(a->w<b->w) return -1;
	return (a->w>b->w);
}


ll calc(int c_num[], int n)
{
	ll ret=1;
	int i, j, sum;

	sum=0;
	for(i=0;i<n;i++)
	{
		if(c_num[i]==0) continue;

		sum+=c_num[i];
		ret=(ret*nCm(sum, c_num[i]))%MODU;
	}
	return ret;
}


int main(void)
{
	int N, X, Y;
	int i, second;

	while(scanf("%d%d%d", &N,&X,&Y)==3)
	{
		for(i=0;i<N;i++)
		{
			scanf("%d%d", &ball[i].c, &ball[i].w);
		}
		qsort(ball, N, sizeof(ball[0]), cmp);

		for(second=1;second<N;second++)
		{
			if(ball[second].c!=ball[0].c) break;
		}

		if(second>=N || ball[0].w+ball[second].w>Y)
		{
			printf("1\n");
			continue;
		}

		memset(movable, 0, sizeof(movable));
		for(i=0;i<N;i++)
		{
			if(ball[0].c==ball[i].c)
			{
				if(ball[0].w+ball[i].w<=X) movable[i]=1;
			}
			else
			{
				if(ball[0].w+ball[i].w<=Y) movable[i]=1;
			}
			if(ball[second].c==ball[i].c)
			{
				if(ball[second].w+ball[i].w<=X) movable[i]=1;
			}
			else
			{
				if(ball[second].w+ball[i].w<=Y) movable[i]=1;
			}
		}

		memset(c_num, 0, sizeof(c_num));
		for(i=0;i<N;i++)
		{
			if(movable[i]) c_num[ball[i].c]++;
		}
/*
		printf("second %d : c=%d w=%d\n", second, ball[second].c, ball[second].w);

		printf("mova  :");
		for(i=0;i<N;i++) printf(" %d", movable[i]);
		printf("\n");

		printf("c_num :");
		for(i=0;i<N;i++) printf(" %d", c_num[i+1]);
		printf("\n");
*/
		printf("%d\n", (int)calc(c_num+1, N));
	}
	return 0;
}
