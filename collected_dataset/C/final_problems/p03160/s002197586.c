
#include<stdlib.h>
#include<stdio.h>
int main(){
	int n,*h,*ct;
	scanf("%d",&n);
	h=(int*)malloc(n*sizeof(int));
	for (int i = 0; i < n; ++i) scanf("%d",&h[i]);
	ct=(int*)malloc(n*sizeof(int));
	ct[0]=0;ct[1]=abs(h[1]-h[0]);
	for (int i = 2; i < n; ++i){
		if((ct[i-2]+abs(h[i]-h[i-2]))>(ct[i-1]+abs(h[i]-h[i-1]))) ct[i]=ct[i-1]+abs(h[i]-h[i-1]);
		else ct[i]=ct[i-2]+abs(h[i]-h[i-2]);
	}
	printf("%d",ct[n-1]);
	return 0;
}