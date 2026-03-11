#include<stdio.h>
#include<math.h>
int main(){
    int k=0,count=0;
    long long S;
    scanf("%ld",&S);
    while(pow(10,k)<=S){
        k++;
    }
    for(int i=1;i<k;i++){
        for(int j=i+3;j<=k;j++){
            long long s=S;
            long long sj=S/pow(10,j);
            s-=sj*pow(10,j);
            s/=pow(10,i-1);
            if(s%2019==0){
                count++;
            }
        }
    }
    printf("%d",count);
    return 0;
}