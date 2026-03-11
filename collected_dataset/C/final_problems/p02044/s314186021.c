#include<stdio.h>

int solve(int n,int m){
	int L[105],i=0,ave,sum=0;
	for(;i<n;++i)
		scanf("%d",L+i);
	ave=m/n;
	--i;
	while(i>=0){
		if(L[i]>=ave) sum+=ave;
		else sum+=L[i];
		--i;}
	return sum;
}

int main(){
	while(1){
		int n,m;
		scanf("%d%d",&n,&m);
		if(n&&m) printf("%d\n",solve(n,m));
		else break;}
	return 0;
}
