#include<stdio.h>
#include<stdlib.h>
#include<math.h>
#include<string.h>
#include <time.h>
typedef long long llint;
typedef long double ldouble;
// 小大
int compare(const void *a, const void *b){return *(llint *)a - *(llint *)b;}
typedef struct{
    llint aa;
    llint bb;
}frequent;
// 小大
int cmp( const void *p, const void *q ) {
    return ((frequent*)p)->bb - ((frequent*)q)->bb;
}
/*[a]+1 ガウス+1*/
llint pgauss(ldouble a){llint b=a;return a==b?b:pgauss(b+1);}
/*2つのうち大きい数を返す*/
llint max2(llint a,llint b){return a>=b?a:b;}

int main(void){
    llint n,h,w,a;
    scanf("%lld%lld%lld",&h,&w,&n);
    a = n%max2(h,w)==0?n/max2(h,w):1+n/max2(h,w);
    printf("%lld\n",a);
    return 0;
}
