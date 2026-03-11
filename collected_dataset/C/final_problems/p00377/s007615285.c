#include<stdio.h>
int main(void)
{
	int n,a,c,p[100],i,sum;
	sum=0;
	scanf("%d %d",&n,&c);
	for(i=0;i<c;i++){
		scanf("%d",&p[i]);
		sum+=p[i];
	}
	n+=1;
	a=sum/n;
	if(sum%n!=0){
		a+=1;
	}
	printf("%d\n",a);
	return 0;
}
	
