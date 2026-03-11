#include <stdio.h>
#include <math.h>
#include <string.h>
#include <stdlib.h>

#define llong long long
#define fr(i,l,r) for(i=(l);i<(r);i++)
#define min(p,q) ((p)<(q)?(p):(q))
#define max(p,q) ((p)>(q)?(p):(q))
#define INF 1000000000000//10^12

int main(void)
{
	//変数の宣言
	int n;
	static llong a[200010];
	static llong ruiseki[200010];
	int p,q,r,s;
	int left,right;
	int m;
	
	//よく使う変数
	int i,j,k,l;
	int flag=0;
	llong int ans=0;
	int count=0;
	int temp,temp1,temp2;
	llong int max,min;
	int len;
	int sum=0;
	//データの読み込み
	
	scanf("%d",&n);
	
	for(i=0;i<n;i++){
		scanf("%lld",&a[i]);
	}
	
	
	
//	printf("nは%dです\n", n);
//	printf("データの読み込み終了\n");
	//実際の処理
	
	for(i=0;i<n;i++){
		if(i==0){
			ruiseki[0]=a[0];
		}else{
			ruiseki[i]=ruiseki[i-1]+a[i];
		}
	}
	
	ans=INF;
	for(i=1;i<n-2;i++){
		m=i;
		
		flag=0;
		left=0;right=m;
		while(flag==0){
			if(right-left<=1){
				if(r==m){
					p=ruiseki[left];
					q=ruiseki[m]-ruiseki[left];
				}else if(abs(ruiseki[m]-2*ruiseki[left])<abs(ruiseki[m]-2*ruiseki[right])){
					p=ruiseki[left];
					q=ruiseki[m]-ruiseki[left];
					flag=1;
				}else{
					p=ruiseki[right];
					q=ruiseki[m]-ruiseki[right];
					flag=1;
				}
			}else{
				temp=(left+right)/2;
				if(ruiseki[m]<2*ruiseki[temp]){
					right=temp;
				}else{
					left=temp;
				}
			}
		}
		
		flag=0;
		left=m+1;right=n-1;
		while(flag==0){
			if(right-left<=1){
				if(r==n-1){
					r=ruiseki[left]-ruiseki[m];
					s=ruiseki[n-1]-ruiseki[left];
				}else if(abs(ruiseki[n-1]+ruiseki[m]-2*ruiseki[left])<abs(ruiseki[n-1]+ruiseki[m]-2*ruiseki[right])){
					r=ruiseki[left]-ruiseki[m];
					s=ruiseki[n-1]-ruiseki[left];
					flag=1;
				}else{
					r=ruiseki[right]-ruiseki[m];
					s=ruiseki[n-1]-ruiseki[right];
					flag=1;
				}
			}else{
				temp=(left+right)/2;
				if(ruiseki[n-1]+ruiseki[m]<2*ruiseki[temp]){
					right=temp;
				}else{
					left=temp;
				}
			}
		}
		
		min=min(min(p,q),min(r,s));
		max=max(max(p,q),max(r,s));
		
		ans=min(ans,max-min);
	}
//	printf("計算部分終了\n");
	//出力
	
	printf("%lld",ans);
	
//	printf("結果の出力終了\n");
	
	return 0;
	
}

