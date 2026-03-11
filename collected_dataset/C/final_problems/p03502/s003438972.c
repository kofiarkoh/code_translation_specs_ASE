#include<stdio.h>

int main()
{
	int a,r,s=0;
	scanf("%d",&a);
	int b=a;

	while(a>0){
		r=a%10;
		s=s+r;
		a=a/10;


	}
	if(b%s==0){
		printf("Yes\n");
	}
	else{
		printf("No");
	}

	


	return 0;
}