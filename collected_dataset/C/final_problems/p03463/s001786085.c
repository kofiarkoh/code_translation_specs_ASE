#include <stdio.h>

int main(){
	int n,a,b;
	
	scanf("%d %d %d",&n,&a,&b);
	if((b-a)%2==1)printf("Borys");
	if((b-a)%2==0)printf("Alice");
	return 0;
}