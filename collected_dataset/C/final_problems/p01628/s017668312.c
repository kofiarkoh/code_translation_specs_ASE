#include <stdio.h>
int main(void){
    int i,n,m,a,l[8];
    int max;
    scanf("%d %d",&n,&m);
    for(i=0;i<n;i++){
        l[i]=0;
    }
    for(i=0;i<m;i++){
        scanf("%d",&a);
        if(l[a-1]<l[a]){
            l[a-1]=l[a];
        }else if(l[a-1]>l[a]){
            l[a]=l[a-1];
        }
        l[a-1]+=1;
        l[a]+=1;
    }
    max=l[0];
    for(i=1;i<n;i++){
        if(max<l[i]){
            max=l[i];
        }
    }
    printf("%d\n",max);
    return 0;
}