#include <stdio.h>
int main(void){
    int n,m,sum=0,ans=0;
    scanf("%d %d",&n,&m);
    int a[n];
    for(int i=0; i<n; i++){
        scanf("%d",&a[i]);
        sum+=a[i];
    }
    for(int i=0; i<n; i++){
        if(a[i]>=sum/(4*m))
            ans++;
    }
    if(ans>=m)
        printf("Yes");
    else
        printf("No");
}
