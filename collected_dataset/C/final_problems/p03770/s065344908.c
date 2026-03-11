#include<stdio.h>
#include<stdlib.h>
#include<string.h>

typedef long long ll;

#define NITEM(arr) (sizeof(arr)/sizeof(arr[0]))
#define MODU 1000000007LL

#define N_MAX 200000


typedef struct
{
	int p, n;
}fact_t;

int prime[N_MAX/4];
int factn[NITEM(prime)];
int prime_num;
fact_t *fact[N_MAX+1];

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


void prime_init(void)
{
	int i, j;

	if(prime_num) return;

	for(i=2;i<=N_MAX;i++)
	{
		for(j=2;j*j<=i;j++)
		{
			if(i%j==0)
			{
				break;
			}
		}
		if(j*j>i) prime[prime_num++]=i;
	}
}


void fact_init(void)
{
	fact_t work[32];
	int n, x, i, j;

	if(fact[0]) return;
	prime_init();
	for(i=1;i<=N_MAX;i++)
	{
		x=i;
		n=0;
		for(j=0;prime[j]*prime[j]<=x;j++)
		{
			if(x%prime[j]==0)
			{
				work[n].p=j;
				work[n].n=0;
				while(x%prime[j]==0)
				{
					x/=prime[j];
					work[n].n++;
				}
				n++;
			}
		}
		if(x>1)
		{
			for(;x>prime[j];j++)
				;
			work[n].p=j;
			work[n].n=1;
			n++;
		}
		fact[i]=malloc(sizeof(fact_t)*(n+1));
		for(j=0;j<n;j++) fact[i][j]=work[j];
		fact[i][n].p=-1;
	}
}


ll nCm(int n, int m)
{
	int i, j;
	ll r;
	fact_t *u, *l;

	if(m*2>n) m=n-m;

	memset(factn, 0, sizeof(factn));
	for(i=0;i<m;i++)
	{
		u=fact[n-i];
		l=fact[i+1];
		for(j=0;u[j].p>=0;j++) factn[u[j].p]+=u[j].n;
		for(j=0;l[j].p>=0;j++) factn[l[j].p]-=l[j].n;
	}

	r=1;
	for(i=0;i<NITEM(factn);i++)
	{
		for(j=0;j<factn[i];j++)
		{
			r=(r*prime[i])%MODU;
		}
	}
	return r;
}


typedef struct
{
	int c, w;
}ball_t;


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


int cmp2(const void *pa, const void *pb)
{
	const ball_t *a, *b;

	a=pa;
	b=pb;
	if(a->c<b->c) return -1;
	if(a->c>b->c) return 1;

	if(a->w<b->w) return -1;
	return (a->w>b->w);
}


ll calc(int c_num[], int n)
{
	ll ret=1;
	int i, sum;

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
	int i, second, light;

	fact_init();
	while(scanf("%d%d%d", &N,&X,&Y)==3)
	{
		for(i=0;i<N;i++)
		{
			scanf("%d%d", &ball[i].c, &ball[i].w);
		}

		qsort(ball, N, sizeof(ball[0]), cmp2);
		light=0;
		for(i=1;i<N;i++)
		{
			if(ball[light].c==ball[i].c)
			{
				if(ball[light].w+ball[i].w<=X)
				{
					ball[i].w=ball[light].w;
				}
			}
			else
			{
				light=i;
			}
		}

		qsort(ball, N, sizeof(ball[0]), cmp);

		for(second=1;second<N;second++)
		{
			if(ball[second].c!=ball[0].c) break;
		}

		if(second>=N)
		{
			printf("1\n");
			continue;
		}

		memset(movable, 0, sizeof(movable));
		for(i=0;i<N;i++)
		{
			if(ball[0].c!=ball[i].c)
			{
				if(ball[0].w+ball[i].w<=Y) movable[i]=1;
			}
			if(ball[second].c!=ball[i].c)
			{
				if(ball[second].w+ball[i].w<=Y) movable[i]=2;
			}
		}

		memset(c_num, 0, sizeof(c_num));
		for(i=0;i<N;i++)
		{
			if(movable[i]) c_num[ball[i].c]++;
		}
		printf("%d\n", (int)calc(c_num+1, N));
	}
	return 0;
}
