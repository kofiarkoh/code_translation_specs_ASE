#include<stdio.h>
#include<stdlib.h>
#include<math.h>
#include<string.h>
#include <time.h>
typedef long long ll;
typedef long double ld;
#define INF (1LL<<60)
#define MOD1 1000000007
/*swap 交換*/
void swap(ll *a, ll *b){ll c;c=*b;*b=*a;*a=c;}
/*2つのうち大きい数を返す*/
ll max2(ll a,ll b){return a>=b?a:b;}
/*2つのうち小さい数を返す*/
ll min2(ll a,ll b){return a>=b?b:a;}
/*絶対値*/
ll ABS(ll a){return a>=0?a:(-a);}
typedef struct{
    ll aa;
    ll bb;
}frequent;
// 小大
int compare(const void *a, const void *b){return *(ll *)a - *(ll *)b;}

int main(void){
    ll n,i,j,l,alpha[26],mins[26];
    scanf("%lld",&n);
    char s[55];
    for(i=0;i<26;i++){
        alpha[i]=0;
        mins[i]=INF;
    }
    for(i=0;i<n;i++){
        scanf("%s",s);
        l=strlen(s);
        for(j=0;j<26;j++){
            alpha[j]=0;
        }
        for(j=0;j<l;j++){
            alpha[(ll)s[j]-97]++;
            // alpha[(ll)s[j]-97][1]=s[j];
        }
        for(j=0;j<26;j++){
            mins[j]=min2(mins[j],alpha[j]);
        }
    }
    for(i=0;i<26;i++){
        for(j=0;j<mins[i];j++){
            printf("%c",(char)i+97 );
        }
    }
    printf("\n");
    return 0;
}
