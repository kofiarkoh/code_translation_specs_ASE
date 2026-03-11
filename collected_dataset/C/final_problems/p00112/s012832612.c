#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int i,j,k;
int n,tim[61];
long long int ans;

int main(){

	while(scanf("%d",&n)*n){
		ans=0;
		for(i=0;i<61;i++)tim[i]=0;
		for(i=0;i<n;i++){
			scanf("%d",&k);
			tim[k]++;
		}
		for(i=1;i<61 && n>0;i++){
			for(;tim[i]>0 && n>0;tim[i]--){
				n--;
				ans+=i*n;
			}
		}
		printf("%d\n",ans);
	}
	return 0;
}