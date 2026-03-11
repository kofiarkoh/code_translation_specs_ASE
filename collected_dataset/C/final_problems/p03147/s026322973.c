#include <stdio.h>
int main(void){
    int N,l,r;
    int k=0;
    int h[100]={};
    scanf("%d",&N);
    for(int i=0;i<N;i++){
        scanf("%d",&h[i]);
    }
    while (1) {
        for(l=0;h[l]==0;l++){
            if(l==N){
                printf("%d\n",k);
                return 0;
            }
        }
        for(r=l;h[r]!=0;r++){
            h[r]--;
        }
        k++;
    }
}
