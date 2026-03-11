#include <stdio.h>

void sort(long long int* n, int len){
	long long int tmp;
	for(int i=0; i<len; i++){
		for(int j=len-1; j>i; j--){
			if(n[j] > n[j-1]){
				tmp=n[j];
				n[j]=n[j-1];
				n[j-1]=tmp;
			}
		}
	}
}

int main(void){
	int x,y,a,b,c;
	scanf("%d %d %d %d %d",&x,&y,&a,&b,&c);
	long long int p[a];
	long long int q[b];
	long long int r[c];
	long long int ans[x+y+c];
	long long int d=0;
	for(int i=0; i<a; i++){
		scanf("%lld",&p[i]);
	}
	for(int i=0; i<b; i++){
		scanf("%lld",&q[i]);
	}
	for(int i=0; i<c; i++){
		scanf("%lld",&r[i]);
	}
	sort(p,a);
	sort(q,b);
	for(int i=0; i<x; i++){
		ans[i]=p[i];
	}
	for(int i=0; i<y; i++){
		ans[i+x]=q[i];
	}
	for(int i=0; i<c; i++){
		ans[i+x+y]=r[i];
	}
	sort(ans,x+y+x);
	for(int i=0; i<x+y; i++){
		d+=ans[i];
	}
	printf("%lld\n",d);

	return 0;

}

