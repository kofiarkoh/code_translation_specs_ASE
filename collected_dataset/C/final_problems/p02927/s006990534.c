
#include <stdio.h>

int main()
{   int m,d,ans=0;
    scanf("%d%d",&m,&d);
    for(int i=1;i<=m;i++){
        for(int j=11;j<=d;j++){
            int d10=j/10,d1=j%10;
            if(d1>=2&&d10>=2&&d10*d1==i){
                ans++;
                //printf("%d/%d%d\n",i,d10,d1);
            }
        }
    }
    printf("%d",ans);

    return 0;
}
