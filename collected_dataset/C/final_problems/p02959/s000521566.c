#include<stdio.h>

int main(){
    int n;
    int a[100000+5], b[100000+5];
    int cnt=0;

    scanf("%d", &n);
    for(int i=0; i<n+1; i++){
        scanf("%d", &a[i]);
    }

    for(int i=0; i<n; i++){
        scanf("%d", &b[i]);
    }
    for(int i=0; i<n; i++){
        if(a[i]-b[i]>=0){
            cnt+=b[i];
            a[i]-=b[i];
            b[i]=0;
        }else{
            cnt+=a[i];
            b[i]-=a[i];
            a[i]=0;
            if(a[i+1]-b[i]>=0){
                cnt+=b[i];
                a[i+1]-=b[i];
                b[i]=0;
            }else{
                cnt+=a[i+1];
                b[i]-=a[i+1];
                a[i+1]=0;
            }
        }
    }
    printf("%d\n", cnt);
    return 0;
}