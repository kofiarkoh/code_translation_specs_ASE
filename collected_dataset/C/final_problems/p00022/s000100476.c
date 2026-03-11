#include <stdio.h>
#include <string.h>
const int INT_MIN=-1e9;
int a[5000];
int left[5010],right[5010];
int main(){
	int n,i;
	while(scanf("%d",&n),n){
		memset(a,0,sizeof(a));
		memset(left,0,sizeof(left));
		memset(right,0,sizeof(right));
		for(i=0;i<n;i++){
			scanf("%d",&a[i]);
		}
		left[0]=0;
		for(i=1;i<n;i++){
			left[i]=left[i-1]+a[i-1]>0?left[i-1]+a[i-1]:0;
		}
		right[n-1]=0;
		for(i=n-2;i>=0;i--){
			right[i]=right[i+1]+a[i+1]>0?right[i+1]+a[i+1]:0;
		}
		int ans=INT_MIN;
		for(i=0;i<n;i++){
			ans=ans>left[i]+a[i]+right[i]?ans:left[i]+a[i]+right[i];
		}
		printf("%d\n",ans);
	}
	return 0;
}