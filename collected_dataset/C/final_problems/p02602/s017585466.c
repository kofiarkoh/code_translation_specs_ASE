
#include <stdio.h>
#include <stdlib.h>

int main(void) {
	int n,k;
	scanf("%d%d",&n,&k);
	int a[n];
	int hyouten[n];
	for(int i=0;i<n;i++){
		scanf("%d",&a[i]);
		hyouten[i]=1;
	}
	for(int i=k-1;i<n;i++){
		for(int p=i;p>i-k;p--){
			hyouten[i] *= a[p];
		}
	}
	for(int i=k;i<n;i++){
		if(hyouten[i-1]<hyouten[i])printf("Yes\n");
		else printf("No\n");
	}
}
