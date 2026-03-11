#include <stdio.h>
#define min(p,q)((p)<(q)?(p):(q))

int a[100010];
int cnt[100010];
int main(){
	int n,k;
	scanf("%d%d",&n,&k);
	for(int i=0;i<n;i++)scanf("%d",a+i);
	int ans=1e6;
	int r=0,kind=0;
	for(int l=0;l<n;l++){
		while(r<n&&kind<k){
			if(a[r]<=k)kind+=!cnt[a[r]]++;
			r++;
		}
		if(kind==k)ans=min(ans,r-l);
		if(a[l]<=k)kind-=!--cnt[a[l]];
	}
	printf("%d\n",ans==1e6?0:ans);
}

