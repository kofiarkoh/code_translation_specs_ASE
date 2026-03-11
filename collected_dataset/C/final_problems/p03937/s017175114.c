#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>
#define Rep(i,a,n)for(int i=(int)(a);i<(int)(n);i++)
#define rep(i,n)Rep(i,0,n)
typedef long long LL;
typedef long double Lf;
#define Ma(x,y) ((x)>(y)?(x):(y))
#define mi(x,y) ((x)<(y)?(x):(y))

int main() {//→or↓のみ と 最短経路 は 同値
	int h,w,sh=0;char c[9];
	scanf("%d%d",&h,&w);
	rep(i,h){
		scanf("%s",c);
		rep(j,w)if(c[j]=='#')sh++;
	}
	printf(sh==h+w-1?"Possible":"Impossible");
}