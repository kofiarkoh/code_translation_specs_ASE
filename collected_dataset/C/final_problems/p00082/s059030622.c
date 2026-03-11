#include<stdio.h>

int main()
{
	int wait[8];
	int jenny[15]={1,2,1,2,1,4,1,4,1,2,1,2,1,4,1};
	int priority[8]={7,3,6,2,4,0,5,1};
	int i,j,cannot,min,pos;

	while(scanf("%d %d %d %d %d %d %d %d",wait,wait+1,wait+2,wait+3,wait+4,wait+5,wait+6,wait+7)!=EOF)
	{
		for(i=0;i<8;i++)
		{
			cannot=0;
			for(j=0;j<8;j++)
			{
				if(wait[j]>jenny[i+j])
				{
					cannot+=wait[j]-jenny[i+j];
				}
			}
			if(i==0)
			{
				min=cannot;
				pos=i;
			}
			else if(min>cannot)
			{
				min=cannot;
				pos=i;
			}
			else if(min==cannot)
			{
				if(priority[i]>priority[pos])
				{
					min=cannot;
					pos=i;
				}
			}
		}
		printf("%d %d %d %d %d %d %d %d\n",jenny[pos],jenny[pos+1],jenny[pos+2],jenny[pos+3],jenny

[pos+4],jenny[pos+5],jenny[pos+6],jenny[pos+7]);
	}
	return 0;
}