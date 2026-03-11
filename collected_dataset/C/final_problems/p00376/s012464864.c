#include <stdio.h>

int main(void)
{
	int x1,x2;
	scanf("%d %d",&x1,&x2);
	if(x2>x1){
		printf("%d\n",x2-x1);
	}
	else if(x1>x2){
		printf("%d\n",x1-x2);
	}
	else{
		printf("%d\n",x1-x2);
	}
	return 0;
}
