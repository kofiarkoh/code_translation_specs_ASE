#include<stdio.h>

int main()
{
	int n,m,i,j,k,ans=0;
	scanf("%d",&n);
	char sn[100][15]={'a'}; 
	for(i=0;i<n;i++)
	{
		scanf("%s",&(sn[i]));
	}
	scanf("%d",&m);
	char sm[100][15]={'a'}; 
	for(i=0;i<m;i++)
	{
		scanf("%s",&(sm[i]));
	}
	
	for(i=0;i<n;i++)
	{
		int sum=0,flag=0;
		for(j=0;j<(n+m);j++)
		{
			flag=0;
			if(j<n)
			{
				for(k=0;k<10;k++)
				{
					if(sn[i][k]==sn[j][k])flag=1;
					else 
					{
						flag=0;
						break;
					}
				}
				if(flag)sum++;
			}
			else
			{
				for(k=0;k<10;k++)
				{
					if(sn[i][k]==sm[j-n][k])flag=1;
					else
					{
						flag=0;
						break;
					}
				}
				if(flag)sum--;
			}
		}
		//printf("%d\n",sum);
		if(sum>ans)ans=sum;
	}
	printf("%d",ans);
	return 0;
}