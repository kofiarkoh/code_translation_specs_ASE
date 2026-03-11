#include<stdio.h>
#include<stdlib.h>
#define ll long long
int cmp(const void*a,const void*b){return*(long long*)a-*(long long*)b;}
int main(){
    int n;
    scanf("%d",&n);
    ll a[n], b[n],c[n];
    for(int i = 0; i < n; i++){
        scanf("%lld",&a[i]);
        b[i] = a[i];
        c[i] = 0;
    }
    ll ans = 0;
    int start = 0, end = n - 1;
    qsort(b,n,sizeof(long long),cmp);
    for(int i = 0; i < n; i++){
        for(int j = 0; j < n; j++){
            ll p = b[n - i - 1];
            if(a[j] == p){
                int d1 = j - start;
                int d2 = end - j;
                if(d1 < d2){
                    ans += d2 * a[j];
                    end --;
                }
                else{
                    ans += d1 * a[j];
                    start ++;
                }
            }
        }
    }
    printf("%lld\n",ans);
    return 0;
}