#include <stdio.h>

int main(){
    char dat[10002];
    int kdc[10]={0,5,6,6,6,6,6,8,6,8};
    char kd[10][10]={"","',.!?","abcABC","defDEF","ghiGHI","jklJKL","mnoMNO","pqrsPQRS","tuvTUV","wxyzWXYZ"};
    int i,t=-1,tc=0,j;
    while (scanf("%s",dat) != EOF){
        for (i=0;dat[i]!=0;i++){
            if (dat[i]-'0' == t){
                tc++;
            }else{
                if (tc>0){
                    if (t==0){
                        if (tc>1){
                            for (j=0;j<tc-1;j++) printf(" ");
                        }
                    }else{
                        while (tc>kdc[t])tc -= kdc[t];
                        printf("%c",kd[t][tc-1]);
                    }
                }
                t=dat[i]-'0';
                tc=1;
            }
            
        }
        if (tc>0){
            if (t==0){
                if (tc>1){
                    for (j=0;j<tc-1;j++) printf(" ");
                }
            }else{
                while (tc>kdc[t])tc -= kdc[t];
                printf("%c",kd[t][tc-1]);
            }
        }
        t=-1;
        tc=0;
        
        printf("\n");
    }
    return 0;
}