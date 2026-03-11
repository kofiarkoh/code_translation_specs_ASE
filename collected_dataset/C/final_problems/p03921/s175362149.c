#include<stdio.h>

long n,m,par[200000];

long find(long x){
	if(x!=par[x]) par[x]=find(par[x]);
	return par[x];
}

int main(){
	long i,x,y;
	scanf("%ld %ld",&n,&m);
	for(i=0;i<n+m;i++){
		par[i]=i;
	}
	for(i=0;i<n;i++){
		scanf("%ld",&x);
		while(x){
			scanf("%ld",&y);
			y=y-1+n;
			par[find(i)]=find(y);
			x--;
		}
	}
	for(i=0;i<n;i++){
		if(find(i)!=find(0)){
			printf("NO\n");
			return 0;
		}
	}
	printf("YES\n");
	return 0;
}
