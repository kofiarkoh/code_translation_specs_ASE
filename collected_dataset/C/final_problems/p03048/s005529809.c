#include <stdio.h>
int main(){
    int R,G,B,n,ans=0;
    scanf("%d%d%d%d",&R,&G,&B,&n);
    for(int i=0;i<=n/R;i++){
        for(int j=0;j<=n/G;j++){
            if(R*i+G*j<=n){
                for(int k=0;k<=n/B;k++){
                    if(n==R*i+G*j+B*k)
                        ans++;
                }
            }
        }
    }
    printf("%d",ans);
    return 0;
}
