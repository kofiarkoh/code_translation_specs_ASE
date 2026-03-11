#include<stdio.h>
#include<string.h>

int main()
{
	int n,i,j,c=0;
	int a[]={105,135,165,189,195};
	scanf("%d",&n);
	if(n<105)
	{
		printf("%d",c);
	}
	else
	{
		for(i=105;i<=n;i++)
		{
			for(j=0;j<5;j++)
			{
				if(i==a[j])
				{
					c++;
					break;
				}
				
			}
			
		}
		printf("%d",c);
	}
	
	
    return 0;  
}