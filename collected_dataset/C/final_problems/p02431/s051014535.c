#include <stdio.h>
#define N 1000000
int main(void){
    int a[N],n,q,query,x,p,back;
    scanf("%d",&q);
    back=0;
    for(int i=0;i<q;i++){
        scanf("%d",&query);
        if(query==2) {
            a[back]=0;
            back--;
        }
        else if(query==1){
            scanf("%d",&p);
            if(a[p]!=0) printf("%d\n",a[p]);
        }
        else if(query==0){
            scanf("%d",&x);
            if(i!=0) back++;
            a[back]=x;
        }
    }
    return 0;
}

