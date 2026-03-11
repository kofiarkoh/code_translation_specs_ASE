#include <stdio.h>

int main(void){
    int n,m,i,j,p,c,max,min,sum;
    int a[100000];
    scanf("%d %d",&n,&m);
    max=0;min=0;
    for(i=0;i<n;i++){
        scanf("%d",&a[i]);
        max += a[i];
        if(min<a[i]){
            min=a[i];
        }
    }
    while(min<max){
        c=1;sum=a[0];
        p = (min+max)/2;
        for(j=1;j<n&&c<=m;j++){
            i = sum+a[j]-p;
            if(i>0){
                c++;
                sum = a[j];
            }else{
                sum += a[j];
            }
        }
        if(c>m){
            min=p+1;
        }else{
            max=p;
        }
    }
    printf("%d\n",max);
    return 0;
}

