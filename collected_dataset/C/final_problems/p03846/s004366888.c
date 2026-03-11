#include<stdio.h>

int main(){
    int n;
    scanf("%d",&n);
    int a[n];
    for(int i=0;i<n;i++){
        scanf("%d",&a[i]);
    }

    int b[n];
    for(int i=0;i<n;i++){
        b[i]=0;
    }
    for(int i=0;i<n;i++){
        for(int j=0;j<n;j++){
            if(a[i]==j){
                b[j]++;
            }
        }
    }

    int sum=1;
    if(n%2==0){
        if(b[0]>0){
            sum=0;
        }
        for(int i=1;i<n;i++){
            if(i%2==0&&b[i]>0){
                sum=0;
                break;
            }
            if(i%2==1&&b[i]!=2){
                sum=0;
                break;
            }
            if(i%2==1){
                sum*=2;
            }
        }
    }else{
        if(b[0]%2==0){
            sum=0;
        }
        for(int i=1;i<n;i++){
            if(i%2==1&&b[i]>0){
                sum=0;
                break;
            }
            if(i%2==0&&b[i]!=2){
                sum=0;
                break;
            }
            if(i%2==0){
                sum*=2;
            }
        }
    }
    sum%=(1000000007);
    printf("%d\n",sum);
    return 0;
}