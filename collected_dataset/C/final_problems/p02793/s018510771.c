#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#include<math.h>
#define max(x,y)(x>y?x:y)
#define min(x,y)(x>y?y:x)
typedef long long int ll;
typedef unsigned long long int ull;
int asc_cmp(const void* X,const void* Y){int x=*(int*)X,y=*(int*)Y;if(x<y)return -1;else if(x==y)return 0;else return 1;}
int desc_cmp(const void* X,const void* Y){int x=*(int*)X,y=*(int*)Y;if(x<y)return 1;else if(x==y)return 0;else return -1;}
ll gcd(ll x,ll y){if(!y)return x;return gcd(y,x%y);}
ll lcm(ll x,ll y){return x/gcd(x,y)*y;}
int main(){
    int n;scanf("%d",&n);
    int a[2000];for(int i=0;i<n;i++)scanf("%d",&a[i]);
    int ac[2000];for(int i=0;i<n;i++)ac[i]=a[i];
    ll LCM=a[0];
    for(int i=0;i<n;i++){
        for(int j=i+1;j<n;j++){
            LCM=lcm(LCM,a[j]);
        }
    }

    ll ans=0;
    for(int i=0;i<n;i++){
        ans+=LCM/a[i];
        ans%=1000000007;
    }
    printf("%lld\n",ans);
}