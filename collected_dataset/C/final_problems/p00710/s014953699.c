#include<string.h>
#include<stdio.h>
int main(){
	int i,n,r;
	int p,c;
	int yama[50],a[50],*src,*dest;
	while(scanf("%d%d",&n,&r)==2&&(n|r)){
		for(i=0;i<n;i++)yama[i] = n-i;
		while(r--){
			scanf("%d%d",&p,&c);
			memcpy(a,yama+(p-1),c*sizeof(int));
			memmove(yama+c,yama,(p-1)*sizeof(int));
			memcpy(yama,a,c*sizeof(int));
		}
		printf("%d\n",*yama);
	}
}