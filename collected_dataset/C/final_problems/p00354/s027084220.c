#include<stdio.h>
int main(void)
{
	int x,n;
	scanf("%d",&x);
	n=x%7;
	if(n==1){
	printf("fri\n");
	}
	else if(n==2){
	printf("sat\n");
	}
	else if(n==3){
	printf("sun\n");
	}
	else if(n==4){
	printf("mon\n");
	}
	else if(n==5){
	printf("tue\n");
	}
	else if(n==6){
	printf("wed\n");
	}
	else {
	printf("thu\n");
	}
	return 0;
}	
