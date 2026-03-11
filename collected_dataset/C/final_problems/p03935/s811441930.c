#include<stdio.h>

int fibo(int n,int m){
	int i,j,ans[3000];
	for(i=1;i<=n;i++){
		for(j=0;j<m;j++){
		if(i==1){
				if(j<2){
					ans[j]=1;
				}else{
					ans[j]=(ans[j-2]+ans[j-1])%998244353;
				}
			}else{
			if(j>0){
				ans[j]=(ans[j-1]+ans[j])%998244353;
			}
		}
	}
	}
	return ans[m-1];
}

int main(){
	int n,m;
	scanf("%d %d",&n,&m);
	printf("%d\n",fibo(n,m));
	return 0;
}
