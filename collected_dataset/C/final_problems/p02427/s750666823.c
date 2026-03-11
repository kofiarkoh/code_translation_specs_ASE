#include <stdio.h>
#define bit(n,m)(((n)>>(m))&1)

int main(){
	int n;
	scanf("%d",&n);
	for(int x=0;x<1<<n;x++){
		printf("%d:",x);
		for(int k=0;k<n;k++)if(bit(x,k))printf(" %d",k);
		puts("");
	}
}
