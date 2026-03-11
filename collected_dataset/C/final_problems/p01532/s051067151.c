#include<stdio.h>

int main()
{
	int N,T,H,L,t[100],h[100],c=0,n=0;
	scanf("%d%d%d%d",&N,&T,&H,&L);
	for(int i=0;i<N;i++)
		scanf("%d%d",&t[i],&h[i]);
	while(1)
	{
		c=n%N;
		if(t[c]<=0 && h[c]<=0)
		{
			printf("%d\n",c+1);
			break;
		}
		else
		{
			if(t[c]==0)
			{
				h[c]-=1;
				t[c]+=T+10-9;
				T-=T+10-9;
			}
			else
			{
				t[c]--;
				T++;
			}
		}
		if(T<0 || T>L)
		{
			printf("%d\n",c+1);
			break;
		}
		if(T>=9)
		{
			t[c]+=T-9;
			T-=T-9;
		}
		n++;
	}
}