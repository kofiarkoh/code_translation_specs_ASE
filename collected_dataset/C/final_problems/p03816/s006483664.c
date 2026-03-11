#include<stdio.h>
int n,now;
int a[100009],vis[100009],sum,ans;
int main(){
	scanf("%d",&n);

	for(int i=1;i<=n;i++){
		scanf("%d",&now);

		if(a[now]==0){
			a[now]=1;
			ans++;
		}
		else{
			if(vis[now]<3){
				vis[now]++;
				sum++;
			}
			else{
				vis[a[i]]=1;
				sum--;
			}
		}
	}
	printf("%d",ans-(sum%2));
}