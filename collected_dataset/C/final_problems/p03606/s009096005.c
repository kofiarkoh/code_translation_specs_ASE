#include <stdio.h>


int main(void){
	int n;
	scanf("%d",&n);
	
	int count=0;
	int l[n],r[n];
	for(int i=0;i<n;i++){
		scanf("%d %d",&l[i],&r[i]);
		count+=(r[i]-l[i]+1);
	}
	printf("%d", count);
	
	return 0;
} 