#include<stdio.h>
int main()
 {
    int N,i,sum=0;
    scanf("%d",&N);
    if(N>=1&&N<=100)
    {
    
    	for(i=1;i<=N;i++)
    	{
    		sum=sum+i;
    	}
    	   printf("%d\n",sum);
    }
      return 0;

}