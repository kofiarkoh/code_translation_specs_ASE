#include<stdio.h>
int W,D,H2,i,t,R;
int main()
{
	for(;scanf("%d%d",&W,&D),W;)
	{
		int H1[21]={0};
		for(i=R=0;i<W;R+=t,H1[t]++,i++)
			scanf("%d",&t);
		for(i=0;i<D;i++)
		{
			scanf("%d",&H2);
			if(H1[H2]>0)
				H1[H2]--;
			else
				R+=H2;
		}
		printf("%d\n",R);
	}
	return 0;
}