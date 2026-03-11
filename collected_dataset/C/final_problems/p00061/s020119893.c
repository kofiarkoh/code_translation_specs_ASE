#include<stdio.h>
typedef struct Team
{
	int num;
	int solved;
	int rank;
}Team;

void sort(Team *t,int end)
{
	int i,j;
	Team tmp;
	for(i=0;i<end-1;i++)
	{
		for(j=i+1;j>0;j--)
		{
			if(t[j].solved>t[j-1].solved)
			{
				tmp=t[j];
				t[j]=t[j-1];
				t[j-1]=tmp;
			}
		}
	}
}

int main()
{
	Team team[10000];
	int i=0,n,s,end,r=1,maxsol=30;
	while(1)
	{
		scanf("%d,%d",&n,&s);
		if(n==0&&s==0)break;
		team[i].num=n;
		team[i].solved=s;
		i++;
	}
	end=i;
	sort(team,end);
	for(i=0;i<end;i++)
	{
		if(team[i].solved<maxsol)
		{
			r++;
			maxsol=team[i].solved;
		}
		team[i].rank=r;
	}
	while(scanf("%d",&n)!=EOF)
	{
		for(i=0;i<end;i++)
		{
			if(team[i].num==n)
			{
				printf("%d\n",team[i].rank);
				break;
			}
		}
	}
	return 0;
}