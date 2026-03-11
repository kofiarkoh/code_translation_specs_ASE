#include <stdio.h>
int main(){
	int a;
	scanf("%d",&a);
	int b;
	scanf("%d",&b);
	b -=a;
	if(b<=0){
		printf("1");
		return 0;
		}
		int count = 1;
	while(1){
		if(b>0){
		   b-=(a - 1);
		   count++;
		   }
		if(b<=0)break;
		 }
		 printf("%d",count);
     return 0;
}