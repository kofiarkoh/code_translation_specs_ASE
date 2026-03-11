#include<stdio.h>
int main(){
    int i,n;
    scanf("%d",&n);
    int a[n+1],b[n+1],c[n+1];
    for(i=1;i<=n;i++)
        scanf("%d",&a[i]);
    int count=0;
    for(i=n;i>0;i--){
        int sum=a[i];
        for(int j=i*2;j<=n;j+=i)
            sum+=b[j];
        if(b[i]=sum%2)
            c[count++]=i;
    }
    printf("%d\n",count);
    while(count--)
        printf("%d ",c[count]);
    return 0;
}