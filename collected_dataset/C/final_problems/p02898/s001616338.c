#include<stdio.h>
int main(void)
{
	int x;
	int n;
		scanf("%d %d",&x,&n);
		char ch;
		ch=getchar();
		int l=2*x;
	
//	int l=2*x;
	int number[l];
	for(int jj=0;jj<l;jj++)
	{
	    number[jj]=0;
	}
	int t=0;
//	scanf("%d %d",&x,&n);
	for(int i=0;i<x;i++)
	{
	    
		scanf("%d",&number[i]);
		
	}
	for(int m=0;m<x;m++)
	{
	    if(number[m]>=n)
		t++;
	}
	printf("%d",t);
	
	return 0;
	} 