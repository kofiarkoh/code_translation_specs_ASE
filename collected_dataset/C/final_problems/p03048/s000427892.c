#include <stdio.h>
int main(void){
    int r, g, b, n;
    scanf("%d %d %d %d", &r, &g, &b, &n);
    
    long long ans=0;
    for(int i=0; i<=n/r; i++){
        for(int j=0; j<=n/g; j++){
            if(n-i*r-j*g>=0 && (n-i*r-j*g)%b==0)ans++;
        }
    }
    printf("%lld\n", ans);
    return 0;
}
