#include<stdio.h>
#include<stdlib.h>

double expect()
{
	int N,M,V[105],i;
	double R[105],EXP=0.0,B=0.0;
	scanf("%d%d",&N,&M);
	for(i=0;i<M;i++)
	{
		scanf("%d%lf",&V[i],&R[i]);
		B+=R[i];
	}
	for(i=0;i<M;i++)
	{
		R[i]=R[i]/B;
		EXP+=V[i]*R[i];
	}
	return EXP;
}

int main()
{
	int i,T;
	double my[10]={0.0},hn=0;
	scanf("%d",&T);
	for(i=0;i<T;i++)
		my[i]=expect();
	hn=expect();
	for(i=0;i<T;i++)
		if(my[i]-hn>0.0000001)
		{
			puts("YES");
			return 0;
		}
	puts("NO");
	return 0;
}