#include<stdio.h>
int N,M,T;
int main(void){
    int a[101]={0};
    scanf("%d%d%d",&N,&M,&T);
    for(int i=0;i<N;i++){
        scanf("%d",&a[i]);
    }
    int sum=0;
    sum+=a[0]-M;
    for(int i=1;i<N;i++){
        if(a[i]-a[i-1]>2*M){
            sum+=a[i]-a[i-1]-2*M;
        }
    }
    if((T-a[N-1])>M)
        printf("%d\n",sum+T-a[N-1]-M);
    else
        printf("%d\n",sum);
}

