#include<stdio.h>
#define MAX 5000
int main(){
	int n;
	int a[MAX];
	int s[MAX];
	int i,j,k;
	int maxs,maxb=0;
	while(1){
		scanf("%d",&n);
		if(n==0){
			break;
		}
		for(i=0;i<n;i++){
			a[i]=0;s[i]=0;
		}
		maxs=-2000000000;
		for(i=0;i<n;i++){
			scanf("%d",&a[i]);
			for(j=i;j<n;j++){
				s[j]+=a[i];
			}
		}
		for(i=0;i<n;i++){
			if(maxs<s[i]){
				maxs=s[i];
				k = i;
			}
		}
		maxb = maxs;
		for(i=0;i<k;i++){
			if(maxs<maxb-s[i]){
				maxs=maxb-s[i];
			}
		}
		printf("%d\n",maxs);
	}
	return 0;
}