#include<stdio.h>
int main()
{
	int x,r;
	scanf("%d",&x);
	r=x%7;
	switch(r){
	case 1:
		printf("fri\n");
		break;
	case 2:
		printf("sat\n");
		break;
	case 3:
		printf("sun\n");
		break;
	case 4:
		printf("mon\n");
		break;
	case 5:
		printf("tue\n");
		break;
	case 6:
		printf("wed\n");
		break;
	default:
		printf("thu\n");
	}
    return 0;
}