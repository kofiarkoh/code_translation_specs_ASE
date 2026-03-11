#include <stdio.h>

int max(int a, int b){
	if(a>b){
		return a;
	}
	return b;
}

int min(int a, int b){
	if(a>b){
		return b;
	}
	return a;
}

int main(){
	int H;
	scanf("%d",&H);
	int W;
	scanf("%d",&W);
	char c;
	char d;
	char l[186][186];
	int i;
	int j;
	for(i=1;i<=H;i++){
		scanf("%c",&d);
		for(j=1;j<=W;j++){
			scanf("%c",&c);
			l[i][j]=c;
		}
	}
	int m;
	m=0;
	int n;
	n=0;
	for(i=1;i<=H;i++){
		int a;
		a=0;
		for(j=1;j<W;j++){
			if(l[i][j]!=l[i][j+1]){
				a=a+1;
			}
		}
		m=max(a,m);
	}
	for(j=1;j<=W;j++){
		int b;
		b=0;
		for(i=1;i<H;i++){
			if(l[i][j]!=l[i+1][j]){
				b=b+1;
			}
		}
		n=max(b,n);
	}
	int ans;
	ans=0;
	while(n>0){
		ans=ans+1;
		n=n/2;
	}
	while(m>0){
		ans=ans+1;
		m=m/2;
	}
	printf("%d",ans);
}