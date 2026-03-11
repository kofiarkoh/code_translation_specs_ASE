#include<stdio.h>

int main(){
	unsigned int a,b;
	int ad,or,xr,x[32] = {0},y[32] = {0},i;
	
	scanf("%d %d",&a,&b);
	
	
	for(i = 0; a>0; i++){
		x[i] = a % 2;
		a = a / 2;
	}
	
	for(i = 0; b>0; i++){
		y[i] = b % 2;
		b = b / 2;
	}
	
	for(i=31;i>=0;i--){
		if(x[i]==1&&y[i]==1)printf("1");
		else printf("0");
	}
	printf("\n");
	
	
	for(i=31;i>=0;i--){
		if(x[i]==1||y[i]==1)printf("1");
		else printf("0");
	}
	printf("\n");
	
	
	for(i=31;i>=0;i--){
		if(x[i]==1&&y[i]==0||x[i]==0&&y[i]==1)printf("1");
		else printf("0");
	}
	printf("\n");
	
    return 0;
}

