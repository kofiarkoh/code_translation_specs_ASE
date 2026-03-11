#include <stdio.h>
#include <stdlib.h>

int s[100010],t[100010],idx[100010];
int c(const void*p,const void*q){
	if(t[*(int*)p]<t[*(int*)q])return -1;
	return 1;
}
int main(){
	int n;
	scanf("%d",&n);
	for(int i=0;i<n;i++)scanf("%d%d",s+i,t+i);
	for(int i=0;i<n;i++)idx[i]=i;
	qsort(idx,n,sizeof(int),c);
	int ans=0,time=0;
	for(int i=0;i<n;i++){
		if(time<s[idx[i]]){
			time=t[idx[i]];
			ans++;
		}
	}
	printf("%d\n",ans);
}
