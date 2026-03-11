#include<stdio.h>
#include<stdlib.h>
int compare_int(const void *a, const void *b)
{
    return *(int*)a - *(int*)b;
}

int main(){
	int n,m,i,x[100000];
	long long ans=0;
	scanf("%d%d",&n,&m);
	for(i=0;i<m;i++)scanf("%d",&x[i]);
	if(n>=m){
		puts("0");
		return 0;
	}
	int sa[100000]={0};
	qsort(x,m,sizeof(int),compare_int);
	for(i=0;i<m-1;i++)sa[i]=abs(abs(x[i])-abs(x[i+1]));
	qsort(sa,m-1,sizeof(int),compare_int);
	for(i=0;i<m-n;i++)ans+=sa[i];
	printf("%lld\n",ans);
	return 0;
}
