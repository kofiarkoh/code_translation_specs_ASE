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
    
    int a[n],x;
    scanf("%d",&a[0]);
    for(i=1,j=1;i<n;i++){
        scanf("%d",&x);
        if(a[j-1]!=x)a[j++]=x;
    }
    
    output(a,j);
    
    return 0;
}

