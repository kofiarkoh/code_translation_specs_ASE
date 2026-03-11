#include <stdio.h>

int main(void){
        int mem[4][3][10]={0};
        int b,f,r,v,n,i,j,k;

        scanf("%d",&n);

        for(i=0;i<n;i++){
                scanf("%d %d %d %d",&b,&f,&r,&v);
                mem[b-1][f-1][r-1]+=v;
        }
        for(i=0;i<4;i++){
                for(j=0;j<3;j++){
                        for(k=0;k<10;k++)
                                printf(" %d",mem[i][j][k]);
                        putchar('\n');
                }
                if(i<3)
                        puts("####################");
        }
        return 0;
}