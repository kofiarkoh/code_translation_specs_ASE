#include<stdio.h>
int a[300000],b[300000];
int main(){
	long long n;
	int q,h,w,t;
	scanf("%lld %d",&n,&q);
	h=n;w=n;
	long long ans=(n-2)*(n-2);
	for (int i=1;i<=n;i++){
		a[i]=n;b[i]=n;
	}
	for (int i=1;i<=q;i++){
		int k;
		scanf("%d",&k);
		if (k==1){
			scanf("%d",&t);
			if (t<w){
				for (int j=t;j<w;j++){
				  b[j]=h;
			}
				w=t;
			}
			ans=ans-(b[t]-2);
		}else{
			scanf("%d",&t);
			if (t<h){
				for (int j=t;j<h;j++){
				  a[j]=w;
			}
			    h=t;
			}
			ans=ans-(a[t]-2);	
		}
	}
	printf("%lld",ans);
}