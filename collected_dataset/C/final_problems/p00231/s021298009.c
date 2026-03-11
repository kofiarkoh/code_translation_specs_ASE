#include<stdio.h>
#include<string.h>
#include<stdlib.h>
typedef struct h{int m,a,b;}h;
int n,i,l,r,x;
h p[200],q[200];

void push(h t)
{
	int i;
	for(i=r;i>=l&&t.b<q[i-1].b;i--)
		q[i]=q[i-1];
	q[i]=t;r++;
}

int cmp(const void *x,const void *y)
{
	if(((h*)x)->a==((h*)y)->a)
		return ((h*)x)->b-((h*)y)->b;
	return ((h*)x)->a-((h*)y)->a;
}

int main()
{
	for(;scanf("%d",&n),n;)
	{
		for(i=0;i<n;i++)
			scanf("%d%d%d",&p[i].m,&p[i].a,&p[i].b);
		for(i=0;i<200;i++)
			q[i].m=q[i].a=q[i].b=2147483647;
		qsort(p,n,sizeof(h),cmp);
		for(i=r=x=l=0;i<n;)
		{
			for(;q[l].b>p[i].a;i++)
			{
				push(p[i]);
				x+=p[i].m;
				if(x>150)
				{
					puts("NG");
					goto e;
				}
			}
			for(;r>=l&&q[l].b<=p[i].a;l++)
				x-=q[l].m;
		}
		puts("OK");
		e:;
	}
	return 0;
}