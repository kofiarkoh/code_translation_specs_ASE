#include <stdio.h>
int main(int argc, char *argv[])
{
	int i,n,k,a,sum=0;
    while(scanf("%d%d",&n,&k)!=EOF)
    {
    	sum=0;
      for(i=1;i<=n;i++)
      {
      	scanf("%d",&a);
      	if(k-a<a)sum+=2*(k-a);
      	else sum+=2*a;
      }  
	  printf("%d\n",sum);  
    }
	return 0;
}