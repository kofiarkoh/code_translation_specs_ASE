#include<stdio.h>
#include<math.h>

int main()
{
	int n=0,d,sw;
	long int i,j,k,s[1000],sum;

	while(1)
	{
		n=1;
		sum=0;
		scanf("%d",&d);
		if(!d)
			break;
		s[0]=2;
		s[1]=3;
		if(d<=2)
		{
			for(i=0;i<d;i++)
				sum+=s[i];
		}
		else
		{
			sum=5;
			for(i=5;;i+=2)
			{
				sw=0;
				k=(long int)sqrt((double)i);
				for(j=0;s[j]<=k;j++)
				{
					if(i%s[j]==0)
					{
						sw=1;
						break;
					}
				}
				if(!sw)
				{
					n++;
					if(n<1000)
						s[n]=i;
					sum+=i;
					if(n==d-1)
						break;
				}
			}
		}
		printf("%d\n",sum);
	}
	return 0;
}