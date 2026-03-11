#include <stdio.h>
int main(void){
    int h, w, m;
    int x[300000+5], y[300000+5]={0};
    
    scanf("%d %d %d", &h, &w, &m);
    for(int i=0; i<m; i++){
        scanf("%d %d", &y[i], &x[i]);
    }
    long long int ans=0;
    for(int i=0; i<m; i++){
        for(int j=0; j<m; j++){
            long long n=0;
            for(int k=0; k<m; k++){
                if(y[i]==y[k] || x[j]==x[k])n++;
            }
            if(n>ans)ans=n;
        }
    }
    printf("%lld\n", ans);
    return 0;
}
