#include <stdio.h>
void swap(int *a,int *b){
	int t=*a;
	*a=*b;
	*b=t;
}
int main(void){
	int a,b;
	double ans;
	scanf("%d %d",&a,&b);
	if(a>b){
		swap(&a,&b);
	}
	if(b-a>180){
		b-=360;
	}
	ans=(a+b)*1.0/2;
	printf("%.1f\n",ans);
	return 0;
}