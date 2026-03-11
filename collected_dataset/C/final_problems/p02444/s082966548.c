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
    int i,j;
    
    int n;
    scanf("%d\n",&n);
    
    int a[n],tmp[n];
    for(i=0;i<n;i++){
        scanf("%d \n",&a[i]);
        tmp[i]=a[i];
    }
    
    int q;
    scanf("%d\n",&q);
    
    int b,m,e;
    int k;
    while(q--){
        scanf("%d %d %d\n",&b,&m,&e);
        for(k=0;k<e-b;k++){
            tmp[b+((k+(e-m))%(e-b))]=a[b+k];
        }
        for(i=0;i<n;i++)a[i]=tmp[i];
    }
    
    output(a,n);
    
    return 0;
}

