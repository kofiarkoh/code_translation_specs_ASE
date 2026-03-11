#include<stdio.h>

struct num
{
    int n1;
    int n2;
};

int ifpass(int s1,int s2,struct num narr[],int m,int n);

int main()
{
    int n,m,i,j,ans=0;
    scanf("%d %d",&n,&m);
    int a[100000];
    for(i=0;i<n;i++)
    {
        scanf("%d",&(a[i]));
    }
    struct num narr[100000]={0};
    int p1,p2;
    for(i=0;i<m;i++)
    {
        scanf("%d %d",&p1,&p2);
        if(p1<p2)narr[i].n1=p1,narr[i].n2=p2;
        else narr[i].n1=p2,narr[i].n2=p1;
    }
    for(i=1;i<=n;i++)
    {
        for(j=0;j<n;j++)
        {
            if(i==a[j])
            {	//printf("\n%d to %d then %d",i,j+1,ifpass(i,(j+1),narr,m,n));
                ans+=ifpass(i,(j+1),narr,m,n);
            }
        }
    }
	printf("%d",ans);
}

int ifpass(int s1,int s2,struct num narr[],int m,int n)
{
	//printf("pass(%d %d)",s1,s2);
    if(s1==s2)return 1;
    int k,k1,k2,flag=0;
	int yarr[100001]={0};
	yarr[s1]=1;//キャリア
	while(!flag)
	{
		flag=1;
		for(k=0;k<m;k++)
		{
			if(yarr[narr[k].n1]==1)
			{
				if(yarr[narr[k].n2]!=1)
				{
					yarr[narr[k].n2]=1;
					flag=0;
				}
			}
			else if(yarr[narr[k].n2]==1)
			{
				if(yarr[narr[k].n1]!=1)
				{
					yarr[narr[k].n1]=1;
					flag=0;
				}
			}
		}
	}
	
    return yarr[s2];
}