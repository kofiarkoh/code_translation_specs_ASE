#include <stdio.h>

int main(){
	int n,k;
	scanf("%d%d",&n,&k);
	int x=0;
	for(int i=0;i<n;i++){
		int t;
		scanf("%d",&t);
		x^=--t%(k+1);
	}
	puts(x?"First":"Second");
}
