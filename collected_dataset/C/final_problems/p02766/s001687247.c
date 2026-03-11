#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void){
	int n,k;
	int i;
	
	scanf("%d %d",&n,&k);
	for(i=0;n!=0;i++){
		n=n/k;
	}
	printf("%d",i);
	return 0;
}
