#include <stdio.h>
#include <stdlib.h>

int x[100010],y[100010];
int c(const void*p,const void*q){
	if(x[*(int*)p]<x[*(int*)q])return -1;
	if(x[*(int*)p]>x[*(int*)q])return 1;
	if(y[*(int*)p]<y[*(int*)q])return -1;
	return 1;
}
int idx[100010];
int main(){
	int n;
	scanf("%d",&n);
	for(int i=0;i<n;i++)scanf("%d%d",x+i,y+i);
	for(int i=0;i<n;i++)idx[i]=i;
	qsort(idx,n,sizeof(int),c);
	for(int i=0;i<n;i++)printf("%d %d\n",x[idx[i]],y[idx[i]]);
}
