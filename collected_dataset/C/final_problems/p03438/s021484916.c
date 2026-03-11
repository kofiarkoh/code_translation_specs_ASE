#include <stdio.h>

int main()
{
    int N,a[10000],b[10000],i;
    int sum_a=0,sum_b=0,BA=0,AB=0,X,Y;

    scanf("%d",&N);

    for(i=0;i<N;i++){
        scanf("%d",&a[i]);
        sum_a += a[i];
    }
    for(i=0;i<N;i++){
        scanf("%d",&b[i]);
        sum_b += b[i];
        if(b[i]-a[i]>0){
            BA += b[i]-a[i];
        }else{
            AB += a[i]-b[i];
        }
    }

    X = sum_b-sum_a;

    if(sum_b < sum_a){
        printf("No\n");
    }else if((2*X - BA == X - AB)&&(X-AB>=0)){
        printf("Yes\n");
    }else{
        printf("No\n");
    }

    return 0;
}