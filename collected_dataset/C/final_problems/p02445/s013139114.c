#include <stdio.h>

void output(int a[],int n){
    int i;
    for(i=0;i<n;i++){
        if(i==0)printf("%d",a[i]);
        else printf(" %d",a[i]);
    }
    printf("\n");
    return;    
}

int main(void){
    int i;
    
    int n;
    scanf("%d\n",&n);
    
    int a[n];
    for(i=0;i<n;i++){
        scanf("%d \n",&a[i]);
    }
    
    int q;
    scanf("%d\n",&q);
    
    int b,e,t;
    int k,tmp;
    while(q--){
        scanf("%d %d %d\n",&b,&e,&t);
        for(k=0;k<e-b;k++){
            tmp=a[b+k];
            a[b+k]=a[t+k];
            a[t+k]=tmp;
        }
    }
    
    output(a,n);
    
    return 0;
}

