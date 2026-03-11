#include <stdio.h>
int main(){
	int a;
	scanf("%d",&a);
	if(a<1200){
		printf("ABC");
	}else if(a>=1200){
		printf("ARC");
	}
	return 0;
}