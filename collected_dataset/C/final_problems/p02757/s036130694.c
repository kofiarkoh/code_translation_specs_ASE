#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#define rep(i,l,n) for(int i=l;i<n;i++)
#define ull unsigned long long
#define min(a,b) (a)<(b)?(a):(b)
#define max(a,b) (a)>(b)?(a):(b)

int main(){
	int N,P;
	scanf("%d %d",&N,&P);
	char t[N];
	int s[N];
	scanf("%s",t);
	rep(i,0,N) s[i]=t[i]-'0';
	int count=0;
	rep(i,0,N){
		int temp=0;
		rep(j,i,N){
			temp = (temp*10+s[j])%P;
			if(temp==0) count++;
		}
	}
	printf("%d",count);
	return 0;
}
